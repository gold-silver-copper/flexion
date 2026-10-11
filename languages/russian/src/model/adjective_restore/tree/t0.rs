use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['в', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14);
							match s {
								[] => true,
								['а', 'ц', s @ ..] => {
									super::push(out, 44);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 45);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 16);
							match s {
								[] => true,
								['о', 'д', s @ ..] => {
									super::push(out, 46);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 183);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 184);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 17);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 47);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 48);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t0::n2(s, out),
		['к', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11);
					match s {
						[] => true,
						['н', 'е', 'т', s @ ..] => {
							super::push(out, 42);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4);
			s.is_empty()
		}
		['я', s @ ..] => super::t0::n5(s, out),
		_ => false,
	}
}

/// Words ending -й.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['и', s @ ..] => super::t0::n8(s, out),
		['о', s @ ..] => {
			super::push(out, 9);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27);
					match s {
						[] => true,
						['у', 'л', 'о', 'г', '-', s @ ..] => {
							super::push(out, 95);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 628);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 1321);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 1322);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 629);
											s.is_empty()
										}
										['т', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 1323);
													s.is_empty()
												}
												['л', s @ ..] => {
													super::push(out, 1324);
													s.is_empty()
												}
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
							super::push(out, 96);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 28);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 97);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 283);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 284);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 630);
											match s {
												[] => true,
												['р', 'е', 'т', 'с', 'х', s @ ..] => {
													super::push(out, 1325);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 285);
									match s {
										[] => true,
										['у', 'л', s @ ..] => {
											super::push(out, 631);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 1326);
													s.is_empty()
												}
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
							super::push(out, 98);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 286);
									match s {
										[] => true,
										['о', 'г', s @ ..] => {
											super::push(out, 632);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 1327);
													match s {
														[] => true,
														['е', s @ ..] => super::t0::n2388(s, out),
														['у', s @ ..] => super::t0::n2389(s, out),
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
									super::push(out, 287);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 633);
											match s {
												[] => true,
												['ч', s @ ..] => {
													match s {
														[] => true,
														['о', s @ ..] => super::t0::n2390(s, out),
														['х', s @ ..] => super::t0::n2391(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 634);
											s.is_empty()
										}
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
					super::push(out, 29);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 99);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 288);
									match s {
										[] => true,
										['т', 'с', 'е', s @ ..] => {
											super::push(out, 635);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 100);
							match s {
								[] => true,
								['е', 'т', s @ ..] => {
									super::push(out, 289);
									match s {
										[] => true,
										['с', 'о', 'с', 'е', 'л', '-', s @ ..] => {
											super::push(out, 636);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 101);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 290);
									match s {
										[] => true,
										['р', '-', s @ ..] => {
											super::push(out, 637);
											match s {
												[] => true,
												['о', 'н', s @ ..] => {
													match s {
														[] => true,
														['р', s @ ..] => super::t0::n2392(s, out),
														['с', s @ ..] => super::t0::n2393(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t0::n10(s, out),
		_ => false,
	}
}

/// Words ending -я.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['й', s @ ..] => super::t0::n43(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ий.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 503);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'х', s @ ..] => {
							super::push(out, 185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 19);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 186);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 50);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['р', '-', s @ ..] => {
							super::push(out, 188);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t0::n20(s, out),
		['н', s @ ..] => {
			super::push(out, 21);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 67);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 68);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 69);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 70);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 227);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 228);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 229);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 540);
									match s {
										[] => true,
										['о', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 2142);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 2143);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 71);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 230);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 231);
							match s {
								[] => true,
								['р', 'о', 'т', 'с', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 1193);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 1194);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 1195);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 2144);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 2145);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 72);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 541);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 542);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 543);
									match s {
										[] => true,
										['о', 'с', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 2146);
													s.is_empty()
												}
												['х', s @ ..] => {
													super::push(out, 2147);
													match s {
														[] => true,
														['е', s @ ..] => super::t0::n3279(s, out),
														['у', s @ ..] => super::t0::n3280(s, out),
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
									super::push(out, 544);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 1196);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 1197);
											s.is_empty()
										}
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
					super::push(out, 73);
					match s {
						[] => true,
						['е', 'е', s @ ..] => {
							super::push(out, 232);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 233);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 545);
									s.is_empty()
								}
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
			super::push(out, 22);
			match s {
				[] => true,
				['а', 'к', '-', s @ ..] => {
					super::push(out, 74);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 75);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 76);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 77);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 24);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78);
					match s {
						[] => true,
						['н', 'м', s @ ..] => {
							super::push(out, 234);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 546);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t0::n25(s, out),
		['щ', s @ ..] => super::t0::n26(s, out),
		_ => false,
	}
}

/// Words ending -ехгодовой.
pub(crate) fn n2388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2264);
	s.is_empty()
}

/// Words ending -ухгодовой.
pub(crate) fn n2389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2265);
	s.is_empty()
}

/// Words ending -очасовой.
pub(crate) fn n2390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2266);
	s.is_empty()
}

/// Words ending -хчасовой.
pub(crate) fn n2391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2267);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3243);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4239);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3244);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рно-речной.
pub(crate) fn n2392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2268);
	s.is_empty()
}

/// Words ending -сно-речной.
pub(crate) fn n2393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2269);
	s.is_empty()
}

/// Words ending -ый.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 30);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 103);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 104);
					match s {
						[] => true,
						['з', 'х', s @ ..] => {
							super::push(out, 291);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 638);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t0::n31(s, out),
		['д', s @ ..] => {
			super::push(out, 32);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 112);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 113);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 114);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 326);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 723);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 116);
					match s {
						[] => true,
						['л', 'г', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 725);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 726);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 1391);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 1392);
											s.is_empty()
										}
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
					super::push(out, 117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 118);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 328);
							match s {
								[] => true,
								['о', 'д', 'о', 'г', 'х', s @ ..] => {
									super::push(out, 727);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 1393);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 1394);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 329);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 728);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 119);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 120);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 121);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 330);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 730);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 1395);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 2308);
													match s {
														[] => true,
														['е', s @ ..] => super::t0::n3486(s, out),
														['р', s @ ..] => super::t0::n3487(s, out),
														['с', s @ ..] => super::t0::n3488(s, out),
														['ч', s @ ..] => super::t0::n3489(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 2309);
													match s {
														[] => true,
														['л', 'е', 'ж', '-', s @ ..] => super::t0::n3490(s, out),
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
									super::push(out, 731);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', 'к', s @ ..] => {
							super::push(out, 331);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 332);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 333);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 334);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 335);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 336);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 337);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 733);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 734);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 122);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 339);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 340);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 123);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 124);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 342);
							match s {
								[] => true,
								['л', 'б', '\u{1}', s @ ..] => {
									super::push(out, 736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 343);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 125);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 126);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 345);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 127);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 128);
					s.is_empty()
				}
				['ш', s @ ..] => super::t0::n130(s, out),
				['ы', s @ ..] => super::t0::n131(s, out),
				['я', s @ ..] => super::t0::n132(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t0::n35(s, out),
		['н', s @ ..] => super::t0::n36(s, out),
		['п', s @ ..] => super::t0::n37(s, out),
		['р', s @ ..] => super::t0::n38(s, out),
		['с', s @ ..] => super::t0::n39(s, out),
		['т', s @ ..] => super::t0::n40(s, out),
		['ц', s @ ..] => super::t0::n41(s, out),
		_ => false,
	}
}

