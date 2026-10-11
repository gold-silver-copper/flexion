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
					super::push(out, 11);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 34);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 35);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 134);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 36);
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 483);
											s.is_empty()
										}
										['с', 'е', 'к', '\u{1}', s @ ..] => {
											super::push(out, 484);
											s.is_empty()
										}
										['ц', s @ ..] => {
											super::push(out, 485);
											s.is_empty()
										}
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
					super::push(out, 12);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 37);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 486);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 487);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 488);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 1474);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 135);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 489);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 38);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 39);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 40);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 41);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 42);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 137);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 138);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 490);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 1475);
													s.is_empty()
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 491);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 139);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 140);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 492);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 43);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 44);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 141);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 142);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 143);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 493);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 45);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 46);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 144);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 494);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 47);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 145);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 495);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 48);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 14);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 15);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 50);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t0::n4(s, out),
		['к', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 1454);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 4355);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 1455);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t0::n21(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22);
					match s {
						[] => true,
						['м', s @ ..] => super::t0::n106(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t0::n7(s, out),
		['т', s @ ..] => super::t0::n8(s, out),
		['ь', s @ ..] => super::t0::n9(s, out),
		['я', s @ ..] => super::t0::n10(s, out),
		_ => false,
	}
}

/// Words ending -й.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['и', s @ ..] => super::t0::n16(s, out),
		['о', s @ ..] => super::t0::n17(s, out),
		['ы', s @ ..] => super::t0::n18(s, out),
		_ => false,
	}
}

/// Words ending -ок.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 446);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мал.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	s.is_empty()
}

/// Words ending -н.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 107);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 108);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 109);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 110);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 111);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 25);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 113);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 114);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 115);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 453);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 1456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 454);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 1457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 455);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 1458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 456);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 1459);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 4356);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 4357);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 4358);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 9914);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', 'н', 'е', 'м', 'а', 'м', '\u{1}', s @ ..] => {
							super::push(out, 457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 116);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 117);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 459);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 1460);
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
									super::push(out, 1461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 461);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 1462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 462);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 1463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 463);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 464);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 465);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 466);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 1464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 119);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 120);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 468);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 469);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 470);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 121);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 472);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 1465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 122);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 474);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 1466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 123);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 476);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 1467);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 1468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 124);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 477);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 1469);
									s.is_empty()
								}
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
			super::push(out, 26);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 125);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 28);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 127);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ь.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 29);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 30);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -я.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 129);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 479);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 1470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 480);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 1471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 130);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t0::n32(s, out),
		_ => false,
	}
}

/// Words ending -ий.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 51);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 148);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 496);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 1476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 149);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 150);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 151);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 152);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 500);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 501);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 502);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 1477);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 1478);
									s.is_empty()
								}
								['и', 'м', s @ ..] => {
									super::push(out, 1479);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 4373);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 1480);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4374);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 4375);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 1481);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 4376);
											s.is_empty()
										}
										['л', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 9967);
													match s {
														[] => true,
														['г', s @ ..] => super::t0::n18099(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 9968);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 9969);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 9970);
													match s {
														[] => true,
														['а', s @ ..] => super::t0::n18100(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 4377);
											match s {
												[] => true,
												['е', 'в', 'з', s @ ..] => {
													super::push(out, 9971);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 9972);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 9973);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 9974);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 4378);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 1482);
									s.is_empty()
								}
								['х', 'е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 4379);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 4380);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t0::n507(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t0::n155(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 154);
					match s {
						[] => true,
						['и', s @ ..] => super::t0::n510(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t0::n55(s, out),
		['ж', s @ ..] => super::t0::n56(s, out),
		['з', s @ ..] => super::t0::n57(s, out),
		['к', s @ ..] => super::t0::n58(s, out),
		['л', s @ ..] => super::t0::n59(s, out),
		['м', s @ ..] => super::t0::n60(s, out),
		['н', s @ ..] => super::t0::n61(s, out),
		['п', s @ ..] => super::t0::n62(s, out),
		['р', s @ ..] => super::t0::n63(s, out),
		['с', s @ ..] => super::t0::n64(s, out),
		['т', s @ ..] => super::t0::n65(s, out),
		['у', s @ ..] => super::t0::n66(s, out),
		['ф', s @ ..] => super::t0::n67(s, out),
		['х', s @ ..] => super::t0::n68(s, out),
		['ч', s @ ..] => super::t0::n69(s, out),
		['ш', s @ ..] => super::t0::n70(s, out),
		['щ', s @ ..] => super::t0::n71(s, out),
		_ => false,
	}
}

/// Words ending -ой.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 275);
					match s {
						[] => true,
						['л', 'о', 'г', '-', 'о', s @ ..] => {
							match s {
								[] => true,
								['н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
									super::push(out, 2296);
									s.is_empty()
								}
								['р', 'у', 'б', '-', 'о', 'т', 'л', 'е', 'ж', '\u{1}', s @ ..] => {
									super::push(out, 2297);
									s.is_empty()
								}
								['т', 'л', 'е', 'ж', '\u{1}', s @ ..] => {
									super::push(out, 2298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 276);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 845);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2299);
									s.is_empty()
								}
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
			super::push(out, 73);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 277);
					match s {
						[] => true,
						['н', 'ж', 'р', 'е', 'т', 'с', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 846);
							s.is_empty()
						}
						['ч', 'у', 'л', 'х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 278);
					match s {
						[] => true,
						['ж', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2300);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 6073);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 2301);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 6074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 848);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 2302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 279);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 849);
							match s {
								[] => true,
								['у', 'д', s @ ..] => {
									super::push(out, 2303);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6075);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', 'о', 'г', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 851);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2304);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 12578);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 12579);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 6076);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 2305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 852);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 2306);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 6077);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 12580);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'а', 'ч', 'х', 'е', 'р', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2307);
									s.is_empty()
								}
								['и', 'т', 'а', 'ц', 'д', 'а', 'в', 'д', '\u{1}', s @ ..] => {
									super::push(out, 2308);
									s.is_empty()
								}
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
			super::push(out, 74);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 280);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 853);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 854);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 2309);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 6078);
											s.is_empty()
										}
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
					super::push(out, 281);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 282);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 856);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 2310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 75);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 283);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 284);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 858);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 2311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 859);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 285);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 286);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 287);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2312);
									s.is_empty()
								}
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
			super::push(out, 76);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 288);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 289);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['р', 'о', 'б', '\u{1}', s @ ..] => {
					super::push(out, 290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 291);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 864);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 2313);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6079);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 865);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									super::push(out, 2314);
									match s {
										[] => true,
										['а', 'з', '-', s @ ..] => super::t0::n6341(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t0::n79(s, out),
		['м', s @ ..] => super::t0::n80(s, out),
		['н', s @ ..] => super::t0::n81(s, out),
		['п', s @ ..] => super::t0::n82(s, out),
		['р', s @ ..] => super::t0::n83(s, out),
		['с', s @ ..] => super::t0::n84(s, out),
		['т', s @ ..] => super::t0::n85(s, out),
		['х', s @ ..] => super::t0::n86(s, out),
		['ш', s @ ..] => super::t0::n87(s, out),
		['щ', s @ ..] => super::t0::n88(s, out),
		_ => false,
	}
}

