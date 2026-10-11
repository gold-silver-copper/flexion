use flexion::Path;

/// Words ending -рат.
pub(crate) fn n3700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3689);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16328);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39313);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 39314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 39315);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 67354);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93130);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 112593);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 125207);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134192(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39316);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67355);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93131);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 112594);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125208);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39317);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67356);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 67357);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 39319);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39320);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 39322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 16329);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39323);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 39324);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67361);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39325);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67362);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39326);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67363);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93135);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112595);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 67364);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93136);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112596);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125209);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67365);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 67366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16330);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 39328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16331);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39329);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39330);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67368);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93137);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112597);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16332);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39331);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 67369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 39332);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67370);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93138);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112598);
											s.is_empty()
										}
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
					super::push(out, 39333);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39334);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67372);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93139);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112599);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 67373);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93140);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112600);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16333);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39335);
					s.is_empty()
				}
				['д', s @ ..] => super::t38::n39712(s, out),
				['л', s @ ..] => super::t38::n39713(s, out),
				['м', s @ ..] => super::t38::n39714(s, out),
				['н', s @ ..] => super::t38::n39715(s, out),
				['ф', s @ ..] => super::t38::n39716(s, out),
				['ц', s @ ..] => super::t38::n39717(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t38::n16429(s, out),
		['и', s @ ..] => super::t38::n16430(s, out),
		['й', s @ ..] => super::t38::n16431(s, out),
		['к', s @ ..] => super::t38::n16432(s, out),
		['м', s @ ..] => super::t38::n16433(s, out),
		['о', s @ ..] => super::t38::n16434(s, out),
		['р', s @ ..] => super::t38::n16435(s, out),
		['с', s @ ..] => super::t38::n16436(s, out),
		['т', s @ ..] => super::t38::n16437(s, out),
		['у', s @ ..] => super::t38::n16438(s, out),
		['ф', s @ ..] => super::t38::n16439(s, out),
		_ => false,
	}
}

/// Words ending -сат.
pub(crate) fn n3701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3690);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16345);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 39375);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16346);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39376);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67430);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93173);
									s.is_empty()
								}
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
			super::push(out, 16347);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39377);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 39378);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67432);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 93174);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112618);
											s.is_empty()
										}
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
					super::push(out, 39379);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 39380);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 39381);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 16348);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39382);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67436);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93175);
									s.is_empty()
								}
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
			super::push(out, 16349);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39383);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67437);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93176);
									s.is_empty()
								}
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
			super::push(out, 16350);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39384);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67438);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93177);
									s.is_empty()
								}
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
			super::push(out, 16351);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39385);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 67439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16352);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39386);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67440);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93178);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112619);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16353);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 39387);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67441);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93179);
									s.is_empty()
								}
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
			super::push(out, 16354);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39388);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67442);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 67443);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39389);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67444);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93181);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112620);
											s.is_empty()
										}
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
					super::push(out, 39390);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67445);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93182);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112621);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16355);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39391);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 67446);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 39392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 16356);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39393);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тат.
pub(crate) fn n3702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3691);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16357);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16358);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 39394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16359);
			match s {
				[] => true,
				['м', 'о', s @ ..] => {
					super::push(out, 39395);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 39396);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67448);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93184);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112622);
											s.is_empty()
										}
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
					super::push(out, 39397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16360);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16361);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39398);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67449);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 67450);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39399);
					match s {
						[] => true,
						['т', 'о', s @ ..] => {
							super::push(out, 67451);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93187);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112623);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16362);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16363);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16364);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39400);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67452);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39401);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67453);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 67454);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93189);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112624);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 93190);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112625);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125219);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 93191);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112626);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 67455);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 67456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16365);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39402);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67457);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 39403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', 'р', s @ ..] => {
			super::push(out, 16366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уат.
pub(crate) fn n3703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3692);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16367);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39404);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67459);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 93193);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112627);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фат.
pub(crate) fn n3704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3693);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16368);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16369);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39405);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16370);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67461);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 67462);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93194);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112628);
											s.is_empty()
										}
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
					super::push(out, 39406);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67463);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112629);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16371);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 39407);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 67464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93196);
									s.is_empty()
								}
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
			super::push(out, 16372);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39408);
					match s {
						[] => true,
						['ф', 'и', s @ ..] => {
							super::push(out, 67465);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93197);
									s.is_empty()
								}
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
			super::push(out, 16373);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39409);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хат.
pub(crate) fn n3705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3694);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16374);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39410);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16375);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39411);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67468);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 93198);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 93199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 67469);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93200);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112630);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16376);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39412);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39413);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67471);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93201);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 112631);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16377);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16378);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67472);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 67473);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93202);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 93203);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112632);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125220);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 93204);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112633);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125221);
													match s {
														[] => true,
														['п', s @ ..] => super::t38::n134198(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67474);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 67475);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 67476);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93207);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112634);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125222);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16379);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чат.
pub(crate) fn n3706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3695);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 16380);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16381);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39416);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 67478);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93208);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112635);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шат.
pub(crate) fn n3707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3696);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16383);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16384);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 16385);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39418);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67480);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word элизабет.
pub(crate) fn n126527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125229);
	s.is_empty()
}

/// Words ending -кводлибет.
pub(crate) fn n126528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125230);
	s.is_empty()
}

/// Words ending -ройдисвет.
pub(crate) fn n126529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125231);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 132833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ссвет.
pub(crate) fn n39819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39442);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 67497);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 93224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тсвет.
pub(crate) fn n39820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39443);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 67498);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 93225);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усвет.
pub(crate) fn n39821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39444);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 67499);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 93226);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 112648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысвет.
pub(crate) fn n39822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39445);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 67500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 93227);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ответ.
pub(crate) fn n39823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39446);
	s.is_empty()
}

/// Words ending -хвет.
pub(crate) fn n16500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16405);
	s.is_empty()
}

/// Words ending -цвет.
pub(crate) fn n16501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16406);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 39447);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 39448);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 39449);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 67501);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93228);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 112649);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125232);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39450);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 67502);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93229);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 112650);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 125233);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 39451);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 39452);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 67503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ювет.
pub(crate) fn n16502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16407);
	s.is_empty()
}