/// Words ending -йся.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['ш', s @ ..] => super::t0::n518(s, out),
				['щ', s @ ..] => super::t0::n519(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кий.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 191);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['а', 'л', s @ ..] => {
					super::push(out, 193);
					match s {
						[] => true,
						['с', '-', s @ ..] => {
							super::push(out, 505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 194);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 196);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 57);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 58);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['о', 'т', 'с', 'о', 'к', s @ ..] => {
					super::push(out, 198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 199);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 61);
			s.is_empty()
		}
		['с', s @ ..] => super::t0::n63(s, out),
		['т', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 216);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 217);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 533);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 534);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 218);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 219);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 220);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 221);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 223);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 536);
							match s {
								[] => true,
								['т', 'о', 'к', s @ ..] => {
									super::push(out, 1182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 224);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 225);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 537);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 1183);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 1184);
									s.is_empty()
								}
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
			super::push(out, 66);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 538);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 2119);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 3064);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 3065);
													s.is_empty()
												}
												['ш', s @ ..] => {
													super::push(out, 3066);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 2120);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 2121);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 1185);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 1186);
									s.is_empty()
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 2122);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 2123);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 3067);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 2124);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 3068);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 3069);
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
										['д', s @ ..] => {
											super::push(out, 2125);
											s.is_empty()
										}
										['е', s @ ..] => super::t0::n2239(s, out),
										['н', s @ ..] => super::t0::n2240(s, out),
										['р', 'е', s @ ..] => super::t0::n2241(s, out),
										['т', s @ ..] => super::t0::n2242(s, out),
										_ => false,
									}
								}
								['р', 'т', 'с', 'е', s @ ..] => super::t0::n1244(s, out),
								['с', s @ ..] => super::t0::n1245(s, out),
								['т', s @ ..] => super::t0::n1246(s, out),
								['ш', s @ ..] => super::t0::n1247(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t0::n557(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t0::n229(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ехсотлетний.
pub(crate) fn n3279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3072);
	s.is_empty()
}

/// Words ending -ухсотлетний.
pub(crate) fn n3280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3073);
	s.is_empty()
}

/// Words ending -ший.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['р', 'г', 'ы', s @ ..] => {
							super::push(out, 548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t0::n81(s, out),
		['г', s @ ..] => {
			super::push(out, 81);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 246);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 582);
							s.is_empty()
						}
						['ж', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 1268);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 1269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 583);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 1270);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 1271);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 247);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 248);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 249);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 250);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 82);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 252);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 584);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 1272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 253);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 254);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 255);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 256);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 258);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 586);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 1273);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 1274);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 1275);
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 3180);
													s.is_empty()
												}
												['ь', s @ ..] => {
													super::push(out, 3181);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 1276);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 2222);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 2223);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 3182);
													s.is_empty()
												}
												['ж', 'я', s @ ..] => {
													super::push(out, 3183);
													s.is_empty()
												}
												['л', s @ ..] => {
													super::push(out, 3184);
													match s {
														[] => true,
														['с', s @ ..] => super::t0::n4140(s, out),
														_ => false,
													}
												}
												['м', s @ ..] => {
													super::push(out, 3185);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 3186);
													s.is_empty()
												}
												['р', 'а', s @ ..] => {
													super::push(out, 3187);
													s.is_empty()
												}
												['ц', s @ ..] => {
													super::push(out, 3188);
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 2224);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 1277);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 1278);
									s.is_empty()
								}
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
			super::push(out, 85);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 259);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 587);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 1279);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 1280);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 1281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t0::n612(s, out),
						['с', s @ ..] => {
							super::push(out, 589);
							s.is_empty()
						}
						['т', s @ ..] => super::t0::n614(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 260);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 262);
					s.is_empty()
				}
				['р', s @ ..] => super::t0::n268(s, out),
				['ы', s @ ..] => super::t0::n269(s, out),
				['я', s @ ..] => super::t0::n270(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t0::n87(s, out),
		['р', s @ ..] => super::t0::n88(s, out),
		['с', s @ ..] => super::t0::n89(s, out),
		['т', s @ ..] => super::t0::n90(s, out),
		['х', s @ ..] => super::t0::n91(s, out),
		_ => false,
	}
}

/// Words ending -щий.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 91);
			match s {
				[] => true,
				['ж', 'е', s @ ..] => {
					super::push(out, 275);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 601);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 92);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 276);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 277);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 603);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 1300);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 1301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 604);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 1302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['а', s @ ..] => super::t0::n284(s, out),
				['е', s @ ..] => {
					super::push(out, 279);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 624);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 1318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 280);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 281);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 625);
							match s {
								[] => true,
								['т', 'с', s @ ..] => super::t0::n1385(s, out),
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 626);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 1319);
									match s {
										[] => true,
										['р', 'б', 'о', 'о', 'к', s @ ..] => super::t0::n2387(s, out),
										_ => false,
									}
								}
								['и', s @ ..] => super::t0::n1387(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t0::n651(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t0::n288(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t0::n95(s, out),
		_ => false,
	}
}

/// Words ending -вый.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 292);
					match s {
						[] => true,
						['г', 'х', s @ ..] => {
							super::push(out, 640);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 1328);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 1329);
									s.is_empty()
								}
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
			super::push(out, 106);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 293);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 294);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 641);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 642);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 643);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 295);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 645);
							match s {
								[] => true,
								['а', 'р', 'о', '-', s @ ..] => {
									super::push(out, 1330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 298);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 646);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 647);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 1331);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 1332);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 3245);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 3246);
													match s {
														[] => true,
														['о', 'с', s @ ..] => super::t0::n4166(s, out),
														_ => false,
													}
												}
												['х', s @ ..] => {
													super::push(out, 3247);
													match s {
														[] => true,
														['е', s @ ..] => super::t0::n4167(s, out),
														['у', s @ ..] => super::t0::n4168(s, out),
														_ => false,
													}
												}
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
							super::push(out, 648);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 649);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 650);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 651);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 1333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', 'е', s @ ..] => {
							super::push(out, 652);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 653);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 1334);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 654);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 1335);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 1336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 655);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 1337);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 1338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 656);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t0::n306(s, out),
				['о', s @ ..] => {
					super::push(out, 300);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 665);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 667);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 668);
							s.is_empty()
						}
						['о', s @ ..] => super::t0::n695(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t0::n310(s, out),
				['у', s @ ..] => super::t0::n311(s, out),
				['ц', s @ ..] => super::t0::n312(s, out),
				['ч', s @ ..] => super::t0::n313(s, out),
				['ш', s @ ..] => super::t0::n314(s, out),
				['щ', s @ ..] => super::t0::n315(s, out),
				['ь', s @ ..] => super::t0::n316(s, out),
				_ => false,
			}
		}
		['з', 'е', 'р', 'е', s @ ..] => super::t0::n108(s, out),
		['и', s @ ..] => super::t0::n109(s, out),
		['о', s @ ..] => super::t0::n110(s, out),
		['т', s @ ..] => super::t0::n111(s, out),
		['я', s @ ..] => super::t0::n112(s, out),
		_ => false,
	}
}

