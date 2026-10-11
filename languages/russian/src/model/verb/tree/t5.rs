use flexion::Path;

/// Words ending -хтать.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1996);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6348);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 13707);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30325);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30326);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30327);
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
				['п', s @ ..] => {
					super::push(out, 6349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13708);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13709);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 22176);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1997);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6350);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6351);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22177);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30328);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чтать.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1998);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13710);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13711);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 13712);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13713);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 22178);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 22179);
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
						['ы', s @ ..] => {
							super::push(out, 13714);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22180);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штать.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1999);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6352);
					match s {
						[] => true,
						['й', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытать.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2000);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6353);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6354);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2001);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6355);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6356);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6357);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13716);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22181);
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
					super::push(out, 6358);
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
					super::push(out, 6359);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13717);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22182);
									match s {
										[] => true,
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
							super::push(out, 13718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22183);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30329);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6360);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13719);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятать.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2002);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6361);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13720);
							match s {
								[] => true,
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
					super::push(out, 6362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13721);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оахать.
pub(crate) fn n2117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2008);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 6379);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6380);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пахать.
pub(crate) fn n2118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2009);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6381);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6382);
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
			super::push(out, 6383);
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
			super::push(out, 6384);
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
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6385);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рахать.
pub(crate) fn n2119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2010);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6386);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13736);
					match s {
						[] => true,
						['а', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 13737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6387);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13738);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13739);
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
					super::push(out, 13740);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13741);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13742);
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
			super::push(out, 6388);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13743);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 22189);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ехать.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2011);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2012);
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
		['е', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6389);
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
		['м', s @ ..] => {
			super::push(out, 2015);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6390);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 13744);
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
		['о', s @ ..] => {
			super::push(out, 2016);
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
		['п', s @ ..] => {
			super::push(out, 2017);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6391);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13745);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2018);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13746);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13747);
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
										['о', s @ ..] => {
											super::push(out, 30330);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36073);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76477(s, out),
														_ => false,
													}
												}
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
							super::push(out, 13748);
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
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 6393);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 13749);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2019);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 2020);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13750);
							match s {
								[] => true,
								['р', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 13751);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13752);
							match s {
								[] => true,
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2021);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6394);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13753);
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

/// Words ending -ихать.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 409);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 2022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6395);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6396);
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6397);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2023);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6398);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 6399);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6400);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 13755);
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
		['ч', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6402);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6403);
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
					super::push(out, 6404);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13758);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22190);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6405);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13759);
							match s {
								[] => true,
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
					super::push(out, 6406);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13760);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -охать.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2025);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2026);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6407);
					match s {
						[] => true,
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
			super::push(out, 2027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6408);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6409);
					match s {
						[] => true,
						['и', 'п', 'т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6410);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13761);
							match s {
								[] => true,
								['х', 'у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6411);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22191);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 22192);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22193);
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
		['з', s @ ..] => {
			super::push(out, 2028);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6412);
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
		['к', s @ ..] => {
			super::push(out, 2029);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2030);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6414);
					match s {
						[] => true,
						['п', 'с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2031);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6415);
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
		['о', s @ ..] => {
			super::push(out, 2032);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6416);
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
			super::push(out, 2033);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13762);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13763);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22194);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 6418);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13764);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пхать.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2034);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2035);
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

/// Words ending -рхать.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2036);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13765);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6420);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13766);
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
			super::push(out, 2037);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6421);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13767);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13768);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22196);
									match s {
										[] => true,
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
							super::push(out, 13769);
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
		['у', s @ ..] => {
			super::push(out, 2038);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 6422);
					match s {
						[] => true,
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
			super::push(out, 2039);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ухать.
pub(crate) fn n417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2040);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2041);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6424);
					match s {
						[] => true,
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
			super::push(out, 2042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6425);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6426);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13770);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 22197);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30331);
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
						['н', s @ ..] => {
							super::push(out, 13771);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6427);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6428);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13772);
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
					super::push(out, 6429);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13773);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6430);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13774);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 13775);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13776);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22199);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30332);
											s.is_empty()
										}
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
					super::push(out, 6431);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13777);
							match s {
								[] => true,
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
					super::push(out, 6432);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6433);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13778);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6434);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13779);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2044);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6435);
					match s {
						[] => true,
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
			super::push(out, 2045);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6436);
					match s {
						[] => true,
						['а', 'л', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22200);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30333);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2046);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6437);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 13780);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 13781);
							match s {
								[] => true,
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
					super::push(out, 6438);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13782);
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
					super::push(out, 6439);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6441);
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
		['р', s @ ..] => {
			super::push(out, 2047);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13783);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13784);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22201);
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
		['т', s @ ..] => {
			super::push(out, 2048);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6443);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6444);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6445);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13785);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 13786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t12::n18423(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t12::n2158(s, out),
		_ => false,
	}
}