/// Words ending -гет.
pub(crate) fn n3717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3706);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16408);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39453);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 39454);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16409);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 39455);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67505);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 93231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39456);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 39457);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67507);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93232);
									s.is_empty()
								}
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
			super::push(out, 16410);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39458);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 67508);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 93233);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112651);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125234);
													s.is_empty()
												}
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
							super::push(out, 67509);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93234);
									s.is_empty()
								}
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
			super::push(out, 16411);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16412);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39460);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67510);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93235);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112652);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16413);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39461);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93236);
									s.is_empty()
								}
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
			super::push(out, 16414);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 39462);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67512);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93237);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112653);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дет.
pub(crate) fn n3718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3707);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16415);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16416);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39464);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 67513);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93238);
									s.is_empty()
								}
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
			super::push(out, 16417);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39465);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67514);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93239);
									s.is_empty()
								}
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
			super::push(out, 16418);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39466);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 67515);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93240);
									s.is_empty()
								}
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
			super::push(out, 16419);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39467);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жет.
pub(crate) fn n3719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3708);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16420);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39468);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67517);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 67518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39469);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 67519);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 67520);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93242);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112654);
											s.is_empty()
										}
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
					super::push(out, 39470);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16421);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39471);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67522);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93243);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112655);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16422);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 16423);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39473);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зет.
pub(crate) fn n3720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3709);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16424);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39474);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16425);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16426);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 39475);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67524);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93244);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112656);
											s.is_empty()
										}
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
					super::push(out, 39476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 16427);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39477);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67525);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 67526);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93246);
									s.is_empty()
								}
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
			super::push(out, 16428);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39478);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 39479);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67527);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93247);
									s.is_empty()
								}
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
			super::push(out, 16429);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39480);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16430);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39481);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67529);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93248);
									s.is_empty()
								}
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
			super::push(out, 16431);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иет.
pub(crate) fn n3721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3710);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 16432);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39482);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 67530);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93249);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112657);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кет.
pub(crate) fn n3722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3711);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16433);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16434);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 39483);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 67531);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93250);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 112658);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125235);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134203(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39484);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 67532);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93251);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112659);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125236);
													match s {
														[] => true,
														['у', s @ ..] => super::t38::n134204(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16435);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39485);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39486);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67533);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 67534);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93252);
									s.is_empty()
								}
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
			super::push(out, 16436);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39487);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39488);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67535);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 67536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 39489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16437);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67537);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 67538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93254);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 93255);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112660);
											s.is_empty()
										}
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
					super::push(out, 39490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16438);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67539);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 67540);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93256);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112661);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125237);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134205(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39492);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 39493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67541);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 67542);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93257);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112662);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16439);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39494);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67543);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 67544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93258);
									s.is_empty()
								}
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
			super::push(out, 16440);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67545);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 67546);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93259);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112663);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 93260);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112664);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 67547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39496);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16441);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 16442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39498);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 39499);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 16443);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39500);
					match s {
						[] => true,
						['о', s @ ..] => super::t38::n68248(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лет.
pub(crate) fn n3723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3712);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16444);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16445);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39501);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 67551);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93262);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 112666);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125239);
													match s {
														[] => true,
														['и', s @ ..] => super::t38::n134206(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 39503);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67553);
							match s {
								[] => true,
								['и', 'п', 'ш', '\u{1}', s @ ..] => {
									super::push(out, 93263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 39504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 16446);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39506);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 67555);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 67556);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 39507);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16447);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39508);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67558);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93265);
									s.is_empty()
								}
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
			super::push(out, 16448);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16449);
			match s {
				[] => true,
				['к', 'с', 'о', s @ ..] => {
					super::push(out, 39509);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39510);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67559);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93266);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 112667);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125240);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16450);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16451);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39511);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 39512);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67560);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 39513);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 67561);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93268);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 112668);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125241);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134207(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16452);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39514);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67562);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93269);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112669);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125242);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39515);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39516);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67563);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93270);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112670);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 125243);
													match s {
														[] => true,
														['б', s @ ..] => super::t38::n134208(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39517);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 39518);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67564);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 93271);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112671);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125244);
													match s {
														[] => true,
														['б', s @ ..] => super::t38::n134209(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t38::n16548(s, out),
		['м', s @ ..] => super::t38::n16549(s, out),
		['н', s @ ..] => super::t38::n16550(s, out),
		['о', s @ ..] => super::t38::n16551(s, out),
		['п', s @ ..] => super::t38::n16552(s, out),
		['с', s @ ..] => {
			super::push(out, 16458);
			s.is_empty()
		}
		['т', s @ ..] => super::t38::n16554(s, out),
		['у', s @ ..] => super::t38::n16555(s, out),
		['ф', s @ ..] => {
			super::push(out, 16461);
			s.is_empty()
		}
		['ы', s @ ..] => super::t38::n16557(s, out),
		_ => false,
	}
}

/// Words ending -мет.
pub(crate) fn n3724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3713);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16463);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39542);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 39543);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 16464);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 16465);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16466);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39546);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67593);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 93288);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112682);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 67594);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93289);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112683);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125251);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n134211(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 112684);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125252);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 16467);
			s.is_empty()
		}
		['и', 'ж', s @ ..] => {
			super::push(out, 16468);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39548);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67595);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93290);
									s.is_empty()
								}
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
			super::push(out, 16469);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39549);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67596);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93291);
									s.is_empty()
								}
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
			super::push(out, 16470);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16471);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39551);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 39552);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67597);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93292);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 67598);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 93293);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112686);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67599);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 67600);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112687);
											s.is_empty()
										}
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
					super::push(out, 39553);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 67601);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93295);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 112688);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125253);
													s.is_empty()
												}
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
							super::push(out, 67602);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93296);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112689);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125254);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39554);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 39555);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39556);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39557);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67603);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93297);
									match s {
										[] => true,
										['а', 'р', 'г', '-', s @ ..] => {
											super::push(out, 112690);
											match s {
												[] => true,
												['т', s @ ..] => super::t38::n126553(s, out),
												_ => false,
											}
										}
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
					super::push(out, 39558);
					match s {
						[] => true,
						['а', s @ ..] => super::t38::n68302(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'и', s @ ..] => super::t38::n16567(s, out),
		['у', s @ ..] => {
			super::push(out, 16472);
			s.is_empty()
		}
		['х', s @ ..] => super::t38::n16569(s, out),
		['ш', s @ ..] => super::t38::n16570(s, out),
		_ => false,
	}
}