/// Words ending -ый.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 89);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 332);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6112);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 6113);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 12598);
													s.is_empty()
												}
												_ => false,
											}
										}
										['х', s @ ..] => {
											super::push(out, 6114);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 12599);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 333);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 924);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 925);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 334);
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 2359);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 2360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 2361);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 6115);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 6116);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 12600);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 6117);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 12601);
													match s {
														[] => true,
														['а', s @ ..] => super::t0::n20925(s, out),
														_ => false,
													}
												}
												['з', s @ ..] => {
													super::push(out, 12602);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 12603);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 6118);
											s.is_empty()
										}
										['т', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 12604);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 12605);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 12606);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'о', '\u{1}', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 335);
					match s {
						[] => true,
						['о', 'г', s @ ..] => {
							match s {
								[] => true,
								['о', 'н', 'д', 'о', '\u{1}', s @ ..] => {
									super::push(out, 2362);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 2363);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 336);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 927);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 2364);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 6119);
											match s {
												[] => true,
												['х', 'у', 'п', '\u{1}', s @ ..] => {
													super::push(out, 12607);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', 'с', '\u{1}', s @ ..] => {
									super::push(out, 2365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => super::t0::n952(s, out),
						['л', s @ ..] => {
							super::push(out, 929);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 930);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 2371);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6128);
											s.is_empty()
										}
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
					super::push(out, 337);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t0::n90(s, out),
		['д', s @ ..] => super::t0::n91(s, out),
		['з', s @ ..] => super::t0::n92(s, out),
		['л', s @ ..] => super::t0::n93(s, out),
		['м', s @ ..] => super::t0::n94(s, out),
		['н', s @ ..] => super::t0::n95(s, out),
		['п', s @ ..] => super::t0::n96(s, out),
		['р', s @ ..] => super::t0::n97(s, out),
		['с', s @ ..] => super::t0::n98(s, out),
		['т', s @ ..] => super::t0::n99(s, out),
		['ц', s @ ..] => super::t0::n100(s, out),
		_ => false,
	}
}

/// Words ending -ся.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['й', s @ ..] => super::t0::n131(s, out),
		_ => false,
	}
}

/// Words ending -голоногий.
pub(crate) fn n18099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17190);
	s.is_empty()
}

/// Words ending -асноногий.
pub(crate) fn n18100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17191);
	s.is_empty()
}

/// Words ending -рогий.
pub(crate) fn n507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 503);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1483);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 4381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1484);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4382);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4383);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9975);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 9976);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 17192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4384);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4385);
					match s {
						[] => true,
						['н', 'и', 'в', '\u{1}', s @ ..] => {
							super::push(out, 9977);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1485);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9979);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9980);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 17193);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24127);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1486);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4388);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 17194);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24128);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 17195);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 24129);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угий.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['п', 'у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4389);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 505);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 1488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -издий.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 506);
	s.is_empty()
}

/// Words ending -еий.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['о', 'т', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 1489);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жий.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1490);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1492);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1493);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4393);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4394);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 9981);
											s.is_empty()
										}
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
					super::push(out, 509);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 511);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1497);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 4395);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', 'е', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 1499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 514);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4396);
									s.is_empty()
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 17196);
													s.is_empty()
												}
												['у', s @ ..] => {
													super::push(out, 17197);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 9982);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 17198);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 9983);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 4397);
									match s {
										[] => true,
										['м', 'е', 'т', '\u{1}', s @ ..] => {
											super::push(out, 9984);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 9985);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 9986);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 9987);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 17199);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 515);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 1500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 516);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1501);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4398);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 9988);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 4399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1502);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 1503);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1505);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 4400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1506);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 4401);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 9989);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 4402);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4403);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 9990);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 1507);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4404);
									s.is_empty()
								}
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
			super::push(out, 160);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 518);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 519);
					match s {
						[] => true,
						['о', 'д', '\u{1}', s @ ..] => {
							super::push(out, 1509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 1510);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4405);
									match s {
										[] => true,
										['н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
											super::push(out, 9991);
											s.is_empty()
										}
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
					super::push(out, 521);
					match s {
						[] => true,
						['с', s @ ..] => super::t0::n1552(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t0::n164(s, out),
		['я', s @ ..] => super::t0::n165(s, out),
		_ => false,
	}
}

/// Words ending -зий.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кий.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 1515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 166);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 530);
					match s {
						[] => true,
						['г', 'х', s @ ..] => {
							super::push(out, 1516);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 4408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 1517);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 4409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 1518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 534);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4410);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1521);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4412);
									s.is_empty()
								}
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
			super::push(out, 168);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 536);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1522);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4413);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 4414);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 9992);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 4415);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 9993);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'о', 'в', '\u{1}', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 538);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 1523);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4416);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4417);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 9994);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 4418);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 9995);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 4419);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 9996);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 4420);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 9997);
											match s {
												[] => true,
												['е', 'в', 'с', '\u{1}', s @ ..] => {
													super::push(out, 17200);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 169);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 539);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 1524);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4421);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 9998);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 1525);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4422);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 4423);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 9999);
											match s {
												[] => true,
												['о', 'н', 'е', 'л', 'о', 'с', '\u{1}', s @ ..] => {
													super::push(out, 17201);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 10000);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 10001);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 10002);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 1526);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1527);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 540);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 1528);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4425);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 541);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 1530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 543);
					match s {
						[] => true,
						['л', 'о', 'с', '\u{1}', s @ ..] => {
							super::push(out, 1531);
							s.is_empty()
						}
						['х', '\u{1}', s @ ..] => {
							super::push(out, 1532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t0::n172(s, out),
		['ж', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['л', s @ ..] => super::t0::n1578(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 549);
					match s {
						[] => true,
						['р', s @ ..] => super::t0::n1579(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t0::n555(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t0::n174(s, out),
		['и', s @ ..] => super::t0::n175(s, out),
		['й', s @ ..] => super::t0::n176(s, out),
		['л', s @ ..] => super::t0::n177(s, out),
		['м', s @ ..] => super::t0::n178(s, out),
		['н', s @ ..] => super::t0::n179(s, out),
		['о', s @ ..] => super::t0::n180(s, out),
		['п', s @ ..] => super::t0::n181(s, out),
		['р', s @ ..] => super::t0::n182(s, out),
		['с', s @ ..] => super::t0::n183(s, out),
		['т', s @ ..] => super::t0::n184(s, out),
		['у', s @ ..] => super::t0::n185(s, out),
		['ц', s @ ..] => super::t0::n186(s, out),
		['ы', s @ ..] => super::t0::n187(s, out),
		['ь', s @ ..] => super::t0::n188(s, out),
		['э', s @ ..] => super::t0::n189(s, out),
		['я', s @ ..] => super::t0::n190(s, out),
		_ => false,
	}
}

/// Words ending -лий.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 1748);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 635);
					match s {
						[] => true,
						['о', 'к', 'о', 'р', 'к', '\u{1}', s @ ..] => {
							super::push(out, 1749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 636);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 637);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 1750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 194);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 195);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мий.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ний.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 639);
					match s {
						[] => true,
						['д', 'е', s @ ..] => {
							super::push(out, 1751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 640);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 1752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['р', 'е', 'с', '\u{1}', s @ ..] => {
							super::push(out, 1753);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 1754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 643);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 644);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 1756);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 10760);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 10761);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 17900);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 10762);
											match s {
												[] => true,
												['ш', 'о', 'р', 'п', 'а', s @ ..] => {
													super::push(out, 17901);
													s.is_empty()
												}
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
							super::push(out, 1757);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 4845);
									match s {
										[] => true,
										['о', 'г', '\u{1}', s @ ..] => {
											super::push(out, 10763);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 1758);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 10764);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 17902);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 10765);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 17903);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 201);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 645);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 646);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1759);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 1760);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 4846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 1761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 649);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 650);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['р', 'о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4847);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4848);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 10766);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 4849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 652);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4850);
									s.is_empty()
								}
								['а', 'з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10767);
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											super::push(out, 10768);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 10769);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 4851);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 4852);
									s.is_empty()
								}
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
			super::push(out, 203);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 1762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1764);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 1765);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4853);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 10770);
											match s {
												[] => true,
												['е', 'н', 'ш', 'и', 'в', '\u{1}', s @ ..] => {
													super::push(out, 17904);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 10771);
											match s {
												[] => true,
												['м', 'е', 'т', '\u{1}', s @ ..] => {
													super::push(out, 17905);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 10772);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 17906);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 656);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t0::n206(s, out),
		['м', s @ ..] => super::t0::n207(s, out),
		['н', s @ ..] => super::t0::n208(s, out),
		['о', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t0::n210(s, out),
		['т', s @ ..] => super::t0::n211(s, out),
		['у', s @ ..] => super::t0::n212(s, out),
		['х', s @ ..] => super::t0::n213(s, out),
		['ш', s @ ..] => super::t0::n214(s, out),
		['ь', s @ ..] => super::t0::n215(s, out),
		['я', s @ ..] => super::t0::n216(s, out),
		_ => false,
	}
}