/// Words ending -чхать.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2050);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыхать.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2052);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6451);
					match s {
						[] => true,
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
			super::push(out, 2053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6452);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6454);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6455);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13789);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22202);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22203);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30334);
											match s {
												[] => true,
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
									super::push(out, 22204);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30335);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30336);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36074);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76478(s, out),
														_ => false,
													}
												}
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
							super::push(out, 13790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13791);
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
					super::push(out, 6456);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13792);
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
					super::push(out, 6457);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13794);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22205);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6459);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6460);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13795);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2054);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6461);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 13796);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22206);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22207);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22208);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 22209);
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
						['п', s @ ..] => {
							super::push(out, 13797);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22210);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22211);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22212);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 22213);
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
				['с', s @ ..] => {
					super::push(out, 6462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13798);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13799);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22214);
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
						['у', s @ ..] => {
							super::push(out, 13800);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22215);
									s.is_empty()
								}
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
			super::push(out, 2055);
			match s {
				[] => true,
				['о', s @ ..] => super::t12::n7633(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t12::n2165(s, out),
		['р', s @ ..] => super::t12::n2166(s, out),
		['с', s @ ..] => super::t12::n2167(s, out),
		['т', s @ ..] => super::t12::n2168(s, out),
		_ => false,
	}
}

/// Words ending -эхать.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 416);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юхать.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 417);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2060);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6477);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13813);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13814);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 30343);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 36077);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t12::n76481(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2061);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13815);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13816);
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
		['н', s @ ..] => {
			super::push(out, 2062);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6478);
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
				['о', s @ ..] => {
					super::push(out, 6479);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 6480);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13817);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2063);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13818);
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
		['т', s @ ..] => {
			super::push(out, 2064);
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

/// Words ending -яхать.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 418);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2065);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13819);
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13820);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22222);
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
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13821);
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
							super::push(out, 13822);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30344);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36078);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76482(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word завосклицать.
pub(crate) fn n65451(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -посозерцать.
pub(crate) fn n65452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36079);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обосцать.
pub(crate) fn n18498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13831);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -посцать.
pub(crate) fn n7678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6494);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уцать.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 425);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2077);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яцать.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 426);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2078);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6495);
					match s {
						[] => true,
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
			super::push(out, 2079);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13832);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13833);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22229);
									match s {
										[] => true,
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
							super::push(out, 13834);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22230);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 22231);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30347);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word переоблачать.
pub(crate) fn n65454(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -отачать.
pub(crate) fn n7705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6508);
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

/// Words ending -стачать.
pub(crate) fn n7706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6509);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13838);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22235);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытачать.
pub(crate) fn n7708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6510);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13839);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гчать.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2086);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13840);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22236);
									match s {
										[] => true,
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
							super::push(out, 13841);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22237);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2087);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13842);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22238);
									match s {
										[] => true,
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ечать.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2088);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6511);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6512);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6513);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22239);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22240);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30349);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2089);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13843);
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
		['р', s @ ..] => {
			super::push(out, 2090);
			match s {
				[] => true,
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22241);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30350);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичать.
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 430);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2091);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6515);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6516);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13844);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22242);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2092);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6518);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13845);
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
				['г', s @ ..] => {
					super::push(out, 6519);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13846);
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
				['д', s @ ..] => {
					super::push(out, 6520);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13847);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30351);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30352);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36080);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76484(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 36081);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76485(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30353);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 30354);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6521);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13848);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22243);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22244);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30355);
											match s {
												[] => true,
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
									super::push(out, 22245);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22246);
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
				['з', s @ ..] => {
					super::push(out, 6522);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6523);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6524);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6526);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 6527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t12::n2203(s, out),
		['р', s @ ..] => {
			super::push(out, 2094);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13934);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13935);
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
							super::push(out, 13936);
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
							super::push(out, 13937);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 13938);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22432);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лчать.
pub(crate) fn n435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2095);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13939);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13940);
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
							super::push(out, 13941);
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
							super::push(out, 13942);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22433);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6548);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13943);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мчать.
pub(crate) fn n436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 432);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2096);
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2097);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6549);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13944);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2099);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчать.
pub(crate) fn n437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 433);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2100);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13945);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13946);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22434);
									match s {
										[] => true,
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
							super::push(out, 13947);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 13948);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 13949);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 13950);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22435);
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
							super::push(out, 13951);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13952);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 13953);
							match s {
								[] => true,
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
					super::push(out, 6552);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13954);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22436);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22437);
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
		['о', s @ ..] => {
			super::push(out, 2101);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6553);
					match s {
						[] => true,
						['з', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13955);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13956);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22438);
									match s {
										[] => true,
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
							super::push(out, 13957);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22439);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 22440);
									match s {
										[] => true,
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
							super::push(out, 13958);
							match s {
								[] => true,
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
					super::push(out, 6555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13959);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22441);
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
							super::push(out, 13960);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22442);
									match s {
										[] => true,
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
							super::push(out, 13961);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									super::push(out, 22443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 13962);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2102);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6556);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очать.
pub(crate) fn n438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 2103);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6557);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6558);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13963);
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
		['р', s @ ..] => {
			super::push(out, 2104);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 13964);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22444);
									match s {
										[] => true,
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
							super::push(out, 13965);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22445);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22446);
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
								['о', s @ ..] => {
									super::push(out, 22447);
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
			super::push(out, 2105);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6559);
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

/// Words ending -пчать.
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 435);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2106);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13966);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13967);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13968);
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