/// Words ending -нет.
pub(crate) fn n3725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3714);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16475);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 39564);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67609);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93302);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 93303);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112692);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125256);
													s.is_empty()
												}
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
							super::push(out, 67610);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93304);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112693);
											s.is_empty()
										}
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
					super::push(out, 39565);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39566);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 39567);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16476);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16477);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39568);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39569);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67612);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93305);
									s.is_empty()
								}
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
			super::push(out, 16478);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39570);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67613);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93306);
									s.is_empty()
								}
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
			super::push(out, 16479);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39571);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67614);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93307);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112694);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16480);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39572);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67615);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93308);
									s.is_empty()
								}
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
			super::push(out, 16481);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39573);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67616);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93309);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112695);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16482);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39574);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67617);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39575);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67618);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39576);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67619);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 67620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 16483);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39577);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пет.
pub(crate) fn n3726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3715);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16484);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39578);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67622);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93313);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112696);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 93314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39579);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67623);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93315);
									s.is_empty()
								}
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
			super::push(out, 16485);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39580);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39581);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67624);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93316);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 39582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16486);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39583);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67625);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 39584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16487);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39585);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67627);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16488);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39587);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67629);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93319);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112697);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16489);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39588);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39589);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67631);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93320);
									s.is_empty()
								}
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
			super::push(out, 16490);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рет.
pub(crate) fn n3727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3716);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16491);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39590);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67632);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93321);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112698);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125257);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39591);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 39592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39593);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 39595);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16492);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39596);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67638);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93322);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112699);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16493);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16494);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39597);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 39598);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 16495);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39599);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67640);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93323);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112700);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16496);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39600);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67641);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93324);
									s.is_empty()
								}
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
			super::push(out, 16497);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39601);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67642);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93325);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112701);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125258);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39602);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39603);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67644);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 93326);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112702);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16498);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39604);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67645);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 39605);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67646);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93328);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112703);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16499);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 16500);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39606);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16501);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39608);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67647);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 39609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39611);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 16502);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39612);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 67651);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сет.
pub(crate) fn n3728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3717);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16503);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39613);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 39614);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 39615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 16504);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39616);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67653);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93331);
									s.is_empty()
								}
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
			super::push(out, 16505);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39617);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 39618);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67654);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93332);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112704);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16506);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39619);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 67655);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93333);
									s.is_empty()
								}
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
			super::push(out, 16507);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39620);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67656);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93334);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112705);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125259);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39621);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16508);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67658);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 67659);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39623);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67660);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93336);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112706);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16509);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39624);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тет.
pub(crate) fn n3729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3718);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16510);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16511);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39626);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16512);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39627);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 67663);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 39628);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 39629);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 39630);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 67664);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39631);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67665);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 67666);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93339);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112707);
											match s {
												[] => true,
												['а', '\u{1}', s @ ..] => {
													super::push(out, 125260);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39632);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 39633);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67667);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93340);
									s.is_empty()
								}
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
			super::push(out, 16513);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 39634);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67668);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93341);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112708);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16514);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39635);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 39636);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фет.
pub(crate) fn n3730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3719);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16515);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16516);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39637);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67670);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93342);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112709);
											s.is_empty()
										}
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
					super::push(out, 39638);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67671);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93343);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112710);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16517);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хет.
pub(crate) fn n3731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3720);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16518);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39640);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67672);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93344);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112711);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цет.
pub(crate) fn n3732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3721);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 16519);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39641);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 67673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93345);
									s.is_empty()
								}
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
			super::push(out, 16520);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 39642);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16521);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 39643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16522);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16523);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39644);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чет.
pub(crate) fn n3733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3722);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16524);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 39645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16525);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39646);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39647);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67676);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 67677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 39648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16526);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39649);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16527);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39650);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67680);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 67681);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93348);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112712);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 125261);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 39652);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39654);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 39655);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67683);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93349);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112713);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 125262);
													match s {
														[] => true,
														['и', s @ ..] => super::t38::n134213(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67684);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93350);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112714);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125263);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134214(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67685);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93351);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112715);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125264);
													match s {
														[] => true,
														['с', s @ ..] => super::t38::n134215(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39656);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 39657);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67687);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93352);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112716);
											s.is_empty()
										}
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
					super::push(out, 39658);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 39659);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39660);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67689);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93353);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112717);
											s.is_empty()
										}
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
					super::push(out, 39662);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67690);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93354);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112718);
											s.is_empty()
										}
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
					super::push(out, 39663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16529);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16530);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 16531);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67691);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 67692);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шет.
pub(crate) fn n3734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3723);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16532);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39666);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93356);
									s.is_empty()
								}
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
			super::push(out, 16533);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16534);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16535);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16536);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 39669);
					match s {
						[] => true,
						['ф', '-', s @ ..] => {
							super::push(out, 67695);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93357);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112719);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125265);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n134216(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьет.
pub(crate) fn n3735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3724);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16537);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ваххабит.
pub(crate) fn n126564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125266);
	s.is_empty()
}

/// Words ending -шерстобит.
pub(crate) fn n126565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125267);
	s.is_empty()
}

/// Words ending -вит.
pub(crate) fn n3738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3727);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16549);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39688);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67710);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 39689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16550);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39690);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67711);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 93370);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 112723);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125268);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134217(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39691);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67712);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93371);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112724);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125269);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n134218(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16551);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39693);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67714);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93372);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112725);
											s.is_empty()
										}
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
					super::push(out, 39694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16552);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39695);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 39696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16553);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 39697);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67717);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93373);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112726);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125270);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гит.
pub(crate) fn n3739(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3728);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16554);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 39699);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67719);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93374);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112727);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125271);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16555);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 39700);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39701);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39702);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67721);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93376);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112728);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16556);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39703);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67722);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93377);
									match s {
										[] => true,
										['х', 'о', s @ ..] => {
											super::push(out, 112729);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125272);
													match s {
														[] => true,
														['с', s @ ..] => super::t38::n134219(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39704);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67723);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93378);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112730);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125273);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', 'е', 'л', '\u{1}', s @ ..] => {
									super::push(out, 93379);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 93380);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112731);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125274);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n134220(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67724);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 67725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дит.