/// Words ending -пий.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рий.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 679);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 680);
					match s {
						[] => true,
						['-', 'о', 'н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
							super::push(out, 1795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сий.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 220);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 683);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 684);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['с', 'о', 'л', '\u{1}', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тий.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 223);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уий.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 225);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фий.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 227);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хий.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['е', 'в', 'о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1798);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 1799);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 1800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 1803);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4897);
									s.is_empty()
								}
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
			super::push(out, 232);
			match s {
				[] => true,
				['р', 'б', 'о', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 1804);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', 'ч', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 10801);
											s.is_empty()
										}
										['ь', s @ ..] => {
											super::push(out, 10802);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 4898);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чий.
pub(crate) fn n69(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4899);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 4900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['у', 'к', s @ ..] => {
							super::push(out, 1805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', 'у', s @ ..] => {
					super::push(out, 693);
					match s {
						[] => true,
						['г', 'я', 'л', '\u{1}', s @ ..] => {
							super::push(out, 1806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 697);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1808);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 10803);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 10804);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 17919);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 699);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 1809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 700);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 4902);
									match s {
										[] => true,
										['я', 'л', '\u{1}', s @ ..] => {
											super::push(out, 10805);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 236);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['е', 'д', 'о', s @ ..] => {
							super::push(out, 1812);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4903);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 4904);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 10806);
											s.is_empty()
										}
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
					super::push(out, 703);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 704);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1814);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 1815);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 4906);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 10807);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 4907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 1816);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 4908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 705);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1817);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1818);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 1819);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 4909);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 10808);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1820);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 4910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 1821);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4911);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 10809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 1822);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 4912);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 10810);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 17920);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 4913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 1823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 706);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 707);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 1824);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 4914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 708);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 1825);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 4915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 709);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 710);
					s.is_empty()
				}
				['щ', s @ ..] => super::t0::n719(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['о', s @ ..] => super::t0::n720(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t0::n240(s, out),
		['о', s @ ..] => super::t0::n241(s, out),
		['п', s @ ..] => super::t0::n242(s, out),
		['с', s @ ..] => super::t0::n243(s, out),
		['т', s @ ..] => super::t0::n244(s, out),
		['у', s @ ..] => super::t0::n245(s, out),
		['ы', s @ ..] => super::t0::n246(s, out),
		['ю', s @ ..] => super::t0::n247(s, out),
		['я', s @ ..] => super::t0::n248(s, out),
		_ => false,
	}
}

/// Words ending -ший.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 248);
			match s {
				[] => true,
				['е', 'р', 'г', 'ы', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t1::n251(s, out),
		['г', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 1966);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 5472);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 11986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 1967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', s @ ..] => {
					super::push(out, 764);
					match s {
						[] => true,
						['т', 'с', 'ы', s @ ..] => {
							super::push(out, 1968);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5473);
									s.is_empty()
								}
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
			super::push(out, 251);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', 'ы', s @ ..] => {
							super::push(out, 1969);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 5475);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 5476);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 11987);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 252);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', 'ы', s @ ..] => {
							super::push(out, 1970);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'ы', s @ ..] => {
							super::push(out, 1971);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'о', 'п', 'ы', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'р', 'г', 'ы', s @ ..] => {
					super::push(out, 767);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t1::n256(s, out),
		['к', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						['п', 'ы', s @ ..] => {
							super::push(out, 1985);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'ы', s @ ..] => {
							super::push(out, 1986);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5509);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'ы', s @ ..] => {
							super::push(out, 1987);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 5510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 771);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 772);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 1988);
							match s {
								[] => true,
								['о', 'в', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 12036);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 19513);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 12037);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 19514);
													s.is_empty()
												}
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
							super::push(out, 1989);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 5511);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 12038);
											s.is_empty()
										}
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
					super::push(out, 773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['о', 'х', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 12039);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 12040);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 19515);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 5512);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 12041);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 257);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['е', s @ ..] => super::t1::n790(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['а', 'п', 'ы', s @ ..] => {
					super::push(out, 778);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'н', 'ы', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 780);
					s.is_empty()
				}
				['о', 'р', 'ы', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t1::n262(s, out),
		['у', s @ ..] => super::t1::n263(s, out),
		['х', s @ ..] => super::t1::n264(s, out),
		['ч', s @ ..] => super::t1::n265(s, out),
		['щ', s @ ..] => super::t1::n266(s, out),
		['ь', s @ ..] => super::t1::n267(s, out),
		['ю', s @ ..] => super::t1::n268(s, out),
		_ => false,
	}
}