/// Words ending -рчать.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 436);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2107);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6560);
					match s {
						[] => true,
						['к', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2108);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6561);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13970);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13971);
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
			super::push(out, 2109);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13972);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13973);
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
						['к', s @ ..] => {
							super::push(out, 13974);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13975);
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
				['г', s @ ..] => {
					super::push(out, 6564);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 13976);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 22448);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30522);
											s.is_empty()
										}
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
					super::push(out, 6565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13977);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13978);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13979);
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
		['у', s @ ..] => {
			super::push(out, 2110);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6566);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6567);
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
							super::push(out, 13980);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13981);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13982);
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
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13983);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13984);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13985);
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
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13986);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13987);
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
				['о', s @ ..] => {
					super::push(out, 6568);
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
				['ф', s @ ..] => {
					super::push(out, 6569);
					match s {
						[] => true,
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
			super::push(out, 2111);
			match s {
				[] => true,
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13988);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 13989);
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
		['я', s @ ..] => {
			super::push(out, 2112);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6570);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13990);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -счать.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2113);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -учать.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2114);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2115);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2116);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6572);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
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
		['в', s @ ..] => {
			super::push(out, 2117);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13991);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13992);
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
		['д', s @ ..] => {
			super::push(out, 2118);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2119);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 13993);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22449);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2120);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6576);
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
			super::push(out, 2121);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6577);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13994);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22450);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30523);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22451);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 22452);
									match s {
										[] => true,
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
							super::push(out, 13995);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 13996);
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
		['л', s @ ..] => {
			super::push(out, 2122);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', s @ ..] => {
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6579);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22453);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 30524);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2123);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6580);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6581);
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
				['з', s @ ..] => {
					super::push(out, 6582);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							super::push(out, 13997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6583);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13999);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22454);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['о', s @ ..] => {
			super::push(out, 2124);
			match s {
				[] => true,
				['в', 'а', 'р', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t12::n2237(s, out),
		['с', s @ ..] => {
			super::push(out, 2126);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t12::n7824(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t12::n2239(s, out),
		_ => false,
	}
}

/// Words ending -ычать.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2128);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6588);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 14005);
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
		['м', s @ ..] => {
			super::push(out, 2129);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6589);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6590);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6591);
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
		['р', s @ ..] => {
			super::push(out, 2130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6592);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6593);
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
					super::push(out, 6594);
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

/// Words ending -ьчать.
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2131);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 14006);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22456);
									match s {
										[] => true,
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
							super::push(out, 14007);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22457);
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
							super::push(out, 14008);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22458);
									match s {
										[] => true,
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
							super::push(out, 14009);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючать.
pub(crate) fn n445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2132);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', s @ ..] => {
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

/// Words ending -ячать.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2133);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6595);
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
		_ => false,
	}
}

/// The word провозглашать.
pub(crate) fn n65781(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word самопровозглашать.
pub(crate) fn n65782(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word заприглашать.
pub(crate) fn n65783(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word понаприглашать.
pub(crate) fn n65784(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -развешать.
pub(crate) fn n18786(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -овешать.
pub(crate) fn n7848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6600);
	match s {
		[] => true,
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 14012);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word свешать.
pub(crate) fn n7849(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отвешать.
pub(crate) fn n7850(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -увешать.
pub(crate) fn n7851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6601);
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

/// Words ending -мешать.
pub(crate) fn n2253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2140);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6602);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6603);
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
			super::push(out, 6604);
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
			super::push(out, 6605);
			match s {
				[] => true,
				['р', 'е', 'п', s @ ..] => {
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
		['з', s @ ..] => {
			super::push(out, 6606);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6607);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 14013);
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
			super::push(out, 6608);
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
		['ы', s @ ..] => {
			super::push(out, 6609);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пешать.
pub(crate) fn n2254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2141);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 6610);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -решать.
pub(crate) fn n2255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2142);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 6611);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14015);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 14016);
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
			super::push(out, 6612);
			match s {
				[] => true,
				['е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 6613);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6614);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6615);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14017);
					match s {
						[] => true,
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
			super::push(out, 6616);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
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

/// Words ending -тешать.
pub(crate) fn n2256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2143);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6617);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ишать.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2144);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 2145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мшать.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2146);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6621);
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

/// Words ending -ошать.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2147);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14018);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14019);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22459);
									match s {
										[] => true,
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
							super::push(out, 14020);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 14021);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2148);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14022);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22460);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2149);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14023);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22461);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30526);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ршать.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2151);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6624);
					match s {
						[] => true,
						['о', s @ ..] => {
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
			super::push(out, 2152);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6625);
					match s {
						[] => true,
						['е', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22462);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22463);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30527);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6626);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14024);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14025);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14026);
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
						['у', s @ ..] => {
							super::push(out, 14027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тшать.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2153);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14028);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 14029);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30528);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36243);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76707(s, out),
														_ => false,
													}
												}
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
							super::push(out, 14030);
							match s {
								[] => true,
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30529);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36244);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76708(s, out),
														_ => false,
													}
												}
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
							super::push(out, 14031);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 22465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ушать.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2154);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2156);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6627);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6628);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 6629);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 14032);
							match s {
								[] => true,
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6631);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6632);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6633);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6634);
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
					super::push(out, 6635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14033);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 14034);
							match s {
								[] => true,
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
					super::push(out, 6636);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 14035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6637);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14036);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2157);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6638);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6639);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14037);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
							super::push(out, 14038);
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 14039);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22466);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2158);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2159);
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 2160);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6641);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2161);
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2162);
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