pub(crate) fn n3740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3729);
	match s {
		[] => true,
		['а', 'б', s @ ..] => {
			super::push(out, 16558);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 16559);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39706);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67727);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93381);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112732);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125275);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39707);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39708);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 39709);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67728);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93382);
									s.is_empty()
								}
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
			super::push(out, 16560);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39710);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67729);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39711);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67730);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 93384);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112733);
											s.is_empty()
										}
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
					super::push(out, 39712);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67731);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93385);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112734);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 67732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16561);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 16562);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39714);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 67733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93386);
									s.is_empty()
								}
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
			super::push(out, 16563);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39715);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67734);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 67735);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93387);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112735);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125276);
													match s {
														[] => true,
														['п', s @ ..] => super::t38::n134221(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67736);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 67737);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93388);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112736);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125277);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39716);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16564);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39717);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67739);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93389);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112737);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125278);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67740);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 67741);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93391);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112738);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 67742);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93392);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112739);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125279);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n134222(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16565);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39718);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16566);
			match s {
				[] => true,
				['а', 'с', s @ ..] => {
					super::push(out, 39719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 39720);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67745);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39721);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67746);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93394);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112740);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => super::t38::n68445(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => super::t38::n16664(s, out),
		['э', s @ ..] => super::t38::n16665(s, out),
		_ => false,
	}
}

/// Words ending -геит.
pub(crate) fn n16668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16571);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 39726);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67751);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 93397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кеит.
pub(crate) fn n16669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16572);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 39727);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 67752);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 93398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -леит.
pub(crate) fn n16670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16573);
	s.is_empty()
}

/// Words ending -пеит.
pub(crate) fn n16671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16574);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 39728);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 67753);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 93399);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 112742);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 125281);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132855);
													match s {
														[] => true,
														['и', s @ ..] => super::t38::n138760(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теит.
pub(crate) fn n16672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16575);
	s.is_empty()
}

/// Words ending -хеит.
pub(crate) fn n16673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16576);
	s.is_empty()
}

/// Words ending -жит.
pub(crate) fn n3742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3731);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16577);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39729);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67754);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93400);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зит.
pub(crate) fn n3743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3732);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16578);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39730);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39731);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67755);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112743);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125282);
													match s {
														[] => true,
														['в', s @ ..] => super::t38::n134225(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 112744);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 125283);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n134226(s, out),
														_ => false,
													}
												}
												['л', s @ ..] => {
													super::push(out, 125284);
													match s {
														[] => true,
														['л', s @ ..] => super::t38::n134227(s, out),
														_ => false,
													}
												}
												['м', s @ ..] => {
													super::push(out, 125285);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n134228(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													match s {
														[] => true,
														['в', s @ ..] => super::t38::n134229(s, out),
														['к', s @ ..] => super::t38::n134230(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67756);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93401);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112745);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16579);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39732);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93402);
									s.is_empty()
								}
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
			super::push(out, 16580);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16581);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 16582);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39733);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67758);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93403);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 93404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 39734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16583);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16584);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иит.
pub(crate) fn n3744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3733);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 16585);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16586);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 16587);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йит.
pub(crate) fn n3745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3734);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16588);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39739);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67760);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93405);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112746);
											s.is_empty()
										}
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
					super::push(out, 39740);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67761);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93406);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112747);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125286);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39741);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кит.
pub(crate) fn n3746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3735);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16589);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 16590);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39742);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67762);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 93407);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112748);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125287);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16591);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39743);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 67763);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93408);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112749);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16592);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16593);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39744);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67764);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39745);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67765);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93410);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112750);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 67766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16594);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39746);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67767);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93411);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112751);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125288);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16595);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39747);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 39748);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39749);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67768);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16596);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 16597);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39751);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67769);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лит.
pub(crate) fn n3747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3736);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16598);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16599);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39753);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 67771);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93414);
									s.is_empty()
								}
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
			super::push(out, 16600);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67772);
							s.is_empty()
						}
						['а', '\u{1}', s @ ..] => {
							super::push(out, 67773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 39755);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 39756);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67774);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93415);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112752);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125289);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39757);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 39758);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67776);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93416);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112753);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125290);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'а', s @ ..] => {
			super::push(out, 16601);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 39759);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67777);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93417);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112754);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'а', s @ ..] => {
			super::push(out, 16602);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39760);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67778);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93418);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112755);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16603);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39761);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 39762);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67779);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93419);
									s.is_empty()
								}
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
			super::push(out, 16604);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39763);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 67780);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93420);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112756);
											s.is_empty()
										}
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
					super::push(out, 39764);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 39765);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 39766);
					s.is_empty()
				}
				['к', 'а', s @ ..] => {
					super::push(out, 39767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39768);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 39769);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93421);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 93422);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112757);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 67782);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93423);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 112758);
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
								['л', s @ ..] => {
									super::push(out, 93424);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 93425);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112759);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125291);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n134231(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', 'т', 'и', 'м', '\u{1}', s @ ..] => {
									super::push(out, 93426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 67783);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39770);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'о', s @ ..] => {
			super::push(out, 16605);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16606);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39773);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 39774);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39775);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мит.