/// Words ending -ено-белый.
pub(crate) fn n3486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3273);
	s.is_empty()
}

/// Words ending -рно-белый.
pub(crate) fn n3487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3274);
	s.is_empty()
}

/// Words ending -сно-белый.
pub(crate) fn n3488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3275);
	s.is_empty()
}

/// Words ending -чно-белый.
pub(crate) fn n3489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3276);
	s.is_empty()
}

/// Words ending --желто-белый.
pub(crate) fn n3490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3277);
	s.is_empty()
}

/// Words ending -шлый.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	s.is_empty()
}

/// Words ending -ылый.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['р', 'к', 'х', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ялый.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	s.is_empty()
}

/// Words ending -мый.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 739);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 1396);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 1397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => super::t0::n767(s, out),
						['г', s @ ..] => {
							super::push(out, 741);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 1402);
									match s {
										[] => true,
										['е', s @ ..] => super::t0::n2462(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => super::t0::n769(s, out),
						['ж', s @ ..] => super::t0::n770(s, out),
						['з', s @ ..] => super::t0::n771(s, out),
						['к', s @ ..] => super::t0::n772(s, out),
						['л', s @ ..] => super::t0::n773(s, out),
						['м', s @ ..] => super::t0::n774(s, out),
						['н', s @ ..] => super::t0::n775(s, out),
						['п', s @ ..] => super::t0::n776(s, out),
						['р', s @ ..] => super::t0::n777(s, out),
						['т', s @ ..] => super::t0::n778(s, out),
						['ц', s @ ..] => super::t0::n779(s, out),
						['ч', s @ ..] => super::t0::n780(s, out),
						['ш', s @ ..] => super::t0::n781(s, out),
						['щ', s @ ..] => super::t0::n782(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t0::n358(s, out),
				['я', s @ ..] => super::t0::n359(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t0::n134(s, out),
		['о', s @ ..] => super::t0::n135(s, out),
		_ => false,
	}
}

/// Words ending -ный.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 352);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							super::push(out, 757);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 353);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 759);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 354);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 761);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 762);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 763);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 355);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 356);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 357);
					match s {
						[] => true,
						['т', 'о', 'к', s @ ..] => {
							super::push(out, 765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 358);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 766);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 359);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 768);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 360);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 362);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 363);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 770);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 771);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 772);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 773);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 1412);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 774);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 776);
							match s {
								[] => true,
								['о', 'п', 'о', 'б', s @ ..] => {
									super::push(out, 1413);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 2333);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 777);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 778);
							match s {
								[] => true,
								['о', 'п', 'с', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 3312);
													s.is_empty()
												}
												['ж', s @ ..] => {
													super::push(out, 3313);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 3314);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 3315);
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 3316);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 365);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 779);
							match s {
								[] => true,
								['а', 'п', 'х', s @ ..] => {
									super::push(out, 1414);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 2334);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 2335);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 366);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 781);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 782);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 2336);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 2337);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 2338);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 2339);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 3317);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 3318);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 368);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 783);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									super::push(out, 1415);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 2340);
											match s {
												[] => true,
												['с', 'и', 'н', 'и', 'м', 'д', 'а', '-', s @ ..] => {
													super::push(out, 3319);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t0::n139(s, out),
		['е', s @ ..] => super::t0::n140(s, out),
		['ж', s @ ..] => super::t0::n141(s, out),
		['з', s @ ..] => super::t0::n142(s, out),
		['и', s @ ..] => {
			super::push(out, 142);
			s.is_empty()
		}
		['й', s @ ..] => super::t0::n144(s, out),
		['к', s @ ..] => {
			super::push(out, 144);
			s.is_empty()
		}
		['м', s @ ..] => super::t0::n146(s, out),
		['н', s @ ..] => super::t0::n147(s, out),
		['п', s @ ..] => {
			super::push(out, 147);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 424);
					match s {
						[] => true,
						['т', 'э', 'х', s @ ..] => super::t0::n974(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t0::n149(s, out),
		['с', s @ ..] => super::t0::n150(s, out),
		['т', s @ ..] => super::t0::n151(s, out),
		['ф', s @ ..] => super::t0::n152(s, out),
		['ч', s @ ..] => super::t0::n153(s, out),
		['ш', s @ ..] => super::t0::n154(s, out),
		['щ', s @ ..] => super::t0::n155(s, out),
		['ь', s @ ..] => super::t0::n156(s, out),
		['я', s @ ..] => super::t0::n157(s, out),
		_ => false,
	}
}

/// Words ending -пый.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1065);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 1066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 158);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рый.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 159);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 160);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 474);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1067);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1068);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 1069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 476);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 477);
					match s {
						[] => true,
						['-', 'о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 1920);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 1921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 164);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1070);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 481);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 1072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 482);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сый.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['л', 'е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1073);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 484);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 485);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 1075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тый.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 172);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 486);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 1922);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 2938);
											match s {
												[] => true,
												['л', 'е', 'з', '-', s @ ..] => {
													super::push(out, 3778);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 1923);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 2939);
											match s {
												[] => true,
												['б', '-', s @ ..] => {
													super::push(out, 3779);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 487);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 488);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							super::push(out, 1076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 489);
					match s {
						[] => true,
						['о', 'л', 'о', 'п', s @ ..] => {
							super::push(out, 1077);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 1924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 491);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1078);
							match s {
								[] => true,
								['у', 'з', 'х', s @ ..] => {
									super::push(out, 1925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 1079);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1080);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 1081);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 1082);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 1926);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 1927);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 1928);
									match s {
										[] => true,
										['д', '-', s @ ..] => {
											super::push(out, 2940);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 1083);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 1929);
									match s {
										[] => true,
										['б', 'е', s @ ..] => {
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 3780);
													s.is_empty()
												}
												['щ', s @ ..] => {
													super::push(out, 3781);
													s.is_empty()
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 2941);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 2942);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 2943);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 3782);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 3783);
													match s {
														[] => true,
														['к', 'х', s @ ..] => super::t0::n4404(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 3784);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 2944);
											match s {
												[] => true,
												['у', 'т', 'с', s @ ..] => {
													match s {
														[] => true,
														['и', s @ ..] => super::t0::n4405(s, out),
														['о', s @ ..] => super::t0::n4406(s, out),
														['х', s @ ..] => super::t0::n4407(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 2945);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 1930);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 1931);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 1084);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1085);
							match s {
								[] => true,
								['о', 'в', 'т', 'с', 'х', s @ ..] => {
									super::push(out, 1932);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 2946);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 2947);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 1086);
							match s {
								[] => true,
								['е', 'ш', s @ ..] => super::t0::n2041(s, out),
								['и', s @ ..] => super::t0::n2042(s, out),
								_ => false,
							}
						}
						['ь', s @ ..] => super::t0::n1131(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t0::n174(s, out),
		['и', s @ ..] => super::t0::n175(s, out),
		['л', s @ ..] => super::t0::n176(s, out),
		['о', s @ ..] => super::t0::n177(s, out),
		['р', s @ ..] => super::t0::n178(s, out),
		['с', s @ ..] => super::t0::n179(s, out),
		['у', s @ ..] => super::t1::n180(s, out),
		['ы', s @ ..] => super::t1::n181(s, out),
		['я', s @ ..] => super::t1::n182(s, out),
		_ => false,
	}
}