/// Words ending -чшать.
pub(crate) fn n456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 2163);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14040);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22467);
									match s {
										[] => true,
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
							super::push(out, 14041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышать.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2164);
			match s {
				[] => true,
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
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
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2165);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6642);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6643);
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
					super::push(out, 6644);
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
		['л', s @ ..] => {
			super::push(out, 2166);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6645);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14042);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14043);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14044);
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
							super::push(out, 14045);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 14046);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2167);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьшать.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 2168);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14047);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22468);
									match s {
										[] => true,
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
							super::push(out, 14048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22469);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 22470);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6646);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14049);
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

/// Words ending -повозвращать.
pub(crate) fn n65789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36245);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тращать.
pub(crate) fn n7932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6651);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22472);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22473);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 30532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 30533);
							match s {
								[] => true,
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
					super::push(out, 22474);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 30534);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 36246);
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
					super::push(out, 22475);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 30535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22476);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ещать.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2173);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14052);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14053);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14054);
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
			super::push(out, 2174);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6653);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14055);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 14056);
							match s {
								[] => true,
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
					super::push(out, 6654);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14057);
							match s {
								[] => true,
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
					super::push(out, 6655);
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6656);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14058);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14059);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6657);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14060);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22478);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22479);
									match s {
										[] => true,
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
							super::push(out, 14061);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22480);
									match s {
										[] => true,
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
							super::push(out, 14062);
							match s {
								[] => true,
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
					super::push(out, 6658);
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
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2175);
			match s {
				[] => true,
				['в', s @ ..] => {
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
				['з', 'а', 'р', s @ ..] => {
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
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'и', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6660);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14064);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22481);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22482);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30537);
											match s {
												[] => true,
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
									super::push(out, 22483);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30538);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30539);
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
				['п', s @ ..] => {
					super::push(out, 6661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14065);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14066);
							match s {
								[] => true,
								['з', s @ ..] => super::t12::n34308(s, out),
								['н', s @ ..] => super::t12::n34309(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t12::n18918(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t12::n2292(s, out),
		_ => false,
	}
}

/// Words ending -ищать.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 2178);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6663);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 6664);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14068);
							match s {
								[] => true,
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
					super::push(out, 6665);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6666);
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
			super::push(out, 2179);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6667);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6668);
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
		['х', s @ ..] => {
			super::push(out, 2180);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', s @ ..] => {
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
		['ч', s @ ..] => {
			super::push(out, 2181);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 2182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мщать.
pub(crate) fn n463(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -ощать.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 459);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 2183);
			s.is_empty()
		}
		['к', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 2184);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['о', 'в', s @ ..] => {
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
			super::push(out, 2185);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2186);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
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
								['о', s @ ..] => {
									super::push(out, 22486);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 30540);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6669);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6670);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14069);
							match s {
								[] => true,
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
					super::push(out, 6671);
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
					super::push(out, 6672);
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

/// The word обмирщать.
pub(crate) fn n465(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ущать.
pub(crate) fn n466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2188);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6673);
					match s {
						[] => true,
						['о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 14070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6674);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 14071);
							match s {
								[] => true,
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
				['с', '\u{1}', s @ ..] => {
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
		['щ', 'о', s @ ..] => {
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

/// Words ending -ыщать.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьщать.
pub(crate) fn n468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	s.is_empty()
}

/// Words ending -ящать.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 463);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2190);
			match s {
				[] => true,
				['с', 'о', s @ ..] => {
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
		['п', s @ ..] => {
			super::push(out, 2191);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6675);
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
		_ => false,
	}
}

/// The word изоржаветь.
pub(crate) fn n65798(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word закуржаветь.
pub(crate) fn n65799(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -еветь.
pub(crate) fn n478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 472);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2207);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6693);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14090);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22497);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22498);
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
						['ы', s @ ..] => {
							super::push(out, 14091);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22499);
									s.is_empty()
								}
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
			super::push(out, 2208);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6694);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14092);
							match s {
								[] => true,
								['р', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6695);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14093);
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
		['л', s @ ..] => {
			super::push(out, 2209);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6696);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6697);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2210);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6698);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14095);
							match s {
								[] => true,
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
					super::push(out, 6699);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14096);
							match s {
								[] => true,
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
					super::push(out, 6700);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14097);
							match s {
								[] => true,
								['л', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30544);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'р', 'п', '\u{1}', s @ ..] => {
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
				['ч', s @ ..] => {
					super::push(out, 6701);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14098);
							match s {
								[] => true,
								['р', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30545);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36247);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76711(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6702);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14099);
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
		['р', s @ ..] => {
			super::push(out, 2211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6703);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6704);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6705);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 14100);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22500);
									match s {
										[] => true,
										['ч', 'а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6706);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6707);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14101);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22501);
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
					super::push(out, 6708);
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
		['ц', s @ ..] => {
			super::push(out, 2212);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6709);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 14102);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22502);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 14103);
							match s {
								[] => true,
								['л', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30546);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36248);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76712(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2213);
			match s {
				[] => true,
				['е', s @ ..] => super::t12::n8037(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -езветь.
pub(crate) fn n2341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2214);
	match s {
		[] => true,
		['р', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14104);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 14105);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иветь.
pub(crate) fn n480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2215);
			match s {
				[] => true,
				['р', 'е', 'ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14106);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14107);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14108);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 14109);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 22505);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2216);
			match s {
				[] => true,
				['у', 'л', 'е', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14110);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14111);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14112);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2217);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6711);
					match s {
						[] => true,
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
			super::push(out, 2218);
			match s {
				[] => true,
				['п', 'о', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14113);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'с', 'а', 'ч', 'с', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2219);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6712);
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
			super::push(out, 2220);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14114);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14115);
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
		['с', s @ ..] => {
			super::push(out, 2221);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6714);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6715);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14116);
							match s {
								[] => true,
								['к', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6716);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14117);
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
					super::push(out, 6717);
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
			super::push(out, 2222);
			match s {
				[] => true,
				['о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14118);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14119);
							match s {
								[] => true,
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
					super::push(out, 6718);
					match s {
						[] => true,
						['о', 'р', 'т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2223);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14120);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
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
								['б', s @ ..] => {
									super::push(out, 22506);
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
				['е', 'л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14121);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14122);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14123);
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
				['р', 'а', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14124);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14125);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14126);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олветь.
pub(crate) fn n2351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2224);
	match s {
		[] => true,
		['м', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оветь.
pub(crate) fn n482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 2225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6720);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6721);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14127);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22507);
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
						['п', s @ ..] => {
							super::push(out, 14128);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22508);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22509);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 22510);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6722);
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
			super::push(out, 2226);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14129);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14130);
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
		['з', s @ ..] => {
			super::push(out, 2227);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6723);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14131);
							match s {
								[] => true,
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
					super::push(out, 6724);
					match s {
						[] => true,
						['о', 'р', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22511);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22512);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30548);
											match s {
												[] => true,
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
									super::push(out, 22513);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30549);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6725);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22514);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22515);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22516);
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
		['к', s @ ..] => {
			super::push(out, 2228);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6726);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14132);
							match s {
								[] => true,
								['р', 'п', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6727);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14133);
							match s {
								[] => true,
								['т', 'с', 'е', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30550);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30551);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2229);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6728);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22517);
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
				['о', s @ ..] => {
					super::push(out, 6729);
					match s {
						[] => true,
						['п', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22518);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22519);
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
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 6730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14134);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14135);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t12::n2357(s, out),
		['р', s @ ..] => super::t12::n2358(s, out),
		['с', s @ ..] => super::t12::n2359(s, out),
		['х', s @ ..] => {
			super::push(out, 2233);
			match s {
				[] => true,
				['о', 'м', s @ ..] => super::t12::n8081(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t12::n2361(s, out),
		_ => false,
	}
}

/// Words ending -тветь.
pub(crate) fn n483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2235);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14148);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14149);
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
		['с', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6743);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14150);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 22526);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 22527);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30560);
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
				['р', s @ ..] => {
					super::push(out, 6744);
					match s {
						[] => true,
						['е', 'ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22528);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22529);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яветь.
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 478);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['е', 'п', 'е', 'ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14151);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', 'у', 'р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14152);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
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
		['н', s @ ..] => {
			super::push(out, 2237);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 6745);
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
		['р', s @ ..] => {
			super::push(out, 2238);
			match s {
				[] => true,
				['ы', 'д', 'з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здеть.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2250);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6761);
					match s {
						[] => true,
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
			super::push(out, 2251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6762);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6763);
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
					super::push(out, 6764);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6765);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14163);
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
			super::push(out, 2252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2253);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6767);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 6768);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14164);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2254);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14165);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14166);
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
							super::push(out, 14167);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 22531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 14168);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14169);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22532);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 22533);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30561);
											match s {
												[] => true,
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
									super::push(out, 22534);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30562);
											match s {
												[] => true,
												['а', 'н', '\u{1}', s @ ..] => {
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
						['с', s @ ..] => {
							super::push(out, 14170);
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
		['о', s @ ..] => {
			super::push(out, 2255);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идеть.
pub(crate) fn n493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 487);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2256);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6771);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14171);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 22535);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30563);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
			super::push(out, 2257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6772);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6773);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14172);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 14173);
							match s {
								[] => true,
								['е', 'н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30564);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 30565);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 36249);
													match s {
														[] => true,
														['в', '\u{1}', s @ ..] => super::t12::n76713(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 6774);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14174);
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
					super::push(out, 6775);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 6776);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14175);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 14176);
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
				['о', s @ ..] => {
					super::push(out, 6777);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14177);
							match s {
								[] => true,
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 14178);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['я', s @ ..] => {
											super::push(out, 30566);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 14179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14180);
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
					super::push(out, 6778);
					match s {
						[] => true,
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
			super::push(out, 2258);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6779);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6780);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14181);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2259);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6781);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6782);
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
					super::push(out, 6783);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6784);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6785);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 14182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6786);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14183);
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
							super::push(out, 14184);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 14185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6787);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14186);
							match s {
								[] => true,
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
					super::push(out, 6788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14187);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 14188);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22536);
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
				['ы', s @ ..] => {
					super::push(out, 6789);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14189);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обалдеть.