pub(crate) fn n3748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3737);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16607);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39776);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67786);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 93428);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112760);
											s.is_empty()
										}
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
					super::push(out, 39777);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 39778);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							super::push(out, 67787);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 67788);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93429);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112761);
											s.is_empty()
										}
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
					super::push(out, 39779);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 39780);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67789);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 67790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93430);
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
						['и', s @ ..] => {
							super::push(out, 67791);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93431);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 67792);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93432);
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
						['б', s @ ..] => {
							super::push(out, 67793);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93433);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 67794);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 67795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 39781);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 39782);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67797);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 39783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16608);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16609);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39785);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 39786);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67799);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39787);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67800);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39788);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67801);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93437);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112762);
											s.is_empty()
										}
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
					super::push(out, 39789);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67802);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93438);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112763);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125292);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16610);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39790);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39791);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 39792);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67804);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93439);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112764);
											s.is_empty()
										}
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
					super::push(out, 39793);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67805);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93440);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112765);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16611);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39794);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16612);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39795);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67807);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39796);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16613);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39797);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67809);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93442);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112766);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125293);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n134232(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t38::n40182(s, out),
				['л', s @ ..] => {
					super::push(out, 39799);
					match s {
						[] => true,
						['о', s @ ..] => super::t38::n68512(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t38::n40184(s, out),
				['с', s @ ..] => super::t38::n40185(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t38::n16711(s, out),
		['у', 'с', s @ ..] => super::t38::n16712(s, out),
		_ => false,
	}
}

/// Words ending -нит.
pub(crate) fn n3749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3738);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16616);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39803);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67817);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93448);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112769);
											s.is_empty()
										}
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
					super::push(out, 39804);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67818);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93449);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112770);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125294);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n134233(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39805);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67819);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 39806);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67820);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93451);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 112771);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125295);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16617);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16618);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39808);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 39809);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 39810);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67822);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93452);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112772);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 125296);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39811);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67823);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93453);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 112773);
											s.is_empty()
										}
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
					super::push(out, 39812);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67824);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93454);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112774);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 125297);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134234(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39813);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67825);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93455);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112775);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16619);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39814);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67826);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 67827);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93457);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112776);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16620);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39815);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39816);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67829);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93459);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112777);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16621);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 67830);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 67831);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93460);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112778);
											s.is_empty()
										}
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
					super::push(out, 39817);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t38::n16719(s, out),
		['р', s @ ..] => {
			super::push(out, 16623);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39825);
					s.is_empty()
				}
				['о', s @ ..] => super::t38::n40212(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t38::n16721(s, out),
		['ю', s @ ..] => super::t38::n16722(s, out),
		_ => false,
	}
}

/// Words ending -оит.
pub(crate) fn n3750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3739);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 16626);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39829);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67840);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93466);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 112783);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125299);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пит.
pub(crate) fn n3751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3740);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16627);
			s.is_empty()
		}
		['-', 'л', 'к', 'р', 'е', 'с', '\u{1}', s @ ..] => {
			super::push(out, 16628);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16629);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39830);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67841);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16630);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16631);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 39832);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67843);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93468);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112784);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16632);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39833);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67844);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93469);
									s.is_empty()
								}
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
			super::push(out, 16633);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 16634);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рит.
pub(crate) fn n3752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3741);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16635);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67845);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 67846);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93470);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112785);
											s.is_empty()
										}
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
					super::push(out, 39834);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 39835);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67848);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93471);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112786);
											s.is_empty()
										}
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
					super::push(out, 39836);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							super::push(out, 67849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 16636);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16637);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39838);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 39839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 16638);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 39840);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67850);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93472);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112787);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125300);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134236(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
						['а', s @ ..] => {
							super::push(out, 67851);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93473);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112788);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125301);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134237(s, out),
														_ => false,
													}
												}
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
							super::push(out, 67852);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 39841);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16639);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39842);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 67854);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93475);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112789);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125302);
													s.is_empty()
												}
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
							super::push(out, 67855);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93476);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112790);
											s.is_empty()
										}
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
					super::push(out, 39843);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67856);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93477);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112791);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 67857);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93478);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112792);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 67858);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 39844);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 39845);
					match s {
						[] => true,
						['е', 'в', '\u{1}', s @ ..] => {
							super::push(out, 67859);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 67860);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 67861);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93480);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112793);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125303);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39846);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67862);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39847);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39848);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67863);
							match s {
								[] => true,
								['р', s @ ..] => super::t38::n94483(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => super::t38::n40237(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t38::n16737(s, out),
		['и', s @ ..] => super::t38::n16738(s, out),
		['й', s @ ..] => super::t38::n16739(s, out),
		['к', s @ ..] => super::t38::n16740(s, out),
		['о', s @ ..] => super::t38::n16741(s, out),
		['п', s @ ..] => super::t38::n16742(s, out),
		['р', s @ ..] => super::t38::n16743(s, out),
		['т', s @ ..] => super::t38::n16744(s, out),
		['у', s @ ..] => {
			super::push(out, 16648);
			s.is_empty()
		}
		['ф', s @ ..] => super::t38::n16746(s, out),
		['я', s @ ..] => super::t38::n16747(s, out),
		_ => false,
	}
}

/// Words ending -сит.
pub(crate) fn n3753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3742);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16651);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16652);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16653);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 39877);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67887);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93500);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112805);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16654);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39878);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 67888);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93501);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112806);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 125312);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16655);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39879);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93502);
									s.is_empty()
								}
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
			super::push(out, 16656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39880);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 39881);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67890);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93503);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112807);
											s.is_empty()
										}
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
					super::push(out, 39882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16657);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39883);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 67891);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93504);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39884);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39885);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67892);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93505);
									s.is_empty()
								}
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
			super::push(out, 16658);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16659);
			match s {
				[] => true,
				['а', 'к', '\u{1}', s @ ..] => {
					super::push(out, 39886);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 39887);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67893);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 67894);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93507);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 67895);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93508);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112808);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125313);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39888);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 39889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 16660);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 39891);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 39892);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67897);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39893);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 67898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 39894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 16661);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39895);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67900);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тит.
pub(crate) fn n3754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3743);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16662);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16663);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39896);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67901);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93511);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112809);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125314);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39897);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39898);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67902);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 93512);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112810);
											s.is_empty()
										}
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
					super::push(out, 39899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16664);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67904);
							s.is_empty()
						}
						['о', 'р', 'д', 'и', 'г', '\u{1}', s @ ..] => {
							super::push(out, 67905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 39900);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67906);
							match s {
								[] => true,
								['а', '\u{1}', s @ ..] => {
									super::push(out, 93513);
									s.is_empty()
								}
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
			super::push(out, 16665);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 16666);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39901);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67907);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93514);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112811);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16667);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16668);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39902);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 67908);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93515);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112812);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125315);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134243(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39903);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уит.
pub(crate) fn n3755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3744);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 16669);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 16670);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39904);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16671);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39905);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67910);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93516);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112813);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16672);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67911);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 67912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 39906);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67913);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 93517);
									s.is_empty()
								}
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
			super::push(out, 16673);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фит.