/// Words ending -щий.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2003);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 5527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 790);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2005);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 5528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 2006);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 5529);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 12057);
											match s {
												[] => true,
												['а', 'н', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n26929(s, out),
														['е', s @ ..] => super::t1::n26930(s, out),
														['и', s @ ..] => super::t1::n26931(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 19518);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', 'и', 'л', 'б', '\u{1}', s @ ..] => {
											super::push(out, 12058);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 5530);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12059);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 5531);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 12060);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 19519);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => super::t1::n2078(s, out),
						['о', s @ ..] => super::t1::n2079(s, out),
						['р', s @ ..] => {
							super::push(out, 2009);
							s.is_empty()
						}
						['у', s @ ..] => super::t1::n2081(s, out),
						['ы', s @ ..] => {
							super::push(out, 2011);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 5539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => super::t1::n2083(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t1::n804(s, out),
				['ч', s @ ..] => super::t1::n805(s, out),
				['ш', s @ ..] => super::t1::n806(s, out),
				['щ', s @ ..] => super::t1::n807(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t1::n270(s, out),
		['е', s @ ..] => super::t1::n271(s, out),
		['и', s @ ..] => super::t1::n272(s, out),
		['о', s @ ..] => super::t1::n273(s, out),
		['у', s @ ..] => super::t1::n274(s, out),
		['ю', s @ ..] => super::t1::n275(s, out),
		['я', s @ ..] => super::t1::n276(s, out),
		_ => false,
	}
}

/// Words ending --заводской.
pub(crate) fn n6341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6080);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лой.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 866);
					match s {
						[] => true,
						['у', '\u{1}', s @ ..] => {
							super::push(out, 2315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 868);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 869);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 2316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 870);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 295);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мой.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['н', 'о', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 2318);
							match s {
								[] => true,
								['у', 'л', 'г', 'о', s @ ..] => {
									super::push(out, 6081);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 12582);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 297);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 298);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2319);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2320);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 6082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ной.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['а', 'т', 'с', 'е', 'р', 'е', 'п', '-', 'о', 'н', 'м', 'е', 'ъ', 'б', 'о', '\u{1}', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['л', 'к', 'а', s @ ..] => {
							super::push(out, 2321);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 6083);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 6084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['ъ', s @ ..] => {
									super::push(out, 6085);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 6086);
									match s {
										[] => true,
										['в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 19907);
													s.is_empty()
												}
												['е', 'н', '\u{1}', s @ ..] => {
													super::push(out, 19908);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 878);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 2322);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 2324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6087);
									s.is_empty()
								}
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
			super::push(out, 303);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 304);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['р', 'а', s @ ..] => {
							super::push(out, 2325);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 6088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 882);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2326);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2327);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 6089);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 12583);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 305);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 884);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2328);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 6090);
									s.is_empty()
								}
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
			super::push(out, 307);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 885);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 2329);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 6091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12584);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 308);
			match s {
				[] => true,
				['е', 'т', 'с', 'о', 'с', 'е', 'л', '-', 'о', 'н', 'ж', 'е', 'а', 'т', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 309);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['в', 'с', s @ ..] => {
							super::push(out, 2330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6092);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 6093);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 12585);
											s.is_empty()
										}
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
					super::push(out, 889);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 891);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 892);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2331);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6094);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 6095);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 12586);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 19909);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 12587);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 19910);
													s.is_empty()
												}
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
							super::push(out, 2332);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 2333);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 6096);
									s.is_empty()
								}
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
			super::push(out, 310);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 893);
					match s {
						[] => true,
						['п', 'е', s @ ..] => {
							super::push(out, 2334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 311);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 312);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 894);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 895);
					match s {
						[] => true,
						['р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 2335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['е', 'р', '-', 'о', 'н', 'р', 'е', 'з', 'о', '\u{1}', s @ ..] => {
					super::push(out, 896);
					s.is_empty()
				}
				['о', 'л', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['е', 'м', s @ ..] => {
							super::push(out, 2336);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6097);
									s.is_empty()
								}
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
			super::push(out, 314);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['м', 'с', 'е', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 6098);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 6099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 899);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 900);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 315);
			match s {
				[] => true,
				['л', s @ ..] => super::t1::n918(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['д', s @ ..] => super::t1::n919(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пой.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 317);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 902);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['к', 'с', '\u{1}', s @ ..] => {
					super::push(out, 903);
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 904);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рой.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['с', 'у', s @ ..] => {
					super::push(out, 905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сой.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 906);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -той.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 321);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 322);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 908);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2341);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 6100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 909);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 2342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 910);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 2343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 914);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2344);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 6101);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2345);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12590);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 12591);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 19912);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n27177(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 915);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6102);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 6103);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 12592);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6104);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 6105);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 12593);
											match s {
												[] => true,
												['о', 'п', '\u{1}', s @ ..] => {
													super::push(out, 19913);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 325);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 916);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 2347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 917);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6106);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 6107);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 12594);
											s.is_empty()
										}
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
					super::push(out, 918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 2348);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12595);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 12596);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 2349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хой.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2350);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 2351);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 6108);
									s.is_empty()
								}
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
			super::push(out, 328);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 919);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6109);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 6110);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 12597);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 19914);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 329);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['л', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2352);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2353);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 6111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 921);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шой.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['л', 'о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2355);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щой.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 922);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -адколобый.
pub(crate) fn n20925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19915);
	s.is_empty()
}

/// Words ending -зубый.
pub(crate) fn n952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 928);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6120);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6121);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2366);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2367);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6122);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6123);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6124);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6125);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2368);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6126);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2369);
			s.is_empty()
		}
		['х', 'е', s @ ..] => {
			super::push(out, 2370);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6127);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 12612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вый.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2373);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 2374);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 6129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 2375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 934);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2376);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 6130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 2377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 6131);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 6132);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 12613);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 2379);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 6133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6134);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 6135);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 6136);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 12614);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 19916);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 12615);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 19917);
													match s {
														[] => true,
														['ц', s @ ..] => super::t1::n27178(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 19918);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 6137);
									match s {
										[] => true,
										['т', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 19919);
													match s {
														[] => true,
														['л', s @ ..] => super::t1::n27179(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 19920);
													match s {
														[] => true,
														['л', s @ ..] => super::t1::n27180(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 19921);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', 'в', 'д', '\u{1}', s @ ..] => {
									super::push(out, 6138);
									s.is_empty()
								}
								['х', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 12616);
											match s {
												[] => true,
												['р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 19922);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 12617);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 937);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2381);
							s.is_empty()
						}
						['г', 'а', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 2382);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 2383);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 6139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 2384);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 6140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 2385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6141);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 6142);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 12618);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 6143);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 12619);
											s.is_empty()
										}
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
					super::push(out, 939);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 2386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['ш', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 2387);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 2388);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t1::n341(s, out),
		['з', s @ ..] => super::t1::n342(s, out),
		['и', s @ ..] => super::t1::n343(s, out),
		['о', s @ ..] => super::t1::n344(s, out),
		['р', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t1::n346(s, out),
		['ю', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t1::n348(s, out),
		_ => false,
	}
}