/// Words ending -цый.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	s.is_empty()
}

/// Words ending -шийся.
pub(crate) fn n518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 501);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1119);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1995);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2976);
							s.is_empty()
						}
						['в', s @ ..] => super::t1::n3178(s, out),
						['г', s @ ..] => {
							super::push(out, 2978);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 3793);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3794);
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 4336);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 4337);
													s.is_empty()
												}
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
							super::push(out, 2979);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 2980);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 2981);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 2982);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 2983);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 2984);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3795);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 4161);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 4162);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2985);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2986);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2987);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 2989);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1996);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1997);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2992);
							s.is_empty()
						}
						['ж', s @ ..] => super::t1::n3194(s, out),
						['л', s @ ..] => {
							super::push(out, 2994);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2996);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t1::n2107(s, out),
				['ы', s @ ..] => {
					super::push(out, 1999);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t1::n1170(s, out),
		['д', s @ ..] => super::t1::n1171(s, out),
		['з', s @ ..] => super::t1::n1172(s, out),
		['к', s @ ..] => super::t1::n1173(s, out),
		['р', s @ ..] => super::t1::n1174(s, out),
		['с', s @ ..] => super::t1::n1175(s, out),
		['т', s @ ..] => super::t1::n1176(s, out),
		_ => false,
	}
}

/// Words ending -щийся.
pub(crate) fn n519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 502);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 2016);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3004);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 3803);
									s.is_empty()
								}
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
			super::push(out, 1129);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2017);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3005);
							s.is_empty()
						}
						['в', s @ ..] => super::t1::n3209(s, out),
						['г', s @ ..] => {
							super::push(out, 3007);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 3807);
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 4369);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 4370);
													s.is_empty()
												}
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
							super::push(out, 3008);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 3009);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 3010);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 3011);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 3012);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3013);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 3015);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 3016);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 3017);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 3018);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 3019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 2018);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 2019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ский.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1132);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1133);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 510);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1135);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1136);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 1137);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1138);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 1139);
							match s {
								[] => true,
								['а', 'к', 'и', s @ ..] => {
									super::push(out, 2020);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 2021);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 2022);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2023);
									s.is_empty()
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 3020);
											s.is_empty()
										}
										['ф', s @ ..] => {
											super::push(out, 3021);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 2024);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 2025);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2026);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2027);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 3022);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 3023);
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
								['е', s @ ..] => {
									super::push(out, 2028);
									s.is_empty()
								}
								['ж', s @ ..] => {
									super::push(out, 2029);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 2030);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 3024);
											s.is_empty()
										}
										['ш', s @ ..] => {
											super::push(out, 3025);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1140);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 1141);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 1142);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2031);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 3026);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 3027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 2032);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 2033);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2034);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2035);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 1143);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 1144);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2036);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 1145);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['к', 'л', s @ ..] => {
							super::push(out, 1146);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2039);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2040);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 1147);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2041);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2042);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 3028);
											match s {
												[] => true,
												['о', 'л', 'е', 'б', '-', s @ ..] => {
													super::push(out, 3809);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 3810);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 3029);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => super::t1::n1198(s, out),
						['р', s @ ..] => {
							super::push(out, 1149);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1150);
							s.is_empty()
						}
						['т', s @ ..] => super::t1::n1201(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t1::n205(s, out),
		['д', s @ ..] => super::t1::n206(s, out),
		['е', s @ ..] => super::t1::n207(s, out),
		['ж', s @ ..] => super::t1::n208(s, out),
		['з', s @ ..] => super::t1::n209(s, out),
		['й', s @ ..] => super::t1::n210(s, out),
		['к', s @ ..] => super::t1::n211(s, out),
		['м', s @ ..] => super::t1::n212(s, out),
		['н', s @ ..] => super::t1::n213(s, out),
		['р', s @ ..] => super::t1::n214(s, out),
		['с', s @ ..] => super::t1::n215(s, out),
		['т', s @ ..] => super::t1::n216(s, out),
		['ь', s @ ..] => super::t1::n217(s, out),
		_ => false,
	}
}

/// Words ending -ененький.
pub(crate) fn n2239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2126);
	s.is_empty()
}

/// Words ending -нненький.
pub(crate) fn n2240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2127);
	s.is_empty()
}

/// Words ending -ерненький.
pub(crate) fn n2241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2128);
	s.is_empty()
}

/// Words ending -тненький.
pub(crate) fn n2242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2129);
	s.is_empty()
}

/// Words ending -естренький.
pub(crate) fn n1244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1187);
	s.is_empty()
}

/// Words ending -сенький.
pub(crate) fn n1245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1188);
	s.is_empty()
}

/// Words ending -тенький.
pub(crate) fn n1246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1189);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шенький.
pub(crate) fn n1247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1190);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2131);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2132);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2133);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2134);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 2135);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онький.
pub(crate) fn n557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['г', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 2136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1191);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 1192);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2139);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 2140);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 2141);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рький.
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	s.is_empty()
}

/// Words ending -вший.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['а', s @ ..] => super::t1::n243(s, out),
		['е', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 568);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1244);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 1245);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2191);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2192);
									s.is_empty()
								}
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
			super::push(out, 241);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 569);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1246);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1247);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2193);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 2194);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 2195);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 2196);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2197);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3166);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 3167);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ъ', s @ ..] => {
									super::push(out, 2198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1248);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1249);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 1250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				['н', 'е', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 577);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 578);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				['ш', 'е', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 242);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 243);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1253);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1256);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 2199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 1257);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2200);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 2201);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2202);
									match s {
										[] => true,
										['е', s @ ..] => super::t1::n3377(s, out),
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 2203);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 2204);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 2205);
									s.is_empty()
								}
								['я', s @ ..] => super::t1::n2324(s, out),
								_ => false,
							}
						}
						['и', s @ ..] => super::t1::n1320(s, out),
						['к', s @ ..] => super::t1::n1321(s, out),
						['о', s @ ..] => super::t1::n1322(s, out),
						['п', s @ ..] => super::t1::n1323(s, out),
						['р', s @ ..] => super::t1::n1324(s, out),
						['с', s @ ..] => super::t1::n1325(s, out),
						['т', s @ ..] => super::t1::n1326(s, out),
						['у', s @ ..] => super::t1::n1327(s, out),
						['х', s @ ..] => super::t1::n1328(s, out),
						['ь', s @ ..] => super::t1::n1329(s, out),
						['ю', s @ ..] => super::t1::n1330(s, out),
						['я', s @ ..] => super::t1::n1331(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t1::n248(s, out),
		['я', s @ ..] => super::t1::n249(s, out),
		_ => false,
	}
}

/// Words ending -сленнейший.
pub(crate) fn n4140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3888);
	s.is_empty()
}

/// Words ending -рекший.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 588);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 2225);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1282);
			s.is_empty()
		}
		['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
			super::push(out, 1283);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 1284);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -текший.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1285);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1286);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1287);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3189);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1288);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ркший.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	s.is_empty()
}

/// Words ending -ыкший.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	s.is_empty()
}

/// Words ending -якший.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	s.is_empty()
}

/// Words ending -пший.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	s.is_empty()
}