pub(crate) fn n3756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3745);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16674);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16675);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39907);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67914);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93518);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112814);
											s.is_empty()
										}
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
					super::push(out, 39908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16676);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39909);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 67915);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 39910);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 67916);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 39911);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16677);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39913);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67917);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93521);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112815);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125316);
													match s {
														[] => true,
														['л', s @ ..] => super::t38::n134244(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16678);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 39914);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67918);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93522);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112816);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16679);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хит.
pub(crate) fn n3757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3746);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16680);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 16681);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39915);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 67919);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93523);
									s.is_empty()
								}
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
			super::push(out, 16682);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 39916);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67920);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93524);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112817);
											s.is_empty()
										}
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
					super::push(out, 39917);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 39918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16683);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 39919);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67921);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93525);
									s.is_empty()
								}
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
			super::push(out, 16684);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 39920);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 67922);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93526);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112818);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125317);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n134245(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16685);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 39921);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16686);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39922);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 67924);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93527);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112819);
											s.is_empty()
										}
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
					super::push(out, 39923);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 67925);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93528);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'е', s @ ..] => {
			super::push(out, 16687);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 39924);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 67926);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цит.
pub(crate) fn n3758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3747);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16688);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 39925);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16689);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 39926);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67928);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93530);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112820);
											s.is_empty()
										}
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
					super::push(out, 39927);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 67929);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93531);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112821);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125318);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чит.
pub(crate) fn n3759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3748);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 16690);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39928);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67930);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93532);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 112822);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125319);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n134246(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16691);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 39929);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67931);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93533);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112823);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16692);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39930);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 67932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шит.
pub(crate) fn n3760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3749);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16693);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 16694);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39932);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67934);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93534);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112824);
											s.is_empty()
										}
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
					super::push(out, 39933);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67935);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 93535);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112825);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16695);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39934);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 39935);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 67937);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93536);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112826);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щит.
pub(crate) fn n3761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3750);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16696);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16697);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39936);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 67938);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93537);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112827);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125320);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n134247(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уотергейт.
pub(crate) fn n126620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125321);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 132879);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word саутгейт.
pub(crate) fn n126621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125322);
	s.is_empty()
}

/// Words ending -тселийт.
pub(crate) fn n94547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93546);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 112834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 125323);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 132880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ойт.
pub(crate) fn n3765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3754);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 39954);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -втодидакт.
pub(crate) fn n126623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125324);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132881);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -таз-компакт.
pub(crate) fn n126624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125325);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 132882);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 137356);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 140026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word дистрикт.
pub(crate) fn n126625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125326);
	s.is_empty()
}

/// Words ending -олго-балт.
pub(crate) fn n126626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125327);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 132883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -штык-болт.
pub(crate) fn n126627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125328);
	s.is_empty()
}

/// Words ending -пустоболт.
pub(crate) fn n126628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125329);
	s.is_empty()
}

/// Words ending -ант.
pub(crate) fn n3777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3766);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16738);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 67972);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93563);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 93564);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 93565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 39976);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67973);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 39977);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67974);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93567);
									s.is_empty()
								}
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
			super::push(out, 16739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 39978);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 67975);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 67976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 39979);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 67977);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93569);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112846);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125330);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n134253(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 39980);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 39981);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 39982);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 67978);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93570);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112847);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125331);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 39983);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 67979);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 39984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16740);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 67980);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 67981);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93572);
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
							super::push(out, 67982);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 93573);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 67983);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93574);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112848);
											s.is_empty()
										}
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
					super::push(out, 39985);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93575);
									s.is_empty()
								}
								['-', s @ ..] => super::t38::n94577(s, out),
								['б', s @ ..] => {
									super::push(out, 93577);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 112851);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125334);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t38::n94579(s, out),
								['х', s @ ..] => super::t38::n94580(s, out),
								_ => false,
							}
						}
						['м', s @ ..] => super::t38::n68686(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t38::n40381(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t38::n16838(s, out),
		['ж', s @ ..] => super::t38::n16839(s, out),
		['з', s @ ..] => super::t38::n16840(s, out),
		['и', s @ ..] => super::t38::n16841(s, out),
		['к', s @ ..] => super::t38::n16842(s, out),
		['л', s @ ..] => super::t38::n16843(s, out),
		['м', s @ ..] => super::t38::n16844(s, out),
		['н', s @ ..] => super::t38::n16845(s, out),
		['п', s @ ..] => super::t38::n16846(s, out),
		['р', s @ ..] => super::t38::n16847(s, out),
		['с', s @ ..] => super::t38::n16848(s, out),
		['т', s @ ..] => super::t38::n16849(s, out),
		['у', s @ ..] => super::t38::n16850(s, out),
		['ф', s @ ..] => super::t38::n16851(s, out),
		['х', s @ ..] => super::t38::n16852(s, out),
		['ч', s @ ..] => super::t38::n16853(s, out),
		_ => false,
	}
}

/// Words ending -ент.
pub(crate) fn n3778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3767);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16755);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 40061);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68109);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93699);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112935);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125375);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 40062);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16756);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40063);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 40064);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68112);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 93700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 68113);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93701);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112936);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125376);
													match s {
														[] => true,
														['д', s @ ..] => super::t38::n134272(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 40065);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68114);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40066);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68115);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93703);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112937);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125377);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', 'л', 'о', 'с', 'б', s @ ..] => {
					super::push(out, 40067);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93704);
									s.is_empty()
								}
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
			super::push(out, 16757);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40068);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 68117);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93705);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112938);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125378);
													match s {
														[] => true,
														['б', s @ ..] => super::t38::n134273(s, out),
														_ => false,
													}
												}
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
							super::push(out, 68118);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93706);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112939);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125379);
													s.is_empty()
												}
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
							super::push(out, 68119);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 68120);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93708);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112940);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 68121);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112941);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125380);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n134274(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 112942);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125381);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n134275(s, out),
														_ => false,
													}
												}
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
							super::push(out, 68122);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93709);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t38::n40472(s, out),
				['и', s @ ..] => super::t38::n40473(s, out),
				['н', s @ ..] => super::t38::n40474(s, out),
				['р', s @ ..] => super::t38::n40475(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t38::n16857(s, out),
		['ж', s @ ..] => super::t38::n16858(s, out),
		['з', s @ ..] => super::t38::n16859(s, out),
		['и', s @ ..] => super::t38::n16860(s, out),
		['к', s @ ..] => super::t38::n16861(s, out),
		['л', s @ ..] => super::t38::n16862(s, out),
		['м', s @ ..] => super::t38::n16863(s, out),
		['н', s @ ..] => super::t38::n16864(s, out),
		['п', s @ ..] => super::t38::n16865(s, out),
		['р', s @ ..] => super::t38::n16866(s, out),
		['с', s @ ..] => super::t38::n16867(s, out),
		['т', s @ ..] => super::t38::n16868(s, out),
		['у', s @ ..] => super::t38::n16869(s, out),
		['ц', s @ ..] => super::t38::n16870(s, out),
		['ю', s @ ..] => super::t38::n16871(s, out),
		_ => false,
	}
}