pub(crate) fn n19146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14191);
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

/// Words ending -галдеть.
pub(crate) fn n8146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6791);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14192);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14193);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22537);
					match s {
						[] => true,
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
			super::push(out, 14194);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндеть.
pub(crate) fn n495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14195);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 22539);
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
		['е', s @ ..] => {
			super::push(out, 2262);
			match s {
				[] => true,
				['р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2263);
			match s {
				[] => true,
				['г', '\u{1}', s @ ..] => {
					super::push(out, 6792);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 6793);
					match s {
						[] => true,
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
			super::push(out, 2264);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6794);
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

/// Words ending -одеть.
pub(crate) fn n496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2265);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 2266);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6795);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2267);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6796);
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
			super::push(out, 2268);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6797);
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
			super::push(out, 2269);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6798);
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
			super::push(out, 2270);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6799);
					match s {
						[] => true,
						['п', s @ ..] => {
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
			super::push(out, 2271);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6800);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14197);
							s.is_empty()
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22540);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22541);
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22542);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
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
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22543);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22544);
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
									super::push(out, 22545);
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
				['п', s @ ..] => {
					super::push(out, 6801);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14198);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22546);
									s.is_empty()
								}
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
			super::push(out, 2272);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6802);
					match s {
						[] => true,
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
			super::push(out, 2273);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 6804);
					match s {
						[] => true,
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
			super::push(out, 2274);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6805);
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