/// Words ending -дый.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['з', 'о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 2736);
							s.is_empty()
						}
						['н', 'ш', s @ ..] => {
							super::push(out, 2737);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 6841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 2738);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 6842);
									s.is_empty()
								}
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
			super::push(out, 348);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1000);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 2739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 2740);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6843);
									s.is_empty()
								}
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
			super::push(out, 349);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1002);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 2741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['р', 'о', 'б', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 2743);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 6844);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 6845);
									s.is_empty()
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 13373);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 13374);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 13375);
											s.is_empty()
										}
										['н', 'и', 'л', 'д', '\u{1}', s @ ..] => {
											super::push(out, 13376);
											s.is_empty()
										}
										['р', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 13377);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 13378);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 351);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1004);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 2744);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13379);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 13380);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 20440);
													match s {
														[] => true,
														['н', 'з', 'а', 'м', 'л', 'а', '\u{1}', s @ ..] => super::t1::n27419(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 13381);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 13382);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 20441);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 13383);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 13384);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 2745);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 13385);
											s.is_empty()
										}
										['к', 'г', s @ ..] => {
											super::push(out, 13386);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 13387);
											s.is_empty()
										}
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
					super::push(out, 1005);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2746);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 2747);
							match s {
								[] => true,
								['о', 'л', s @ ..] => {
									super::push(out, 6847);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 13388);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 20442);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 352);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2748);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 2749);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 6848);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 13389);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 13390);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 13391);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 20443);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зый.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 353);
			match s {
				[] => true,
				['л', 'г', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 6849);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 6850);
									match s {
										[] => true,
										['у', 'п', '\u{1}', s @ ..] => {
											super::push(out, 13392);
											s.is_empty()
										}
										['я', s @ ..] => {
											super::push(out, 13393);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 2750);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 6851);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 6852);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 6853);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 13394);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 13395);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 20444);
													s.is_empty()
												}
												_ => false,
											}
										}
										['б', s @ ..] => {
											super::push(out, 13396);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 20445);
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
											super::push(out, 13397);
											s.is_empty()
										}
										['м', 'у', s @ ..] => {
											super::push(out, 13398);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 13399);
											s.is_empty()
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 20446);
													s.is_empty()
												}
												['я', s @ ..] => {
													super::push(out, 20447);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 6854);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 13400);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 6855);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 6856);
									s.is_empty()
								}
								['т', 'а', s @ ..] => {
									super::push(out, 6857);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 13401);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 2751);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 6858);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 2752);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 13402);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 20448);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 13403);
											match s {
												[] => true,
												['т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 20449);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 6859);
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
						['о', s @ ..] => {
							super::push(out, 2753);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 6860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 2754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1008);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 2756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 1009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лый.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1010);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 1011);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2758);
							match s {
								[] => true,
								['д', 'о', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13404);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 13405);
											match s {
												[] => true,
												['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 20450);
													s.is_empty()
												}
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
							super::push(out, 2759);
							match s {
								[] => true,
								['б', 'е', s @ ..] => {
									super::push(out, 6861);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 13406);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 20451);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1013);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2760);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 6862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 2761);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6863);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 6864);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 13407);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 20452);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 13408);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 20453);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1014);
					match s {
						[] => true,
						['е', 'л', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6865);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 6866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 2762);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2763);
							s.is_empty()
						}
						['у', 'м', 'з', 'о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 2764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2765);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 2766);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 6867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2767);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 2768);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 2769);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 6868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 2770);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 13409);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 13410);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 6869);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 2771);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 2772);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 6870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 2773);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 13411);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 20454);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 13412);
											match s {
												[] => true,
												['т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 20455);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2774);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 2775);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 6871);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 6872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2776);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 2777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 1020);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1021);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 2779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1022);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1023);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 1024);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 2781);
							s.is_empty()
						}
						['р', s @ ..] => super::t1::n2884(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t1::n362(s, out),
		['д', s @ ..] => super::t1::n363(s, out),
		['е', s @ ..] => super::t1::n364(s, out),
		['з', s @ ..] => super::t1::n365(s, out),
		['и', s @ ..] => super::t1::n366(s, out),
		['к', s @ ..] => super::t1::n367(s, out),
		['о', s @ ..] => super::t1::n368(s, out),
		['п', s @ ..] => super::t1::n369(s, out),
		['р', s @ ..] => super::t1::n370(s, out),
		['с', s @ ..] => super::t1::n371(s, out),
		['т', s @ ..] => super::t1::n372(s, out),
		['у', s @ ..] => super::t1::n373(s, out),
		['х', s @ ..] => super::t1::n374(s, out),
		['ш', s @ ..] => super::t1::n375(s, out),
		['ы', s @ ..] => super::t1::n376(s, out),
		['я', s @ ..] => super::t1::n377(s, out),
		_ => false,
	}
}

/// Words ending -мый.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 1089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t1::n379(s, out),
		['и', s @ ..] => super::t1::n380(s, out),
		['о', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7271);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 7272);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13752);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 13753);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 13754);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 20737);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 13755);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 20738);
													s.is_empty()
												}
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
							super::push(out, 3028);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 7273);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 13756);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 3029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1121);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 7274);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13757);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 7275);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 3030);
							match s {
								[] => true,
								['р', 'з', s @ ..] => {
									super::push(out, 7276);
									match s {
										[] => true,
										['и', 'е', 'н', '\u{1}', s @ ..] => {
											super::push(out, 13758);
											s.is_empty()
										}
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
					super::push(out, 1123);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 3031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1126);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ный.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3032);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7277);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 7278);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 13759);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 3033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3034);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 7279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 3035);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 7280);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 13760);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3036);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 7281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3038);
							match s {
								[] => true,
								['т', 'с', '\u{1}', s @ ..] => {
									super::push(out, 7282);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 3039);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 7283);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 13761);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 20739);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 7284);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 13762);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3040);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 7285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 3041);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 7286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 1130);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3042);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 7287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3043);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 7288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 3044);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 3045);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 7289);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 13763);
											s.is_empty()
										}
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
					super::push(out, 1132);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 3046);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 7290);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 13764);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 7291);
									match s {
										[] => true,
										['к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
											super::push(out, 13765);
											s.is_empty()
										}
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
					super::push(out, 1133);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3047);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 3048);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 7292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1134);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3050);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 7293);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 13766);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 7294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1136);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 3051);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 3052);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1137);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 3054);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 7296);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 7297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 3055);
							match s {
								[] => true,
								['п', 'е', 'л', s @ ..] => {
									super::push(out, 7298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t1::n1169(s, out),
				['х', s @ ..] => {
					super::push(out, 1139);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 3059);
							match s {
								[] => true,
								['п', 'е', s @ ..] => {
									super::push(out, 7301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => super::t1::n1171(s, out),
				['ш', s @ ..] => super::t1::n1172(s, out),
				['щ', s @ ..] => super::t1::n1173(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t1::n385(s, out),
		['в', s @ ..] => super::t1::n386(s, out),
		['г', s @ ..] => super::t1::n387(s, out),
		['д', s @ ..] => super::t1::n388(s, out),
		['е', s @ ..] => super::t1::n389(s, out),
		['ж', s @ ..] => super::t1::n390(s, out),
		['з', s @ ..] => super::t1::n391(s, out),
		['и', s @ ..] => super::t1::n392(s, out),
		['й', s @ ..] => super::t1::n393(s, out),
		['к', s @ ..] => super::t1::n394(s, out),
		['л', s @ ..] => super::t1::n395(s, out),
		['м', s @ ..] => super::t1::n396(s, out),
		['н', s @ ..] => super::t1::n397(s, out),
		['о', s @ ..] => super::t1::n398(s, out),
		['п', s @ ..] => super::t1::n399(s, out),
		['р', s @ ..] => super::t1::n400(s, out),
		['с', s @ ..] => super::t1::n401(s, out),
		['т', s @ ..] => super::t1::n402(s, out),
		['ф', s @ ..] => super::t1::n403(s, out),
		['ц', s @ ..] => super::t1::n404(s, out),
		['ч', s @ ..] => super::t1::n405(s, out),
		['ш', s @ ..] => super::t1::n406(s, out),
		['щ', s @ ..] => super::t1::n407(s, out),
		['ь', s @ ..] => super::t1::n408(s, out),
		['ю', s @ ..] => super::t1::n409(s, out),
		['я', s @ ..] => super::t2::n410(s, out),
		_ => false,
	}
}

/// Words ending -пый.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4047);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9472);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 9473);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 16612);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 9474);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 9475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 4048);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 9476);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													super::push(out, 23412);
													s.is_empty()
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 23413);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 410);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4049);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['ж', 'о', 'н', 'с', s @ ..] => {
					super::push(out, 1343);
					match s {
						[] => true,
						['а', 'р', 'к', '\u{1}', s @ ..] => {
							super::push(out, 4051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9477);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 9478);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 16613);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 16614);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													super::push(out, 23414);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рый.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 1346);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1348);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4054);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9479);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 9480);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 16615);
											s.is_empty()
										}
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
					super::push(out, 1349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1350);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4055);
							match s {
								[] => true,
								['х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 16616);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 16617);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 23415);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1351);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9481);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 9482);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 16618);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 23416);
													s.is_empty()
												}
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
							super::push(out, 4057);
							match s {
								[] => true,
								['о', 'х', '\u{1}', s @ ..] => {
									super::push(out, 9483);
									s.is_empty()
								}
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
			super::push(out, 415);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1352);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4058);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 9484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4059);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 4060);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9486);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9487);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 16619);
											s.is_empty()
										}
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
					super::push(out, 1353);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4061);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1354);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4062);
							match s {
								[] => true,
								['о', 'т', 'о', s @ ..] => {
									super::push(out, 9489);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 16620);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 4063);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9490);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 9491);
									s.is_empty()
								}
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
			super::push(out, 416);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4064);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 9492);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 9493);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 9494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4065);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 4066);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 16621);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 23417);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 16622);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 23418);
													match s {
														[] => true,
														['л', 'е', 'з', '\u{1}', s @ ..] => super::t2::n29641(s, out),
														_ => false,
													}
												}
												['м', 'е', 'т', '\u{1}', s @ ..] => {
													super::push(out, 23419);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 16623);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 23420);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1357);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 1358);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 1359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1360);
					match s {
						[] => true,
						['м', s @ ..] => super::t2::n4216(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t2::n421(s, out),
		['т', s @ ..] => super::t2::n422(s, out),
		['у', s @ ..] => super::t2::n423(s, out),
		['ы', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['д', s @ ..] => super::t2::n1405(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t2::n425(s, out),
		_ => false,
	}
}

/// Words ending -сый.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 9509);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 4084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 9511);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 9512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 4085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4087);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 9513);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1375);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9514);
									match s {
										[] => true,
										['е', s @ ..] => {
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 23427);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 23428);
													match s {
														[] => true,
														['и', s @ ..] => super::t2::n29645(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 16637);
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 23429);
													match s {
														[] => true,
														['а', s @ ..] => super::t2::n29646(s, out),
														_ => false,
													}
												}
												['д', s @ ..] => {
													super::push(out, 23430);
													s.is_empty()
												}
												['к', s @ ..] => {
													match s {
														[] => true,
														['д', 'а', s @ ..] => super::t2::n29647(s, out),
														['т', s @ ..] => super::t2::n29648(s, out),
														_ => false,
													}
												}
												['л', s @ ..] => super::t2::n24626(s, out),
												['н', s @ ..] => super::t2::n24627(s, out),
												['т', s @ ..] => super::t2::n24628(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['г', s @ ..] => super::t2::n9974(s, out),
								['к', s @ ..] => super::t2::n9975(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t2::n1412(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t2::n429(s, out),
		['ы', s @ ..] => super::t2::n430(s, out),
		['я', 'м', 'о', s @ ..] => super::t2::n431(s, out),
		_ => false,
	}
}

/// Words ending -тый.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['а', s @ ..] => super::t2::n432(s, out),
		['е', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4158);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4159);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4160);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4161);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 9599);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 16723);
											s.is_empty()
										}
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
					super::push(out, 1404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4162);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4164);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4165);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9600);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9601);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1405);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9603);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 9604);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9605);
									s.is_empty()
								}
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
			super::push(out, 431);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['б', s @ ..] => super::t2::n1443(s, out),
				['в', s @ ..] => super::t2::n1444(s, out),
				['д', s @ ..] => {
					super::push(out, 1409);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t2::n1446(s, out),
				['л', s @ ..] => super::t2::n1447(s, out),
				['м', s @ ..] => {
					super::push(out, 1412);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4206);
							match s {
								[] => true,
								['м', 'а', s @ ..] => {
									super::push(out, 9641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t2::n1450(s, out),
				['р', s @ ..] => super::t2::n1451(s, out),
				['т', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => super::t2::n1454(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t2::n436(s, out),
		['о', s @ ..] => super::t2::n437(s, out),
		['р', s @ ..] => super::t2::n438(s, out),
		['с', s @ ..] => super::t2::n439(s, out),
		['у', s @ ..] => super::t2::n440(s, out),
		['ы', s @ ..] => super::t2::n441(s, out),
		['ю', s @ ..] => super::t2::n442(s, out),
		['я', s @ ..] => super::t2::n443(s, out),
		_ => false,
	}
}

/// Words ending -цый.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 441);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4353);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 9908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9909);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 16982);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 23664);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 16983);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 9910);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 16984);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 23665);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 16985);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 16986);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 9911);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 16987);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 16988);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 23666);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 16989);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 16990);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 16991);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 9912);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 16992);
											match s {
												[] => true,
												['е', s @ ..] => {
													match s {
														[] => true,
														['л', s @ ..] => super::t2::n29767(s, out),
														['м', s @ ..] => super::t2::n29768(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 16993);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 16994);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 16995);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 23667);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ч', s @ ..] => {
											super::push(out, 16996);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 23668);
													s.is_empty()
												}
												_ => false,
											}
										}
										['я', s @ ..] => {
											super::push(out, 16997);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 23669);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 9913);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 16998);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 4354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1453);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йся.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['и', s @ ..] => super::t2::n486(s, out),
		_ => false,
	}
}