/// Words ending -родонт.
pub(crate) fn n68946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68230);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 93825);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 113044);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фодонт.
pub(crate) fn n68948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68232);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 93826);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 113045);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зонт.
pub(crate) fn n16884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16785);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 40151);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 40152);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68233);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40153);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 68234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ионт.
pub(crate) fn n16885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16786);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 40154);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68235);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['с', 'к', 'о', 'н', s @ ..] => {
							super::push(out, 93829);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113046);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 93830);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113047);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 125450);
											s.is_empty()
										}
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
					super::push(out, 68236);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 93831);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113048);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 68237);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 93832);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 113049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'а', s @ ..] => {
							super::push(out, 93833);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 113050);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125451);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 93834);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 113051);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 125452);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 132941);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n138806(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', 'н', s @ ..] => {
									super::push(out, 113052);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 125453);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 132942);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n138807(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -конт.
pub(crate) fn n16886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16787);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 40155);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 68238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40156);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 68239);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 93835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 40157);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 68240);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 93836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -монт.
pub(crate) fn n16887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16788);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40158);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 68241);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 68242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 93837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40159);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40160);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 68244);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 93838);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113053);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 125454);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -понт.
pub(crate) fn n16888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16789);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 40161);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ронт.
pub(crate) fn n16889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16790);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 40162);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40163);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 68246);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 93839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 40164);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68247);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 68248);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 93840);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 93841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 68249);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93842);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113054);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 125455);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132943);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фонт.
pub(crate) fn n16890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16791);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40165);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 68250);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 93843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 40166);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 68251);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 93844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40167);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 68252);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 93845);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 113055);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', 'л', 'л', 'е', 'б', '\u{1}', s @ ..] => {
					super::push(out, 68253);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хонт.
pub(crate) fn n16891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16792);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 40168);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 40169);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 68255);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унт.
pub(crate) fn n3782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3771);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16793);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16794);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40170);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68256);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 40171);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 16795);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 16796);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 40172);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16797);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 40174);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 40175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 16798);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 16799);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40176);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янт.
pub(crate) fn n3783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3772);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 16800);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68260);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93847);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 113056);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 68261);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93848);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 113057);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125456);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134314(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 40177);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 40178);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 68263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 68264);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93849);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113058);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125457);
													s.is_empty()
												}
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
							super::push(out, 68265);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93851);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 113059);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 93852);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 113060);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125458);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16801);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40179);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68266);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93853);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113061);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -порохобот.
pub(crate) fn n126758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125459);
	s.is_empty()
}

/// Words ending -ьбот.
pub(crate) fn n16910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16811);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68288);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 68289);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 93863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вот.
pub(crate) fn n3785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3774);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16812);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16813);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16814);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 40202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 40203);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 40204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16815);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40205);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гот.
pub(crate) fn n3786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3775);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16816);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 40206);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 40207);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68293);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93864);
									s.is_empty()
								}
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
			super::push(out, 16817);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40208);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68294);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 93865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16818);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40210);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16819);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40211);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 68297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16820);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40212);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68298);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93866);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113067);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16821);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16822);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40214);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дот.
pub(crate) fn n3787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3776);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16823);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 40215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16824);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 16825);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40216);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еот.
pub(crate) fn n3788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3777);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 16826);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 40217);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68302);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зот.
pub(crate) fn n3789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3778);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16827);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 16828);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 16829);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40218);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иот.
pub(crate) fn n3790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3779);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16830);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16831);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40220);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 68303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 16832);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16833);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40223);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 68304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16835);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16836);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40224);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68305);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 93868);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 113068);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 93869);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113069);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 93870);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 113070);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 93871);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 113071);
											s.is_empty()
										}
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
					super::push(out, 40225);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40226);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 68307);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 93872);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113072);
											s.is_empty()
										}
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
					super::push(out, 40227);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68308);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йот.
pub(crate) fn n3791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3780);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16837);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16838);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16839);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кот.
pub(crate) fn n3792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3781);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16840);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16841);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40229);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68309);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40230);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93875);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113073);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125460);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 93876);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113074);
											s.is_empty()
										}
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
					super::push(out, 40231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16842);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 40232);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 16843);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40233);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16844);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 40234);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 68312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 16845);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40235);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							super::push(out, 68313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 40236);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16846);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40237);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 40238);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40239);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 40240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16847);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40241);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 68317);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93877);
									s.is_empty()
								}
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
			super::push(out, 16848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40242);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40243);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 68318);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93878);
									s.is_empty()
								}
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
			super::push(out, 16849);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 40244);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 68319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93879);
									s.is_empty()
								}
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
			super::push(out, 16850);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 40245);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68320);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93880);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 113075);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 125461);
													match s {
														[] => true,
														['щ', s @ ..] => super::t38::n134315(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16851);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лот.
pub(crate) fn n3793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3782);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 16852);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 40247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 16853);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 40248);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 68321);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93881);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 113076);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125462);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134316(s, out),
														_ => false,
													}
												}
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
							super::push(out, 68322);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93882);
									s.is_empty()
								}
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
			super::push(out, 16854);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40249);
					s.is_empty()
				}
				['а', 'з', s @ ..] => {
					super::push(out, 40250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 40251);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68324);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 40252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16855);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 40253);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68325);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93884);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 40254);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 16856);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 40255);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 40256);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 40257);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68327);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 68328);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93885);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113077);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125463);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134317(s, out),
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
									super::push(out, 93886);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113078);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125464);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 93887);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113079);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 125465);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 93888);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 113080);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125466);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134318(s, out),
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
									super::push(out, 93889);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113081);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 125467);
													s.is_empty()
												}
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
							super::push(out, 68329);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 93890);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113082);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 125468);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 93891);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 113083);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125469);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40258);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t38::n16957(s, out),
		['п', s @ ..] => {
			super::push(out, 16859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40262);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40263);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 40264);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 40265);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16860);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16861);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 40268);
					match s {
						[] => true,
						['м', s @ ..] => super::t38::n69060(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t38::n16961(s, out),
		['ф', s @ ..] => super::t38::n16962(s, out),
		['ш', s @ ..] => super::t38::n16963(s, out),
		['ю', s @ ..] => super::t38::n16964(s, out),
		_ => false,
	}
}