/// Words ending -рдеть.
pub(crate) fn n497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 491);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2275);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2276);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6806);
					match s {
						[] => true,
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
			super::push(out, 2277);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6807);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22547);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22548);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 22549);
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
				['м', s @ ..] => {
					super::push(out, 6808);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22550);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22551);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30567);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30568);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6809);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14201);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14202);
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
							super::push(out, 14203);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22552);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 22553);
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
							super::push(out, 14204);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22554);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2278);
			match s {
				[] => true,
				['г', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2279);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6810);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сдеть.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 492);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тдеть.
pub(crate) fn n499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2280);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удеть.
pub(crate) fn n500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 494);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 2281);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6811);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6812);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14205);
							match s {
								[] => true,
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
					super::push(out, 6813);
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
					super::push(out, 6814);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2282);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6815);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6816);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14207);
							match s {
								[] => true,
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
					super::push(out, 6817);
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
					super::push(out, 6818);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14208);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2283);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 2284);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14209);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14210);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22555);
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
							super::push(out, 14211);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6820);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6821);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14212);
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
		['н', s @ ..] => {
			super::push(out, 2286);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6822);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6823);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2287);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6824);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6825);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14214);
							match s {
								[] => true,
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
					super::push(out, 6826);
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

/// Words ending -ьдеть.
pub(crate) fn n501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 495);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2288);
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

/// Words ending -юдеть.
pub(crate) fn n502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2289);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6827);
					match s {
						[] => true,
						['е', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22556);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22557);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядеть.
pub(crate) fn n503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 497);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2290);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14215);
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
						['з', s @ ..] => {
							super::push(out, 14216);
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
							super::push(out, 14217);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 14218);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22558);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30569);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word посожалеть.
pub(crate) fn n65821(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -елеть.
pub(crate) fn n519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 512);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2308);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6862);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6863);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 6864);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14237);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22562);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6865);
					match s {
						[] => true,
						['е', 'д', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6866);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14238);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2309);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6867);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6868);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2310);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 6869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2311);
			match s {
				[] => true,
				['я', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14240);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14241);
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
		['м', s @ ..] => {
			super::push(out, 2312);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6870);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6871);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14242);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6872);
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
					super::push(out, 6873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14243);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14244);
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
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14245);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14246);
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
						['о', s @ ..] => {
							super::push(out, 14247);
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
		['с', s @ ..] => {
			super::push(out, 2313);
			match s {
				[] => true,
				['е', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14248);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14249);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22564);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2314);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -силеть.
pub(crate) fn n2459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2318);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 6876);
			match s {
				[] => true,
				['е', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 22565);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 22566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тилеть.
pub(crate) fn n2460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2319);
	match s {
		[] => true,
		['ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14251);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 14252);
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

/// Words ending -хилеть.
pub(crate) fn n2461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2320);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6877);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6878);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6879);
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

/// Words ending -клеть.
pub(crate) fn n522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 515);
	match s {
		[] => true,
		['е', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6880);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6881);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -млеть.
pub(crate) fn n523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 516);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2321);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2322);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 6882);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14253);
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
			super::push(out, 2323);
			match s {
				[] => true,
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олеть.
pub(crate) fn n524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 517);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6883);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6884);
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
				['е', s @ ..] => {
					super::push(out, 6885);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 6886);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6887);
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
				['т', s @ ..] => {
					super::push(out, 6888);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 6889);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14254);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2325);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6890);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2326);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2327);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6891);
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
		['з', s @ ..] => {
			super::push(out, 2328);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6892);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 14256);
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
		['к', s @ ..] => {
			super::push(out, 2329);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6893);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6894);
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
		['с', s @ ..] => {
			super::push(out, 2330);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6895);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14257);
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