/// Words ending -рший.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 266);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1289);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 2227);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 2228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1291);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1292);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1293);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1294);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1295);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 1296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'г', '\u{1}', s @ ..] => {
			super::push(out, 268);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сший.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 269);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 1298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 271);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 272);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 273);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 274);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тший.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['ц', s @ ..] => {
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 2229);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 2230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 1299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хший.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	s.is_empty()
}

/// Words ending -ающий.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1304);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1305);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2231);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 2232);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 2233);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3191);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3192);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 3193);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3194);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 3889);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 3890);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 3195);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3196);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 3197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 2234);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3198);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3199);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 3891);
											s.is_empty()
										}
										['л', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 4224);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 4225);
													match s {
														[] => true,
														['и', s @ ..] => super::t1::n4665(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 4226);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 4227);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 3892);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 4228);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 4229);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ц', s @ ..] => {
											super::push(out, 3893);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 3200);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 3201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => super::t1::n2355(s, out),
						['л', s @ ..] => {
							super::push(out, 2236);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 2237);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2238);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 2239);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2240);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 2241);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 2242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1306);
					s.is_empty()
				}
				['ы', s @ ..] => super::t1::n1373(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 608);
			match s {
				[] => true,
				['р', s @ ..] => super::t1::n1374(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t1::n633(s, out),
		['ж', s @ ..] => {
			super::push(out, 610);
			s.is_empty()
		}
		['з', s @ ..] => super::t1::n635(s, out),
		['к', s @ ..] => super::t1::n636(s, out),
		['л', s @ ..] => {
			super::push(out, 613);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 614);
			s.is_empty()
		}
		['н', s @ ..] => super::t1::n639(s, out),
		['п', s @ ..] => super::t1::n640(s, out),
		['р', s @ ..] => super::t1::n641(s, out),
		['с', s @ ..] => super::t1::n642(s, out),
		['т', s @ ..] => super::t1::n643(s, out),
		['х', s @ ..] => super::t1::n644(s, out),
		['ч', s @ ..] => super::t1::n645(s, out),
		['ш', s @ ..] => super::t1::n646(s, out),
		['щ', s @ ..] => super::t1::n647(s, out),
		_ => false,
	}
}

/// Words ending -ствующий.
pub(crate) fn n1385(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2259);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 2260);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3239);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 3240);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3241);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2262);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кообразующий.
pub(crate) fn n2387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2263);
	s.is_empty()
}

/// Words ending -изующий.
pub(crate) fn n1387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1320);
	s.is_empty()
}

/// Words ending -рующий.
pub(crate) fn n651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	s.is_empty()
}

/// Words ending -яющий.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	s.is_empty()
}

/// Words ending -ящий.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	s.is_empty()
}

/// Words ending -сотрублевый.
pub(crate) fn n4166(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4241);
			s.is_empty()
		}
		['х', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4386);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4387);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ехрублевый.
pub(crate) fn n4167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3911);
	s.is_empty()
}

/// Words ending -ухрублевый.
pub(crate) fn n4168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3912);
	s.is_empty()
}

/// Words ending -невый.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 659);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2271);
							match s {
								[] => true,
								['у', 'х', s @ ..] => {
									super::push(out, 3248);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3913);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3914);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 660);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 661);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1339);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1341);
					s.is_empty()
				}
				['и', 'р', 'о', 'к', '-', s @ ..] => {
					super::push(out, 1342);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 3249);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 3250);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 3251);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 3915);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 3916);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 3252);
									s.is_empty()
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 3917);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 3918);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 664);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['в', '-', s @ ..] => {
							super::push(out, 2272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осевый.
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 669);
	s.is_empty()
}

/// Words ending -тевый.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 670);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 671);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уевый.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 673);
			s.is_empty()
		}
		['х', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1346);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 1347);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цевый.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	s.is_empty()
}

/// Words ending -чевый.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['т', 'а', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['м', 'х', 'е', s @ ..] => {
					super::push(out, 1348);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шевый.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1349);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 676);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щевый.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	s.is_empty()
}

/// Words ending -ьевый.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['н', 'а', s @ ..] => {
			super::push(out, 677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерезвый.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	s.is_empty()
}

/// Words ending -ивый.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 309);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 678);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1351);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2273);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 2274);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 2275);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2276);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 2277);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1352);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1353);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1354);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', 'б', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овый.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 312);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 683);
					match s {
						[] => true,
						['и', 'р', s @ ..] => {
							super::push(out, 1356);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2278);
									match s {
										[] => true,
										['л', 'к', '-', s @ ..] => {
											super::push(out, 3253);
											s.is_empty()
										}
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
					super::push(out, 684);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1357);
							match s {
								[] => true,
								['а', 'з', 'х', s @ ..] => {
									super::push(out, 2279);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3254);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 1358);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2280);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2281);
									s.is_empty()
								}
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
			super::push(out, 314);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 685);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 1359);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 2282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 688);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1360);
							s.is_empty()
						}
						['о', 'б', s @ ..] => {
							super::push(out, 1361);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 2283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2284);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2285);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3256);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3257);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 315);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 1362);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2286);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2287);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3258);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3259);
											s.is_empty()
										}
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
					super::push(out, 691);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2288);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 2289);
									s.is_empty()
								}
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
			super::push(out, 316);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 692);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 693);
					match s {
						[] => true,
						['е', 'ш', s @ ..] => {
							super::push(out, 1363);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 694);
					match s {
						[] => true,
						['а', 'в', s @ ..] => {
							super::push(out, 1365);
							match s {
								[] => true,
								['к', 'х', s @ ..] => {
									super::push(out, 2290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['с', 'е', s @ ..] => {
							super::push(out, 1366);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2291);
									s.is_empty()
								}
								['п', 'е', 'л', 'х', s @ ..] => {
									super::push(out, 2292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1367);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 2293);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3260);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3261);
											s.is_empty()
										}
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
					super::push(out, 697);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 1368);
							match s {
								[] => true,
								['е', 'в', 'х', s @ ..] => {
									super::push(out, 2294);
									s.is_empty()
								}
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
			super::push(out, 317);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1369);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 2295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 699);
					match s {
						[] => true,
						['г', 'х', s @ ..] => {
							super::push(out, 1370);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2296);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2297);
									s.is_empty()
								}
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
			super::push(out, 318);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						['ю', 'д', 'х', s @ ..] => {
							super::push(out, 1372);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2298);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['а', 'р', 'г', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2300);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2301);
									s.is_empty()
								}
								['т', s @ ..] => super::t1::n2428(s, out),
								_ => false,
							}
						}
						['е', s @ ..] => super::t1::n1446(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t1::n730(s, out),
				['ь', s @ ..] => super::t1::n731(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t1::n327(s, out),
		['п', s @ ..] => super::t1::n328(s, out),
		['р', s @ ..] => super::t1::n329(s, out),
		['с', s @ ..] => super::t1::n330(s, out),
		['т', s @ ..] => super::t1::n331(s, out),
		['х', s @ ..] => super::t1::n332(s, out),
		['ц', s @ ..] => super::t1::n333(s, out),
		_ => false,
	}
}

/// Words ending -твый.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	s.is_empty()
}

/// Words ending -явый.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	s.is_empty()
}