/// Words ending -мот.
pub(crate) fn n3794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3783);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16866);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16867);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40277);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16868);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68349);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93902);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113093);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125475);
													s.is_empty()
												}
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
							super::push(out, 68350);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93903);
									s.is_empty()
								}
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
			super::push(out, 16869);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 16870);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40278);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 16871);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40279);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нот.
pub(crate) fn n3795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3784);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 16872);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 16873);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40280);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 68352);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16874);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 40282);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40283);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16875);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 40284);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пот.
pub(crate) fn n3796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3785);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16876);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16877);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40285);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 40286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16878);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 40287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68356);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 68357);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 68358);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93905);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113094);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16879);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40288);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 68359);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93906);
									s.is_empty()
								}
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
			super::push(out, 16880);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40289);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16881);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40290);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68361);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 93907);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113095);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 68362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40291);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 40292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16882);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 40293);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 68364);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93908);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 113096);
											s.is_empty()
										}
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
					super::push(out, 40294);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68365);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 40295);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 40296);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68367);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93910);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113097);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16883);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рот.
pub(crate) fn n3797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3786);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16884);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 16885);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 16886);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40298);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 68368);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93911);
									s.is_empty()
								}
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
			super::push(out, 16887);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 40299);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68369);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93912);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 113098);
											s.is_empty()
										}
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
					super::push(out, 40300);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 68370);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93913);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113099);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125476);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16888);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 40301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 16889);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40303);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 40304);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68373);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 40305);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68374);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93915);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 113100);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125477);
													s.is_empty()
												}
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
							super::push(out, 68375);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93916);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 113101);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 125478);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16891);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40306);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 40307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68377);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 68378);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 93917);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113102);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 68379);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 68380);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 68381);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 93918);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113103);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 68382);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 93919);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113104);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 68383);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 93920);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113105);
											s.is_empty()
										}
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
					super::push(out, 40308);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68384);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 40309);
					match s {
						[] => true,
						['о', 'р', s @ ..] => {
							super::push(out, 68385);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93922);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 113106);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125479);
													s.is_empty()
												}
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
							super::push(out, 68386);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93923);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 113107);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125480);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 40310);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 68387);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93924);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113108);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16892);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 40311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16893);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сот.
pub(crate) fn n3798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3787);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 16894);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40313);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 68389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тот.
pub(crate) fn n3799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3788);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 16896);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40314);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 68390);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93925);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113109);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16897);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40315);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -афот.
pub(crate) fn n16997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16898);
	s.is_empty()
}

/// Words ending -офот.
pub(crate) fn n16998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16899);
	s.is_empty()
}

/// Words ending -хот.
pub(crate) fn n3801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3790);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 16900);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40316);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 68392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 16901);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40317);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68393);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93926);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 113110);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 68394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 93927);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 93928);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113111);
											s.is_empty()
										}
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
					super::push(out, 40318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16902);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шот.
pub(crate) fn n3802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3791);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16903);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 16904);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16905);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40322);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 68396);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 93929);
									s.is_empty()
								}
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
			super::push(out, 16906);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 16907);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -айенс-арт.
pub(crate) fn n126780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125481);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 132953);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word йифф-арт.
pub(crate) fn n126781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125482);
	s.is_empty()
}

/// The word штутгарт.
pub(crate) fn n126782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125483);
	s.is_empty()
}

/// Words ending -ерт.
pub(crate) fn n3809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3798);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16929);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 40343);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68410);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93941);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113116);
											s.is_empty()
										}
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
					super::push(out, 40344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40345);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68412);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 93942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 40346);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 93943);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113117);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 93944);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 113118);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 93945);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 113119);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16930);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40347);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68413);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93946);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 113120);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125484);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n134324(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 40348);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68414);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93947);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 113121);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125485);
													match s {
														[] => true,
														['д', 'о', 'п', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t38::n134325(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 40349);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 40350);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68415);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93948);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113122);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 68416);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 93949);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 113123);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125486);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 93950);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113124);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125487);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 40351);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68417);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 40352);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68418);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93952);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 113125);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125488);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n134326(s, out),
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
									super::push(out, 93953);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113126);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125489);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 93954);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 113127);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125490);
													s.is_empty()
												}
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
							super::push(out, 68419);
							match s {
								[] => true,
								['у', s @ ..] => super::t38::n94963(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t38::n69140(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 40353);
					s.is_empty()
				}
				['ш', s @ ..] => super::t38::n40763(s, out),
				['ы', s @ ..] => super::t38::n40764(s, out),
				['ь', s @ ..] => super::t38::n40765(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t38::n17030(s, out),
		['л', s @ ..] => super::t38::n17031(s, out),
		['п', s @ ..] => super::t38::n17032(s, out),
		['с', s @ ..] => super::t38::n17033(s, out),
		['т', s @ ..] => super::t38::n17034(s, out),
		['ф', s @ ..] => super::t38::n17035(s, out),
		['х', s @ ..] => super::t38::n17036(s, out),
		['ц', s @ ..] => super::t38::n17037(s, out),
		['ч', s @ ..] => super::t38::n17038(s, out),
		_ => false,
	}
}