/// Words ending -плеть.
pub(crate) fn n525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 518);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6896);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6897);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6898);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14258);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14259);
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
					super::push(out, 6899);
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

/// Words ending -слеть.
pub(crate) fn n526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 519);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2331);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2332);
			match s {
				[] => true,
				['р', 'з', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14260);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14261);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22567);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2333);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6900);
					match s {
						[] => true,
						['с', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлеть.
pub(crate) fn n527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 520);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2334);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2335);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 6901);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 6902);
					match s {
						[] => true,
						['е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22568);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22569);
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
									super::push(out, 22570);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30571);
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
		['е', s @ ..] => {
			super::push(out, 2336);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14262);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14263);
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
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2337);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			super::push(out, 2338);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6903);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14264);
							match s {
								[] => true,
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
					super::push(out, 6904);
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
		['ы', s @ ..] => {
			super::push(out, 2339);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлеть.
pub(crate) fn n528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 521);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 2340);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6906);
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
		['ы', s @ ..] => {
			super::push(out, 2341);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
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
		['я', 'р', 'д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6907);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6908);
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

/// Words ending -шлеть.
pub(crate) fn n529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 522);
	match s {
		[] => true,
		['о', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6909);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6910);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылеть.
pub(crate) fn n530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 523);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 2342);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 6911);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22571);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22572);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word недоразуметь.
pub(crate) fn n65822(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выразуметь.
pub(crate) fn n65823(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -суметь.
pub(crate) fn n2502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2356);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чуметь.
pub(crate) fn n2503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2357);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6923);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шуметь.
pub(crate) fn n2504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2358);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6925);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6926);
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
		['о', s @ ..] => {
			super::push(out, 6927);
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
			super::push(out, 6928);
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

/// Words ending -ыметь.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 530);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 2359);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2360);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 6930);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юметь.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 531);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 2361);
			match s {
				[] => true,
				['г', 'у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14273);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14274);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 22579);
									match s {
										[] => true,
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
							super::push(out, 14275);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 22580);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енеть.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2369);
			match s {
				[] => true,
				['л', 'о', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14292);
							match s {
								[] => true,
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
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22585);
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22586);
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
		['в', s @ ..] => {
			super::push(out, 2370);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6943);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22587);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22588);
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
					super::push(out, 6944);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14293);
							match s {
								[] => true,
								['е', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30574);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30575);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30576);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14294);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14295);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 14296);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 22589);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 14297);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14298);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 22590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							super::push(out, 14299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6946);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14300);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30577);
											s.is_empty()
										}
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
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6947);
					match s {
						[] => true,
						['е', 'т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22591);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22592);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6948);
					match s {
						[] => true,
						['с', 'и', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22593);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 22594);
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
			super::push(out, 2371);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6949);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22595);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22596);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 22597);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => super::t12::n34602(s, out),
								['о', s @ ..] => super::t12::n34603(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t12::n8356(s, out),
				['о', s @ ..] => super::t12::n8357(s, out),
				['у', s @ ..] => super::t12::n8358(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t12::n2520(s, out),
		['м', s @ ..] => super::t12::n2521(s, out),
		['п', s @ ..] => super::t12::n2522(s, out),
		['р', s @ ..] => super::t12::n2523(s, out),
		['с', s @ ..] => super::t12::n2524(s, out),
		['т', s @ ..] => super::t12::n2525(s, out),
		['ф', s @ ..] => super::t12::n2526(s, out),
		['ч', s @ ..] => super::t12::n2527(s, out),
		_ => false,
	}
}

/// Words ending -нежнеть.
pub(crate) fn n8375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6960);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -знеть.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 536);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2381);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 6961);
					match s {
						[] => true,
						['р', 'е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22610);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22611);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30580);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2382);
			match s {
				[] => true,
				['р', 'г', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2383);
			match s {
				[] => true,
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14323);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14324);
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
		['я', s @ ..] => {
			super::push(out, 2384);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6962);
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
		_ => false,
	}
}

/// Words ending -инеть.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 537);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2385);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6963);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6964);
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
		['т', s @ ..] => {
			super::push(out, 2386);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6965);
					match s {
						[] => true,
						['к', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стройнеть.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2387);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2388);
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

/// Words ending -кнеть.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 538);
	match s {
		[] => true,
		['с', 'у', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6966);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6967);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14325);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лнеть.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['о', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6968);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6969);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6970);
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

/// Words ending -мнеть.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6971);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6972);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6973);
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
					super::push(out, 6974);
					match s {
						[] => true,
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
			super::push(out, 2389);
			match s {
				[] => true,
				['р', 'к', 'с', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6975);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6976);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 14326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -длиннеть.
pub(crate) fn n549(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2391);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2392);
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

/// Words ending -онеть.
pub(crate) fn n550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2393);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6977);
					match s {
						[] => true,
						['е', 'ч', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2394);
			match s {
				[] => true,
				['о', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14327);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14328);
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
			super::push(out, 2395);
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -крупнеть.
pub(crate) fn n551(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2396);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2397);
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