/// Words ending -стыжий.
pub(crate) fn n1552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1511);
	s.is_empty()
}

/// Words ending -южий.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 522);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1512);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1513);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 4407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1514);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжий.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 163);
	s.is_empty()
}

/// Words ending -екий.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10003);
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									super::push(out, 10004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1536);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4428);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', 'п', s @ ..] => {
									super::push(out, 10005);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17202);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 10006);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 17203);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', 'л', s @ ..] => {
							super::push(out, 4429);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 10007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лежкий.
pub(crate) fn n1578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1537);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4430);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 10008);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рожкий.
pub(crate) fn n1579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1538);
	s.is_empty()
}

/// Words ending -яжкий.
pub(crate) fn n555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 550);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 1539);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зкий.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10009);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 10010);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 4433);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 10011);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17204);
											s.is_empty()
										}
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
					super::push(out, 1541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4434);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 4435);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 10012);
									s.is_empty()
								}
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
			super::push(out, 553);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4436);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 4437);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 10013);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10014);
									s.is_empty()
								}
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
			super::push(out, 554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1542);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['л', 'о', 'к', 'с', 'е', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -икий.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1546);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1547);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1548);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4440);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10015);
									s.is_empty()
								}
								['е', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 17205);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 17206);
											match s {
												[] => true,
												['п', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n30329(s, out),
														['е', s @ ..] => super::t2::n30330(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 17207);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 10016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 4441);
							match s {
								[] => true,
								['н', 'л', 'о', 'с', '\u{1}', s @ ..] => {
									super::push(out, 10017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 4442);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 10018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1549);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1550);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4443);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 10019);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 17208);
											s.is_empty()
										}
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
					super::push(out, 1551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йкий.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['л', 'к', '\u{1}', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 560);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1554);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10020);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 10021);
									match s {
										[] => true,
										['н', 'г', 'о', '\u{1}', s @ ..] => {
											super::push(out, 17209);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 10022);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17210);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 17211);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 24130);
													s.is_empty()
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 17212);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 17213);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 17214);
											match s {
												[] => true,
												['е', 'р', 'п', 'у', '\u{1}', s @ ..] => {
													super::push(out, 24131);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 17215);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 17216);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 24132);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', 'е', s @ ..] => {
											super::push(out, 17217);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 24133);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 17218);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 24134);
													match s {
														[] => true,
														['н', 'з', 'и', '\u{1}', s @ ..] => super::t2::n30331(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 17219);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 24135);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкий.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 561);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 1556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 562);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1559);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1560);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мкий.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1561);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', s @ ..] => {
							super::push(out, 4448);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4449);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 4450);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 10023);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 17220);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 566);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 1562);
					s.is_empty()
				}
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4451);
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							super::push(out, 4452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нкий.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1564);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1565);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окий.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1567);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4455);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4456);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 10024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10025);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 10026);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 17221);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1568);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4458);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17222);
											s.is_empty()
										}
										['е', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 24136);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n30332(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 24137);
													match s {
														[] => true,
														['п', s @ ..] => super::t2::n30333(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 570);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 1569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 572);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1571);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1574);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1575);
					match s {
						[] => true,
						['ш', 'е', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 10027);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 10028);
									s.is_empty()
								}
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
			super::push(out, 574);
			match s {
				[] => true,
				['ы', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4460);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 4461);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 10029);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 4462);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 10030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4463);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 10031);
									s.is_empty()
								}
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
			super::push(out, 575);
			match s {
				[] => true,
				['с', 'е', 'ж', '\u{1}', s @ ..] => {
					super::push(out, 1576);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пкий.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1577);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 4464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 1578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 579);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркий.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4466);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 10032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', '\u{1}', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4467);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 4469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ский.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 586);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['л', 'а', s @ ..] => {
							super::push(out, 4470);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 10033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4471);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4472);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 10034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4473);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 10035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t2::n593(s, out),
		['г', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1598);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1599);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4516);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 10185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 4517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10186);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 17424);
											match s {
												[] => true,
												['ч', 'н', 'е', s @ ..] => {
													super::push(out, 24274);
													match s {
														[] => true,
														['й', s @ ..] => super::t2::n30361(s, out),
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
									super::push(out, 10187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1601);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4519);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 4520);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 10188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4521);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 10189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 4522);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['н', 'р', 'ю', 'н', '\u{1}', s @ ..] => {
											super::push(out, 17425);
											s.is_empty()
										}
										['с', 'г', 'и', 'н', 'е', 'к', '\u{1}', s @ ..] => {
											super::push(out, 17426);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4523);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 17427);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 24275);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 17428);
											match s {
												[] => true,
												['т', 'е', 'г', '\u{1}', s @ ..] => {
													super::push(out, 24276);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 17429);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 24277);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 10190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 4524);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 17430);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 17431);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 24278);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 24279);
													match s {
														[] => true,
														['с', s @ ..] => super::t2::n30362(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 24280);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 17432);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 17433);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 24281);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 17434);
											s.is_empty()
										}
										['ц', s @ ..] => {
											super::push(out, 17435);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 24282);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1603);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 1604);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 1605);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 4525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t2::n595(s, out),
		['е', s @ ..] => super::t2::n596(s, out),
		['ж', s @ ..] => super::t2::n597(s, out),
		['з', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1626);
					s.is_empty()
				}
				['у', 'я', s @ ..] => {
					super::push(out, 1627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t2::n599(s, out),
		['к', s @ ..] => super::t2::n600(s, out),
		['л', s @ ..] => {
			super::push(out, 596);
			s.is_empty()
		}
		['м', s @ ..] => super::t2::n602(s, out),
		['н', s @ ..] => super::t2::n603(s, out),
		['о', s @ ..] => super::t2::n604(s, out),
		['п', s @ ..] => super::t2::n605(s, out),
		['р', s @ ..] => super::t2::n606(s, out),
		['с', s @ ..] => super::t2::n607(s, out),
		['т', s @ ..] => super::t2::n608(s, out),
		['у', s @ ..] => super::t2::n609(s, out),
		['ф', s @ ..] => super::t2::n610(s, out),
		['х', s @ ..] => super::t2::n611(s, out),
		['ц', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['н', s @ ..] => super::t2::n1756(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t2::n613(s, out),
		['ш', s @ ..] => super::t2::n614(s, out),
		['ь', s @ ..] => super::t2::n615(s, out),
		['я', s @ ..] => super::t2::n616(s, out),
		_ => false,
	}
}

/// Words ending -ткий.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 612);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1714);
					match s {
						[] => true,
						['х', '\u{1}', s @ ..] => {
							super::push(out, 4792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1716);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4793);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 10666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4794);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4796);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4797);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 10667);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17795);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 17796);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 24430);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 615);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1721);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 4798);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10668);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 10669);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 17797);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 4799);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 10670);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 10671);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17798);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 617);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -укий.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4800);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 4801);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 10672);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 17799);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 10673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'с', s @ ..] => {
							super::push(out, 4802);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 10674);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 17800);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цкий.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 620);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1724);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 4803);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 10675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'т', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 621);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 1727);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 1728);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4804);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 10676);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 17801);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 10677);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 17802);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 10678);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 4805);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1729);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 4807);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 10679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 4808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 622);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1731);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 1732);
					match s {
						[] => true,
						['у', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 10680);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 10681);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17803);
											s.is_empty()
										}
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
					super::push(out, 1733);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4810);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 10682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1734);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['р', 'е', s @ ..] => {
							super::push(out, 4812);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 10683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4813);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 10684);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4814);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 4815);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 10685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 4816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 4817);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 10686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 4818);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 10687);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4819);
							match s {
								[] => true,
								['о', 'п', s @ ..] => {
									super::push(out, 10688);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 17804);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 4820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 1740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'л', s @ ..] => {
			super::push(out, 623);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 624);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 625);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 626);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкий.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['я', 'о', 'л', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ький.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['н', s @ ..] => super::t2::n634(s, out),
		['р', s @ ..] => {
			super::push(out, 630);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 4842);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экий.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	s.is_empty()
}