/// Words ending -ваемый.
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 740);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1398);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1399);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1400);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 2312);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3278);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3279);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 3280);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 3281);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3282);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 3283);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 3284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3285);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3286);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 3926);
									s.is_empty()
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 4246);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 4247);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 4248);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 4249);
											s.is_empty()
										}
										['о', 'к', s @ ..] => {
											super::push(out, 4250);
											s.is_empty()
										}
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
					super::push(out, 2313);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3927);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3288);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3929);
									s.is_empty()
								}
								['е', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 4251);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 4252);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3289);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3930);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3931);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 3932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 2317);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 2318);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 2320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1401);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2321);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3290);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3291);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2322);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3293);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2323);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2324);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 2325);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2326);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2327);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3295);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2328);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3297);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3298);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3299);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2329);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3301);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 3302);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 3303);
							s.is_empty()
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 3933);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 3934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3304);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 3935);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 3936);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 3937);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ергаемый.
pub(crate) fn n2462(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 3306);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 3307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -даемый.
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 742);
	s.is_empty()
}

/// Words ending -жаемый.
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 743);
	s.is_empty()
}

/// Words ending -заемый.
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 744);
	s.is_empty()
}

/// Words ending -каемый.
pub(crate) fn n772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 745);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1403);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1404);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2330);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лаемый.
pub(crate) fn n773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 746);
	s.is_empty()
}

/// Words ending -маемый.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 747);
	s.is_empty()
}

/// Words ending -наемый.
pub(crate) fn n775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 748);
	s.is_empty()
}

/// Words ending -паемый.
pub(crate) fn n776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 749);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1407);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3308);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 3309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1408);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 3310);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 3311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1409);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раемый.
pub(crate) fn n777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 750);
	s.is_empty()
}

/// Words ending -таемый.
pub(crate) fn n778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 751);
	s.is_empty()
}

/// Words ending -цаемый.
pub(crate) fn n779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 752);
	s.is_empty()
}

/// Words ending -чаемый.
pub(crate) fn n780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 753);
	s.is_empty()
}

/// Words ending -шаемый.
pub(crate) fn n781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 754);
	s.is_empty()
}

/// Words ending -щаемый.
pub(crate) fn n782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 755);
	s.is_empty()
}

/// Words ending -уемый.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 349);
	s.is_empty()
}

/// Words ending -яемый.
pub(crate) fn n359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	s.is_empty()
}

/// Words ending -имый.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 351);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 1410);
							s.is_empty()
						}
						['р', 'о', s @ ..] => {
							super::push(out, 1411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омый.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	s.is_empty()
}

/// Words ending -дный.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 370);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 371);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 784);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 785);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1416);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 2341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 1417);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1418);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'о', s @ ..] => {
					super::push(out, 787);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 788);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1420);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 789);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1422);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 1423);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 3320);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 3321);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 3938);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 3939);
													s.is_empty()
												}
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
							super::push(out, 1424);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1425);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1426);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 1427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1428);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1429);
							match s {
								[] => true,
								['б', 'е', s @ ..] => {
									super::push(out, 2342);
									s.is_empty()
								}
								['к', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 3322);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 3940);
													s.is_empty()
												}
												['и', s @ ..] => {
													super::push(out, 3941);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 3323);
											match s {
												[] => true,
												['е', 'п', s @ ..] => {
													super::push(out, 3942);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 2343);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 2344);
									match s {
										[] => true,
										['е', 'з', s @ ..] => {
											super::push(out, 3324);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 2345);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 2346);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3325);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 3326);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 2347);
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
							super::push(out, 1430);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2348);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 1431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1432);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 791);
					s.is_empty()
				}
				['у', 'к', s @ ..] => {
					super::push(out, 792);
					match s {
						[] => true,
						['е', 'с', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2350);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2351);
									s.is_empty()
								}
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
			super::push(out, 375);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 377);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', 'х', s @ ..] => {
							super::push(out, 1434);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2352);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2353);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 1435);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 3327);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 3328);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 3943);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 3944);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еный.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 793);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 380);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['а', 'с', 'о', s @ ..] => {
					super::push(out, 795);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 796);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 797);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 798);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 799);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 800);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1436);
							s.is_empty()
						}
						['р', 'г', s @ ..] => {
							super::push(out, 1437);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 801);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1439);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2354);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 1440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 802);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1441);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1443);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 804);
					match s {
						[] => true,
						['з', '-', 'о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 2356);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 3329);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 3330);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 3331);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 2357);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 3332);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 3333);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'р', s @ ..] => {
					super::push(out, 805);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 1445);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 1446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 806);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 807);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 808);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1447);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 811);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1450);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1451);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 1452);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 813);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 814);
					match s {
						[] => true,
						['у', 'п', s @ ..] => {
							super::push(out, 1454);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 815);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 816);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 1456);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 820);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 822);
					match s {
						[] => true,
						['о', 'к', '-', s @ ..] => {
							super::push(out, 1458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 823);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1459);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'о', s @ ..] => {
					super::push(out, 824);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2358);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2359);
									s.is_empty()
								}
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
			super::push(out, 388);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1462);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 827);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жный.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 830);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 831);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1466);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 1467);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2360);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2361);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2362);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3334);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3335);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 391);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 832);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 833);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1468);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 2363);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 1469);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 835);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['н', 'о', s @ ..] => {
									super::push(out, 2365);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 2366);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 839);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1477);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2367);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2368);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 1480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 842);
					match s {
						[] => true,
						['с', 'х', s @ ..] => {
							super::push(out, 1481);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2370);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2371);
									s.is_empty()
								}
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
			super::push(out, 394);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 843);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1482);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 844);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 845);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 1484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 396);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 397);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 398);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зный.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 846);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2372);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 3336);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 3945);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 3946);
													match s {
														[] => true,
														['л', s @ ..] => super::t1::n4530(s, out),
														['р', s @ ..] => super::t1::n4531(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['к', 'ш', s @ ..] => {
											super::push(out, 3337);
											match s {
												[] => true,
												['е', 'п', s @ ..] => {
													super::push(out, 3947);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 3338);
											match s {
												[] => true,
												['е', 'л', 'о', s @ ..] => {
													super::push(out, 3948);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 847);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 1486);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2373);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2374);
									s.is_empty()
								}
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
			super::push(out, 400);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 848);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 849);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1487);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1488);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 1489);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1490);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 851);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 852);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 853);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1493);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2375);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 854);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 401);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 856);
					match s {
						[] => true,
						['а', 'м', 'е', s @ ..] => {
							super::push(out, 1495);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 2377);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 2378);
									s.is_empty()
								}
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
			super::push(out, 403);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 404);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['в', 'с', 'х', s @ ..] => {
					super::push(out, 857);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йный.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 858);
					match s {
						[] => true,
						['и', 'л', 'х', s @ ..] => {
							super::push(out, 1496);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2379);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2380);
									s.is_empty()
								}
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
			super::push(out, 407);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 859);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 860);
					match s {
						[] => true,
						['е', 'с', 'х', s @ ..] => {
							super::push(out, 1497);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2381);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'р', 'а', 'п', 'х', s @ ..] => {
					super::push(out, 861);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 862);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 863);
					match s {
						[] => true,
						['х', 'о', s @ ..] => {
							super::push(out, 1500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 864);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2383);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2384);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3339);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3340);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мный.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 409);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 865);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 866);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 867);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 868);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 2385);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 2386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1503);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1504);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2387);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 2388);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 2389);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 3341);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 3342);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 2390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 869);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 870);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 871);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 873);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 874);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 1506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1507);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 877);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 878);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 879);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 880);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 1510);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 881);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 882);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 411);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['а', 'р', 'г', 'о', 'р', 'п', 'х', s @ ..] => {
					super::push(out, 884);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1512);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 885);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1514);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2391);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2392);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2393);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3343);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 1516);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2394);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2395);
									s.is_empty()
								}
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
			super::push(out, 414);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1517);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нный.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1519);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1520);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2396);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 2397);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2398);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 2399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 1521);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1522);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 1523);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1525);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 2400);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 2401);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 3345);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 3949);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 2402);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2403);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 2404);
									match s {
										[] => true,
										['р', 'о', 'к', 'ы', s @ ..] => {
											super::push(out, 3346);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 3347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 1526);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1527);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 2405);
									match s {
										[] => true,
										['и', 'ц', s @ ..] => {
											super::push(out, 3348);
											match s {
												[] => true,
												['и', 'ф', s @ ..] => {
													match s {
														[] => true,
														['е', s @ ..] => super::t1::n4532(s, out),
														['и', s @ ..] => super::t1::n4533(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 890);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1529);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1530);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2406);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 1531);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1532);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 1533);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2408);
									match s {
										[] => true,
										['д', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 3950);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 3951);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 2409);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => super::t1::n1618(s, out),
						_ => false,
					}
				}
				['д', s @ ..] => super::t1::n929(s, out),
				['ж', s @ ..] => super::t1::n930(s, out),
				['з', s @ ..] => super::t1::n931(s, out),
				['к', s @ ..] => super::t1::n932(s, out),
				['л', s @ ..] => super::t1::n933(s, out),
				['м', s @ ..] => super::t1::n934(s, out),
				['н', s @ ..] => super::t1::n935(s, out),
				['п', s @ ..] => super::t1::n936(s, out),
				['р', s @ ..] => super::t1::n937(s, out),
				['с', s @ ..] => super::t1::n938(s, out),
				['т', s @ ..] => super::t1::n939(s, out),
				['х', s @ ..] => super::t1::n940(s, out),
				['ч', s @ ..] => super::t1::n941(s, out),
				['ш', s @ ..] => super::t1::n942(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t1::n428(s, out),
		['и', s @ ..] => super::t1::n429(s, out),
		['о', s @ ..] => super::t1::n430(s, out),
		['у', s @ ..] => super::t1::n431(s, out),
		['ы', s @ ..] => super::t1::n432(s, out),
		['я', s @ ..] => super::t1::n433(s, out),
		_ => false,
	}
}