/// Words ending -рнеть.
pub(crate) fn n552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2398);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6978);
					match s {
						[] => true,
						['к', 'с', '\u{1}', s @ ..] => {
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
						['а', s @ ..] => {
							super::push(out, 14329);
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14330);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14331);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14332);
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
		['и', s @ ..] => {
			super::push(out, 2399);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14333);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14334);
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
				['м', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14335);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14336);
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
		['о', s @ ..] => {
			super::push(out, 2400);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6979);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14337);
							match s {
								[] => true,
								['о', 'р', 'п', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2401);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 6980);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14338);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14339);
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
				['м', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 14340);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22612);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22613);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30581);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36251);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 36252);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t12::n76715(s, out),
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
									super::push(out, 22614);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30582);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 14341);
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

/// Words ending -снеть.
pub(crate) fn n553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2402);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14342);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14343);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14344);
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2403);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6981);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6982);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22615);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22616);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30583);
											match s {
												[] => true,
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
									super::push(out, 22617);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 30584);
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
									super::push(out, 22618);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30585);
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
		['о', s @ ..] => {
			super::push(out, 2404);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14345);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14346);
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
		['я', s @ ..] => {
			super::push(out, 2405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6983);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6984);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14347);
							match s {
								[] => true,
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
					super::push(out, 6985);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 14348);
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 14349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 6986);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14350);
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
				['ы', s @ ..] => {
					super::push(out, 6987);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14351);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнеть.
pub(crate) fn n554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 544);
	match s {
		[] => true,
		['о', 'л', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6988);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6989);
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
		['с', 'у', 'р', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6990);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6991);
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
		['у', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6992);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6993);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6994);
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

/// Words ending -унеть.
pub(crate) fn n555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2406);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14352);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14353);
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
		['г', s @ ..] => {
			super::push(out, 2407);
			match s {
				[] => true,
				['у', 'ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14354);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14355);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14356);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2408);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 6995);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 14357);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2409);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6996);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14358);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 2410);
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

/// Words ending -чнеть.
pub(crate) fn n556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 546);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2411);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14359);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 22619);
									match s {
										[] => true,
										['р', 'п', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 14360);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22620);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22621);
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
								['у', s @ ..] => {
									super::push(out, 22622);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 30586);
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
				_ => false,
			}
		}
		['о', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6997);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6998);
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
			super::push(out, 2412);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14361);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14362);
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
					super::push(out, 6999);
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
							super::push(out, 14363);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 14364);
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
						['у', s @ ..] => {
							super::push(out, 14365);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шнеть.
pub(crate) fn n557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 547);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 2413);
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

/// Words ending -щнеть.
pub(crate) fn n558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 548);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2414);
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

/// Words ending -ьнеть.
pub(crate) fn n559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 549);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2415);
			match s {
				[] => true,
				['и', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юнеть.
pub(crate) fn n560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 550);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2416);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2417);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янеть.
pub(crate) fn n561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 551);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2418);
			match s {
				[] => true,
				['а', 'р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2419);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7001);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22623);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7002);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 22624);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30587);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'к', 'е', 'т', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 7003);
					match s {
						[] => true,
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
			super::push(out, 2420);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 7004);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14366);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22625);
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
			super::push(out, 2421);
			match s {
				[] => true,
				['г', 'а', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14367);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14368);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14369);
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
		['ф', s @ ..] => {
			super::push(out, 2422);
			match s {
				[] => true,
				['р', 'о', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2423);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14370);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14371);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14372);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оскрипеть.
pub(crate) fn n34690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22631);
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

/// Words ending -припеть.
pub(crate) fn n8467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7014);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хрипеть.
pub(crate) fn n8468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7015);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14379);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 14380);
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

/// Words ending -сипеть.
pub(crate) fn n2589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2432);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7016);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7017);
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
		_ => false,
	}
}

/// Words ending -шипеть.
pub(crate) fn n2590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2433);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7018);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 7019);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7020);
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

/// Words ending -лпеть.
pub(crate) fn n568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 558);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2434);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7021);
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

/// Words ending -опеть.
pub(crate) fn n569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2435);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2436);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2438);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7022);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 14381);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 22632);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 22633);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 7023);
					match s {
						[] => true,
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
			super::push(out, 2439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7024);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7025);
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
					super::push(out, 7026);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14382);
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
					super::push(out, 7027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14383);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 14384);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14385);
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

/// Words ending -рпеть.
pub(crate) fn n570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2440);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7028);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14386);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 14387);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 30588);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 30589);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 36253);
													s.is_empty()
												}
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
							super::push(out, 14388);
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
							super::push(out, 14389);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 14390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 14391);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22634);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2441);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14392);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 14393);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14394);
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
							super::push(out, 14395);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 22635);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спеть.
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2442);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2443);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7031);
					match s {
						[] => true,
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
			super::push(out, 2444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2445);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2446);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2447);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7032);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7033);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7034);
					match s {
						[] => true,
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
			super::push(out, 2448);
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
		['ы', s @ ..] => {
			super::push(out, 2449);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тпеть.
pub(crate) fn n572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 562);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2450);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