/// Words ending -який.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 631);
			match s {
				[] => true,
				['в', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4843);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 632);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йний.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 658);
			match s {
				[] => true,
				['р', 'к', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4854);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 10773);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4855);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 10774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мний.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 659);
			match s {
				[] => true,
				['з', 'д', s @ ..] => {
					super::push(out, 1766);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нний.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 660);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1768);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4857);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 10775);
									match s {
										[] => true,
										['и', 'е', s @ ..] => {
											super::push(out, 17907);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 4858);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 10776);
									match s {
										[] => true,
										['ы', 'в', '\u{1}', s @ ..] => {
											super::push(out, 17908);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4859);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 10777);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 17909);
											s.is_empty()
										}
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
					super::push(out, 1769);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4860);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4861);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 10779);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 10780);
									s.is_empty()
								}
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
			super::push(out, 662);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1770);
					match s {
						[] => true,
						['о', 'т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10781);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 10782);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10783);
									match s {
										[] => true,
										['в', 'и', s @ ..] => {
											super::push(out, 17910);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 24501);
													s.is_empty()
												}
												_ => false,
											}
										}
										['г', s @ ..] => {
											super::push(out, 17911);
											s.is_empty()
										}
										['н', s @ ..] => {
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 24502);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 24503);
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
										['в', s @ ..] => {
											super::push(out, 17912);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 17913);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 10784);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 17914);
											match s {
												[] => true,
												['р', s @ ..] => {
													match s {
														[] => true,
														['т', s @ ..] => super::t2::n30474(s, out),
														['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => super::t2::n30475(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 17915);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рний.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['ч', 'е', 'в', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 4862);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тний.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4864);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 4865);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 10785);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 17916);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 4866);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 4867);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 10786);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 10787);
									match s {
										[] => true,
										['н', 'е', 'ш', 'р', 'е', 'в', 'о', 'с', 'е', s @ ..] => {
											super::push(out, 17917);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'о', 'с', 'х', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 10788);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 10789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 4868);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24504);
													s.is_empty()
												}
												['и', 'т', 'я', 'с', 'е', 'д', 'и', 'м', 'ь', 'с', 'о', 'в', '\u{1}', s @ ..] => {
													super::push(out, 24505);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 17918);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 667);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1773);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уний.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 668);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хний.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['и', '\u{1}', s @ ..] => {
			super::push(out, 669);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 670);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шний.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 671);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1774);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4870);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 10790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 4871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1775);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1776);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4873);
							match s {
								[] => true,
								['ч', 'в', 'а', s @ ..] => {
									super::push(out, 10791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4874);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 1777);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 4875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 672);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1778);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4876);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4877);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 10793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1779);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1781);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4880);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 10794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1782);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4881);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4882);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 10795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 4883);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 10796);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1783);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4884);
							match s {
								[] => true,
								['п', 'е', 'т', '\u{1}', s @ ..] => {
									super::push(out, 10797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4885);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4886);
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
				['л', s @ ..] => {
					super::push(out, 1784);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4887);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4888);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 10799);
									s.is_empty()
								}
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
			super::push(out, 674);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['е', '\u{1}', s @ ..] => {
							super::push(out, 4889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1786);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1788);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4893);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 10800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 4894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 1791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьний.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['а', 'д', 'х', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яний.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 1793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щичий.
pub(crate) fn n719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 711);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1827);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1828);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1829);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 4918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1831);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4920);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олчий.
pub(crate) fn n720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 712);
	s.is_empty()
}