/// Words ending -хэтапный.
pub(crate) fn n974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 935);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1724);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 1725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рный.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['о', 'т', 'о', s @ ..] => {
							super::push(out, 1726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['и', 'з', s @ ..] => {
							super::push(out, 1727);
							match s {
								[] => true,
								['о', 'п', 'е', 'д', '-', s @ ..] => {
									super::push(out, 2756);
									s.is_empty()
								}
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
			super::push(out, 426);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1728);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1729);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1731);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2757);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 1732);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 2759);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3688);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 1733);
							s.is_empty()
						}
						['о', 'к', s @ ..] => {
							super::push(out, 1734);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 2760);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 2761);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1735);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2762);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 2763);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 3690);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1737);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2764);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 2765);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 3691);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 3692);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 4102);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 4103);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 942);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1738);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 1740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1741);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1742);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1743);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1744);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 1745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 1746);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 2767);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 946);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1749);
							match s {
								[] => true,
								['к', 'х', s @ ..] => {
									super::push(out, 2769);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3693);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3694);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 1751);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1752);
							match s {
								[] => true,
								['р', 'г', s @ ..] => {
									super::push(out, 2770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1753);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 1754);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2771);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2772);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t1::n986(s, out),
				['п', s @ ..] => super::t1::n987(s, out),
				['с', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['с', s @ ..] => super::t1::n1858(s, out),
						_ => false,
					}
				}
				['ф', s @ ..] => super::t1::n990(s, out),
				['ч', s @ ..] => super::t1::n991(s, out),
				['ш', s @ ..] => {
					super::push(out, 953);
					s.is_empty()
				}
				['ь', s @ ..] => super::t1::n993(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t1::n437(s, out),
		['о', s @ ..] => super::t1::n438(s, out),
		['у', s @ ..] => super::t1::n439(s, out),
		['ю', s @ ..] => super::t1::n440(s, out),
		['я', s @ ..] => super::t1::n441(s, out),
		_ => false,
	}
}

/// Words ending -сный.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 960);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 2783);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3695);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 4104);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 4105);
													match s {
														[] => true,
														['м', s @ ..] => super::t1::n4575(s, out),
														['н', s @ ..] => super::t1::n4576(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 4106);
													match s {
														[] => true,
														['а', s @ ..] => super::t1::n4577(s, out),
														['л', s @ ..] => super::t1::n4578(s, out),
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
									super::push(out, 2784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 1768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1769);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1770);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1772);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 2785);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 2786);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 3696);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 3697);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 4107);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 4108);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 2787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 434);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 435);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 436);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 969);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 1774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['о', 'п', 'х', s @ ..] => {
							super::push(out, 1775);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2788);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 971);
					match s {
						[] => true,
						['о', 'т', s @ ..] => {
							super::push(out, 1776);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2790);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 3698);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 3699);
											s.is_empty()
										}
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
					super::push(out, 972);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 438);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 439);
			match s {
				[] => true,
				['а', 'л', 'к', 'х', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 1779);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 1780);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2791);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 2792);
									match s {
										[] => true,
										['х', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 4109);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 4110);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 441);
			match s {
				[] => true,
				['л', 'о', 'п', 'х', s @ ..] => {
					super::push(out, 975);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1781);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тный.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 977);
					match s {
						[] => true,
						['о', 'р', 'п', 'о', s @ ..] => {
							super::push(out, 1783);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 2793);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3700);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 3701);
											s.is_empty()
										}
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
					super::push(out, 978);
					match s {
						[] => true,
						['а', 'п', s @ ..] => {
							super::push(out, 1784);
							match s {
								[] => true,
								['х', 'е', s @ ..] => {
									super::push(out, 2794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 980);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 1785);
							match s {
								[] => true,
								['д', 'р', 'о', 'о', 'к', 'х', s @ ..] => {
									super::push(out, 2795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 1786);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 3702);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 3703);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 4111);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 4112);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 981);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 1787);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2796);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 2797);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2798);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2799);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3704);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3705);
											s.is_empty()
										}
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
					super::push(out, 982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 983);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 984);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 1788);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 1789);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2800);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2801);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3706);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3707);
											s.is_empty()
										}
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
					super::push(out, 985);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1790);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 1791);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2802);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 2803);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 2804);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2805);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2806);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 1792);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 1793);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1794);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 2807);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 1795);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						['к', s @ ..] => super::t1::n1896(s, out),
						['о', s @ ..] => super::t1::n1897(s, out),
						['п', s @ ..] => super::t1::n1898(s, out),
						['с', s @ ..] => super::t1::n1899(s, out),
						['т', 'о', 'к', s @ ..] => super::t1::n1900(s, out),
						['у', 'м', s @ ..] => super::t1::n1901(s, out),
						['ф', s @ ..] => super::t1::n1902(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t1::n1029(s, out),
				['н', s @ ..] => super::t1::n1030(s, out),
				['п', s @ ..] => super::t1::n1031(s, out),
				['р', s @ ..] => super::t1::n1032(s, out),
				['с', s @ ..] => super::t1::n1033(s, out),
				['т', s @ ..] => super::t1::n1034(s, out),
				['у', s @ ..] => super::t1::n1035(s, out),
				['ф', s @ ..] => super::t1::n1036(s, out),
				['ч', s @ ..] => super::t1::n1037(s, out),
				['ш', s @ ..] => super::t1::n1038(s, out),
				['щ', s @ ..] => super::t1::n1039(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t1::n454(s, out),
		['й', s @ ..] => super::t1::n455(s, out),
		['к', s @ ..] => super::t1::n456(s, out),
		['н', s @ ..] => super::t1::n457(s, out),
		['о', s @ ..] => super::t1::n458(s, out),
		['р', s @ ..] => super::t1::n459(s, out),
		['с', s @ ..] => super::t1::n460(s, out),
		['т', s @ ..] => super::t1::n461(s, out),
		['у', s @ ..] => super::t1::n462(s, out),
		['ш', s @ ..] => super::t1::n463(s, out),
		['ы', s @ ..] => super::t1::n464(s, out),
		['ь', s @ ..] => super::t1::n465(s, out),
		['ю', s @ ..] => super::t1::n466(s, out),
		['я', s @ ..] => super::t1::n467(s, out),
		_ => false,
	}
}