/// Words ending -нчий.
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -очий.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 714);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 716);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1834);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4921);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 10811);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 4922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 717);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1835);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1836);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 4923);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 1837);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 1838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 1839);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 719);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пчий.
pub(crate) fn n242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 240);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -счий.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тчий.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 722);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -учий.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 1841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 724);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1843);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 10812);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1844);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4925);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 10814);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 4926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1845);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 4927);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 1846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 725);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 1848);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 726);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1851);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 1852);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1853);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 4930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 727);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 728);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1856);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 4931);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4932);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10815);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 10816);
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
						['е', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 17921);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 17922);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 10817);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 4933);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 731);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1859);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 1860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 732);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 733);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 1861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 734);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1862);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 1863);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 735);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1864);
					s.is_empty()
				}
				['е', 'л', 'е', s @ ..] => {
					super::push(out, 1865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 736);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 737);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ычий.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 738);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ючий.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 739);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 740);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 1867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 741);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 742);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1870);
					match s {
						[] => true,
						['г', 'е', s @ ..] => {
							super::push(out, 4939);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 10818);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячий.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 743);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 744);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 1872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 745);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1874);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 4940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1875);
					match s {
						[] => true,
						['р', 'б', '\u{1}', s @ ..] => {
							super::push(out, 4941);
							s.is_empty()
						}
						['х', '\u{1}', s @ ..] => {
							super::push(out, 4942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 747);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 748);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 749);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 1876);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 750);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 1879);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 4944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 1880);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4945);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 10819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1881);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 4946);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 10820);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 17923);
											s.is_empty()
										}
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
					super::push(out, 1882);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 4947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1883);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 4948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1884);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1885);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4950);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1886);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 4952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['р', 'е', 'в', s @ ..] => {
					super::push(out, 1887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 754);
			s.is_empty()
		}
		_ => false,
	}
}