/// Words ending -фный.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	s.is_empty()
}

/// Words ending -чный.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2851);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2852);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3729);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3730);
											s.is_empty()
										}
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
					super::push(out, 1018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1019);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1021);
					match s {
						[] => true,
						['п', 'о', 'к', 'х', s @ ..] => {
							super::push(out, 1836);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2853);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 1022);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1023);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 1837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1024);
					match s {
						[] => true,
						['о', 'к', 'х', s @ ..] => {
							super::push(out, 1838);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2855);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1025);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1026);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1839);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 2857);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 2858);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 2859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1027);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 2860);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 2861);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 2862);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1028);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 1029);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1844);
							match s {
								[] => true,
								['п', 'е', s @ ..] => {
									super::push(out, 2864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 1030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 460);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1031);
					match s {
						[] => true,
						['о', 'г', 'х', s @ ..] => {
							super::push(out, 1845);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2865);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1032);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'а', s @ ..] => {
					super::push(out, 1033);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							super::push(out, 1848);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 3731);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 3732);
											s.is_empty()
										}
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
					super::push(out, 1034);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 461);
			s.is_empty()
		}
		['о', s @ ..] => super::t1::n472(s, out),
		['у', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1048);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 1896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['з', 'я', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1897);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 1898);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2910);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2911);
									s.is_empty()
								}
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
			super::push(out, 465);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1049);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 1899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['с', 'е', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2912);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 2913);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2914);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3756);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 3757);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шный.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 467);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1051);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1052);
					s.is_empty()
				}
				['р', 'е', s @ ..] => {
					super::push(out, 1053);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1054);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 1055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['а', 'л', 'к', '-', s @ ..] => {
							super::push(out, 1900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 470);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 2915);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 2916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щный.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1901);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1060);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1903);
							match s {
								[] => true,
								['г', 'а', 'л', 'в', '-', s @ ..] => {
									super::push(out, 2917);
									s.is_empty()
								}
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
			super::push(out, 472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьный.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1904);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2918);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 2919);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 3758);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 3759);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1905);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 2920);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 3760);
											match s {
												[] => true,
												['ф', 'о', '-', s @ ..] => {
													super::push(out, 4125);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 3761);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 1906);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1907);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2921);
									match s {
										[] => true,
										['м', 'о', s @ ..] => {
											super::push(out, 3762);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 4126);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 4127);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1062);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 1908);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1909);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 3763);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 4128);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 4129);
													s.is_empty()
												}
												['х', s @ ..] => {
													super::push(out, 4130);
													match s {
														[] => true,
														['е', s @ ..] => super::t1::n4580(s, out),
														['у', s @ ..] => super::t1::n4581(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 3764);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', 'а', 'р', 'х', s @ ..] => {
									super::push(out, 2922);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 1910);
							match s {
								[] => true,
								['е', 'з', 'о', s @ ..] => {
									super::push(out, 2924);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 3765);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1911);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2925);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 3766);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 4131);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 3767);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 3768);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 2926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 1912);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2927);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 2928);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 3769);
											match s {
												[] => true,
												['р', 'т', 'с', 'о', 'т', s @ ..] => {
													super::push(out, 4132);
													match s {
														[] => true,
														['е', s @ ..] => super::t1::n4582(s, out),
														['с', s @ ..] => super::t1::n4583(s, out),
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
									super::push(out, 2929);
									match s {
										[] => true,
										['о', 'к', s @ ..] => {
											super::push(out, 3770);
											s.is_empty()
										}
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
					super::push(out, 1063);
					match s {
						[] => true,
						['д', s @ ..] => super::t1::n2021(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t1::n1106(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яный.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	s.is_empty()
}

/// Words ending -хколенчатый.
pub(crate) fn n4404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4135);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4304);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иступенчатый.
pub(crate) fn n4405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4136);
	s.is_empty()
}

/// Words ending -оступенчатый.
pub(crate) fn n4406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4137);
	s.is_empty()
}

/// Words ending -хступенчатый.
pub(crate) fn n4407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4138);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4306);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шетчатый.
pub(crate) fn n2041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1933);
	match s {
		[] => true,
		['е', 'р', 'о', s @ ..] => {
			super::push(out, 2948);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -итчатый.
pub(crate) fn n2042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1934);
	s.is_empty()
}

/// Words ending -ьчатый.
pub(crate) fn n1131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1087);
	s.is_empty()
}

/// Words ending -етый.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	s.is_empty()
}

/// Words ending -итый.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	s.is_empty()
}

/// Words ending -лтый.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['е', 'ж', '-', 'о', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1935);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 1936);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 1937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отый.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 493);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1090);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 1091);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1938);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1939);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртый.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1940);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1941);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1942);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1092);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1945);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1946);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 1947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стый.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 496);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1948);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1949);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1094);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 1095);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 1096);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1951);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1952);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2949);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 2950);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 2951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1098);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1954);
							match s {
								[] => true,
								['з', 'и', s @ ..] => {
									super::push(out, 2952);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 2953);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1955);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1100);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1958);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 2954);
									match s {
										[] => true,
										['ч', 'х', s @ ..] => {
											super::push(out, 3785);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', 'е', s @ ..] => {
					super::push(out, 1101);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 1959);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1961);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 2955);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 2956);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 2957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1962);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 2958);
									match s {
										[] => true,
										['х', 'х', s @ ..] => {
											super::push(out, 3786);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'е', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1963);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'е', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 1965);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 1106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['в', 'х', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1967);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1969);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1970);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 1971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

