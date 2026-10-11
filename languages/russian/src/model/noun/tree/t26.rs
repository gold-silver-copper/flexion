use flexion::Path;

/// Words ending -орт.
pub(crate) fn n3811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3800);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16944);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40368);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 40369);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68429);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 93964);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113134);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125496);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n134330(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 40370);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 40371);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68430);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 93965);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113135);
											s.is_empty()
										}
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
					super::push(out, 40372);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68431);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93966);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 113136);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125497);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 40373);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68432);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 93967);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 113137);
											s.is_empty()
										}
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
					super::push(out, 40374);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 68433);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 93968);
									s.is_empty()
								}
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
			super::push(out, 16945);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40375);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 40376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 16946);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40377);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 68434);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 93969);
									s.is_empty()
								}
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
			super::push(out, 16947);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40378);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40379);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68435);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 68436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 40380);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 40381);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68438);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 68439);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 68440);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 93971);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113138);
											match s {
												[] => true,
												['ю', s @ ..] => super::t38::n126797(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t38::n69161(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t38::n40791(s, out),
				['н', s @ ..] => super::t38::n40792(s, out),
				['о', s @ ..] => super::t38::n40793(s, out),
				['п', s @ ..] => super::t38::n40794(s, out),
				['р', s @ ..] => super::t38::n40795(s, out),
				['с', s @ ..] => super::t38::n40796(s, out),
				['т', s @ ..] => super::t38::n40797(s, out),
				['ф', s @ ..] => super::t38::n40798(s, out),
				['ч', s @ ..] => super::t38::n40799(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t38::n17047(s, out),
		['с', s @ ..] => super::t39::n17048(s, out),
		['т', s @ ..] => super::t39::n17049(s, out),
		['ф', s @ ..] => super::t39::n17050(s, out),
		['х', s @ ..] => super::t39::n17051(s, out),
		['ч', s @ ..] => super::t39::n17052(s, out),
		['ш', s @ ..] => super::t39::n17053(s, out),
		_ => false,
	}
}

/// Words ending -урт.
pub(crate) fn n3812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3801);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16955);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16956);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40403);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40404);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 68466);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 93999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 40405);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68467);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 40406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 16957);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40407);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40408);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68468);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 68469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 40409);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68470);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 94001);
									s.is_empty()
								}
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
			super::push(out, 16958);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40410);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 68471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 16959);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 16960);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40412);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68472);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94002);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113159);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 125509);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрт.
pub(crate) fn n3814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3803);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16962);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40413);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 68473);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94003);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 113160);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16963);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 16964);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кклезиаст.
pub(crate) fn n126810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125510);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 132970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --энтузиаст.
pub(crate) fn n126811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125511);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 132971);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 137402);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 140059);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 141631);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142541);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таст.
pub(crate) fn n17072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16973);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 40429);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 68490);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 94012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хаст.
pub(crate) fn n17073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16974);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 40430);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 68491);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бст.
pub(crate) fn n3816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3805);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 16975);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40431);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 68492);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 94014);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 113166);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 125512);
													match s {
														[] => true,
														['л', s @ ..] => super::t39::n134342(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ест.
pub(crate) fn n3817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3806);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 16976);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 16977);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 40433);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 40434);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40435);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 68494);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94015);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 113167);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125513);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134343(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 16978);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 16979);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40436);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 40437);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 68496);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 16980);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40439);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 68497);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94017);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 113168);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16981);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40440);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 68498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 40441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 40442);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68500);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 68501);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 68502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 16982);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40443);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 68503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94020);
									s.is_empty()
								}
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
			super::push(out, 16983);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40444);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40445);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68504);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94021);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113169);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16984);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40446);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 68505);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 94022);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113170);
											s.is_empty()
										}
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
					super::push(out, 40447);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 40448);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68506);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 68507);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 94023);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 68508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 40449);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68509);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 68510);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94024);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113171);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 94025);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113172);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125514);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134344(s, out),
														_ => false,
													}
												}
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
							super::push(out, 68511);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 94026);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113173);
											s.is_empty()
										}
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
					super::push(out, 40450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40451);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68513);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 94027);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 113174);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125515);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 40452);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68514);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94028);
									s.is_empty()
								}
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
			super::push(out, 16985);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 16986);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 40453);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 16987);
			match s {
				[] => true,
				['е', s @ ..] => super::t39::n40864(s, out),
				['и', s @ ..] => super::t39::n40865(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t39::n17087(s, out),
		['ш', s @ ..] => super::t39::n17088(s, out),
		_ => false,
	}
}

/// Words ending -ист.
pub(crate) fn n3818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3807);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16990);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40460);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 40461);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68518);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94030);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113175);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 16991);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 16992);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40462);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40463);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 40464);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 40465);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 40466);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 68520);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 94031);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 113176);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125516);
													match s {
														[] => true,
														['р', 'п', '-', s @ ..] => super::t39::n134345(s, out),
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
									super::push(out, 94032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 68521);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94033);
									match s {
										[] => true,
										['а', '-', s @ ..] => {
											super::push(out, 113177);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125517);
													match s {
														[] => true,
														['и', s @ ..] => super::t39::n134346(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 40467);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 40468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68522);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 68523);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94034);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 68524);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94035);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113179);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125518);
													s.is_empty()
												}
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
							super::push(out, 68525);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94036);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 68526);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94037);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 94038);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113181);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 68527);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 94039);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113182);
											s.is_empty()
										}
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
					super::push(out, 40469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 16993);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 40470);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['л', 'р', 'е', 'к', '\u{1}', s @ ..] => {
									super::push(out, 94040);
									s.is_empty()
								}
								['ф', 'р', 'е', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113183);
											s.is_empty()
										}
										['д', 'н', 'и', 'в', '\u{1}', s @ ..] => super::t39::n114391(s, out),
										['й', 'а', 'к', 'с', '\u{1}', s @ ..] => super::t39::n114392(s, out),
										['п', 'а', 'с', '\u{1}', s @ ..] => super::t39::n114393(s, out),
										['т', 'й', 'а', 'к', '\u{1}', s @ ..] => super::t39::n114394(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t39::n40880(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t39::n17093(s, out),
		['е', s @ ..] => super::t39::n17094(s, out),
		['ж', s @ ..] => super::t39::n17095(s, out),
		['з', s @ ..] => super::t39::n17096(s, out),
		['к', s @ ..] => super::t39::n17097(s, out),
		['л', s @ ..] => super::t39::n17098(s, out),
		['м', s @ ..] => super::t39::n17099(s, out),
		['н', s @ ..] => super::t39::n17100(s, out),
		['о', s @ ..] => super::t39::n17101(s, out),
		['п', s @ ..] => super::t39::n17102(s, out),
		['р', s @ ..] => super::t39::n17103(s, out),
		['с', s @ ..] => super::t39::n17104(s, out),
		['т', s @ ..] => super::t39::n17105(s, out),
		['ф', s @ ..] => super::t39::n17106(s, out),
		['х', s @ ..] => super::t39::n17107(s, out),
		['ц', s @ ..] => super::t39::n17108(s, out),
		['ч', s @ ..] => super::t39::n17109(s, out),
		['ш', s @ ..] => super::t39::n17110(s, out),
		_ => false,
	}
}

/// Words ending -йст.
pub(crate) fn n3819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3808);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17012);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40511);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 68590);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 94087);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113226);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17013);
			match s {
				[] => true,
				['г', 'р', 'е', 'т', 'л', 'о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 40512);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 40513);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68591);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кст.
pub(crate) fn n3820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3809);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17014);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17015);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 40516);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олст.
pub(crate) fn n17115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17016);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 40517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нст.
pub(crate) fn n3822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3811);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 17017);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ост.
pub(crate) fn n3823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3812);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 17018);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40519);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 17019);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 40520);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 68595);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 94089);
									s.is_empty()
								}
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
			super::push(out, 17020);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 40522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68597);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 68598);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 68599);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 94090);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 94091);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 94092);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113227);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125549);
													match s {
														[] => true,
														['в', s @ ..] => super::t39::n134370(s, out),
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
									super::push(out, 94093);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 113228);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125550);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134371(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17021);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40523);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68600);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 94094);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113229);
											s.is_empty()
										}
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
					super::push(out, 40524);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 68601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17022);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17023);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40526);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40528);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40529);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 40530);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68603);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94095);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 113230);
											s.is_empty()
										}
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
					super::push(out, 40531);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68604);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 40532);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 68605);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 68606);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 94097);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113231);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125551);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 40533);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68607);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94098);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113232);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17025);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40534);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68608);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 94099);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 113233);
											s.is_empty()
										}
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
					super::push(out, 40535);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68609);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 94100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t39::n17125(s, out),
		['р', s @ ..] => {
			super::push(out, 17027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40547);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40548);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 40549);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 40550);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 40551);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68626);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 40552);
					match s {
						[] => true,
						['в', s @ ..] => super::t39::n69350(s, out),
						['н', s @ ..] => super::t39::n69351(s, out),
						['р', s @ ..] => super::t39::n69352(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t39::n40964(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t39::n17127(s, out),
		['т', s @ ..] => super::t39::n17128(s, out),
		['х', s @ ..] => super::t39::n17129(s, out),
		_ => false,
	}
}

/// Words ending -рст.
pub(crate) fn n3824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3813);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17031);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17032);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40556);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68632);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 40557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40558);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 68634);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94121);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113243);
											s.is_empty()
										}
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
					super::push(out, 40559);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 68635);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 94122);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113244);
											s.is_empty()
										}
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
					super::push(out, 40560);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68636);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 94123);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 113245);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125557);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17033);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17034);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40562);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68637);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94124);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113246);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 68638);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94125);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113247);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125558);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17035);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 40563);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68639);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 94126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уст.
pub(crate) fn n3825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3814);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17036);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40564);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68640);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94127);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 113248);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125559);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17037);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 40565);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 68641);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94128);
									s.is_empty()
								}
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
			super::push(out, 17038);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40566);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 40567);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68643);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 94129);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 94130);
									s.is_empty()
								}
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
			super::push(out, 17039);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40568);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40569);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 68644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17040);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17041);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17042);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40571);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68645);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94131);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113249);
											s.is_empty()
										}
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
					super::push(out, 40572);
					match s {
						[] => true,
						['а', 'л', 'з', '\u{1}', s @ ..] => {
							super::push(out, 68646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17043);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40573);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68647);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 68648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40574);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94132);
									s.is_empty()
								}
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
			super::push(out, 17044);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40575);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68650);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 94133);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 113250);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 68651);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94134);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113251);
											s.is_empty()
										}
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
					super::push(out, 40576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 17045);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 17046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыст.
pub(crate) fn n3826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3815);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17047);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 40577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68653);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 68654);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 94135);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113252);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125560);
													match s {
														[] => true,
														['р', s @ ..] => super::t39::n134373(s, out),
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
									super::push(out, 94136);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113253);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эст.
pub(crate) fn n3827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3816);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 17048);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 40578);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68655);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 94137);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113254);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юст.
pub(crate) fn n3828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3817);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17049);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 17050);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17051);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 40580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17052);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40581);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 68657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яст.
pub(crate) fn n3829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3818);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 17053);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 40582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -герлскаут.
pub(crate) fn n126861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125561);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 133004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ейнгут.
pub(crate) fn n69401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68678);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 94151);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 113259);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргут.
pub(crate) fn n17180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17081);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 40613);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 68679);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 68680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тгут.
pub(crate) fn n17181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17082);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40614);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 68681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дут.
pub(crate) fn n3838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3827);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17083);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еут.
pub(crate) fn n3839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3828);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17084);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40616);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 40617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жут.
pub(crate) fn n3840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3829);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 17085);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 17086);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 40618);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зут.
pub(crate) fn n3841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3830);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17087);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 40619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68683);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 68684);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 94153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иут.
pub(crate) fn n3842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3831);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 17088);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 40620);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68685);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кут.
pub(crate) fn n3843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3832);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17089);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17090);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 40621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17091);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40622);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68686);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94155);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113260);
											s.is_empty()
										}
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
					super::push(out, 40623);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68687);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94156);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113261);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 125562);
													match s {
														[] => true,
														['о', s @ ..] => super::t39::n134375(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17092);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40624);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17093);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40625);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 68689);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94157);
									s.is_empty()
								}
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
			super::push(out, 17094);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лут.
pub(crate) fn n3844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3833);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 17095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40627);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 40628);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 68690);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94158);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113262);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мут.
pub(crate) fn n3845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3834);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17096);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40629);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68691);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 94159);
									s.is_empty()
								}
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
			super::push(out, 17097);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 40630);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 40631);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 68693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17098);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40632);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 68694);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 94160);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113263);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125563);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 40633);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 68695);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 94161);
									s.is_empty()
								}
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
			super::push(out, 17099);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40634);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 40635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17100);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40636);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 68697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 40637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17101);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40638);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 68698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 17102);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40639);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 17103);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40640);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68700);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 94163);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 94164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нут.
pub(crate) fn n3846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3835);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 17104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 17105);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40642);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 68701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оут.
pub(crate) fn n3847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3836);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 17106);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 40643);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17107);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 17108);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40644);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пут.
pub(crate) fn n3848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3837);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17109);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17110);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40645);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 40646);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 68704);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 94165);
									s.is_empty()
								}
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
			super::push(out, 17111);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40647);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 17112);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40648);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17113);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40649);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68707);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 94166);
									s.is_empty()
								}
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
			super::push(out, 17114);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17115);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40651);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 68709);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94167);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113264);
											s.is_empty()
										}
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
					super::push(out, 40652);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68710);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 68711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17116);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40653);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68712);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 94168);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113265);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125564);
													match s {
														[] => true,
														['м', s @ ..] => super::t39::n134376(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рут.
pub(crate) fn n3849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3838);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 17117);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 17118);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40654);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 68713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 40655);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 68714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 68715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94170);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 94171);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113266);
											s.is_empty()
										}
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
					super::push(out, 40656);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68716);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 94172);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113267);
											s.is_empty()
										}
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
					super::push(out, 40657);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 68717);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 68718);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 68719);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 40658);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 40659);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 68721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94176);
									s.is_empty()
								}
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
			super::push(out, 17119);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 40660);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 68722);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94177);
									s.is_empty()
								}
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
			super::push(out, 17120);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 17121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 40661);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 40662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17122);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 17123);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40663);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 40664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 17124);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сут.
pub(crate) fn n3850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3839);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17125);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 40666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17126);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40667);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 68725);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 94178);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113268);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125565);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тут.
pub(crate) fn n3851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3840);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17127);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 17128);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 40668);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94179);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 94180);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113269);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125566);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134377(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фут.
pub(crate) fn n3852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3841);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 17129);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 17130);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 17131);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40670);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 68726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 17132);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шут.
pub(crate) fn n3853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3842);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17134);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 17135);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40671);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 68727);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 94181);
									s.is_empty()
								}
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
			super::push(out, 17136);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 40672);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яут.
pub(crate) fn n3854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3843);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17137);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40673);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 40674);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 68729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -майнкрафт.
pub(crate) fn n126867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125567);
	s.is_empty()
}

/// Words ending -оверкрафт.
pub(crate) fn n126868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125568);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 133008);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 137429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловермахт.
pub(crate) fn n126869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125569);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 133009);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137430);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 140078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аутбенахт.
pub(crate) fn n126870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125570);
	s.is_empty()
}

/// Words ending -маастрихт.
pub(crate) fn n126871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125571);
	s.is_empty()
}

/// The word рузвельт.
pub(crate) fn n126872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125572);
	s.is_empty()
}

/// Words ending -исконсульт.
pub(crate) fn n126873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125573);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 133010);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 137431);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льм-дебют.
pub(crate) fn n126874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125574);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 133011);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 137432);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пустосвят.
pub(crate) fn n126875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125575);
	s.is_empty()
}

/// Words ending -ык-изолят.
pub(crate) fn n126876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125576);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 133012);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 137433);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рийгикогу.
pub(crate) fn n126878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125578);
	s.is_empty()
}

/// Words ending -диджериду.
pub(crate) fn n126879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125579);
	s.is_empty()
}

/// Words ending -илопа-гну.
pub(crate) fn n126880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125580);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 133013);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -акита-ину.
pub(crate) fn n126881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125581);
	s.is_empty()
}

/// Words ending -в-на-дону.
pub(crate) fn n126882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125582);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133014);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 137434);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 140079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ч-на-дону.
pub(crate) fn n126883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125583);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 133015);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 137435);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увердьяну.
pub(crate) fn n126884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125584);
	s.is_empty()
}

/// Words ending -п-энд-гоу.
pub(crate) fn n126885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125585);
	s.is_empty()
}

/// The word габаджоу.
pub(crate) fn n126886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125586);
	s.is_empty()
}

/// Words ending -аршмеллоу.
pub(crate) fn n126887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125587);
	s.is_empty()
}

/// Words ending -аршмэллоу.
pub(crate) fn n126888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125588);
	s.is_empty()
}

/// Words ending -рнет-гуру.
pub(crate) fn n126889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125589);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 133016);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -к-кенгуру.
pub(crate) fn n126890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125590);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 133017);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 137436);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 140081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аматэрасу.
pub(crate) fn n126891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125591);
	s.is_empty()
}

/// Words ending -иу-джитсу.
pub(crate) fn n126892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125592);
	s.is_empty()
}

/// Words ending -иту-санту.
pub(crate) fn n126893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125593);
	s.is_empty()
}

/// Words ending -адалберту.
pub(crate) fn n126894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125594);
	s.is_empty()
}

/// Words ending -ва-йыэсуу.
pub(crate) fn n126895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125595);
	s.is_empty()
}

/// Words ending -термошкаф.
pub(crate) fn n126896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125596);
	s.is_empty()
}

/// Words ending -раф.
pub(crate) fn n4036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4025);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17420);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 40941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40942);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t39::n17520(s, out),
		['и', s @ ..] => {
			super::push(out, 17422);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 40955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17423);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 40956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 17424);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40957);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -библиотаф.
pub(crate) fn n126947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125647);
	s.is_empty()
}

/// Words ending -роман-миф.
pub(crate) fn n126948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125648);
	s.is_empty()
}

/// Words ending -гогриф.
pub(crate) fn n69687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68963);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 94364);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -погриф.
pub(crate) fn n69688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68964);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 94365);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 113395);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ериф.
pub(crate) fn n17564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17465);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 40995);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 68965);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 94366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 40996);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -криф.
pub(crate) fn n17565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17466);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 40997);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 68966);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сиф.
pub(crate) fn n4061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4050);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 17467);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68967);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 68968);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 94368);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113396);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125649);
													match s {
														[] => true,
														['р', s @ ..] => super::t39::n134411(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тиф.
pub(crate) fn n4062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4051);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17468);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40999);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 41000);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68969);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94369);
									s.is_empty()
								}
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
			super::push(out, 17469);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41001);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 68970);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94370);
									s.is_empty()
								}
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
			super::push(out, 17470);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41002);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 68971);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94371);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 94372);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 113397);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 125650);
													match s {
														[] => true,
														['б', s @ ..] => super::t39::n134412(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17471);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41003);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фиф.
pub(crate) fn n4063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4052);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17472);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 68972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чиф.
pub(crate) fn n4064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4053);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 17473);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41005);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 68973);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 94373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шиф.
pub(crate) fn n4065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4054);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17474);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word эвритроф.
pub(crate) fn n126952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125652);
	s.is_empty()
}

/// The word апостроф.
pub(crate) fn n126953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125653);
	s.is_empty()
}

/// Words ending -нтропософ.
pub(crate) fn n126954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125654);
	s.is_empty()
}

/// Words ending -дельсдорф.
pub(crate) fn n126955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125655);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 133044);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 137446);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тропоморф.
pub(crate) fn n126956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125656);
	s.is_empty()
}

/// Words ending -минигольф.
pub(crate) fn n126957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125657);
	s.is_empty()
}

/// The word рэндольф.
pub(crate) fn n126958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125658);
	s.is_empty()
}

/// The word хизбаллах.
pub(crate) fn n134415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133045);
	s.is_empty()
}

/// Words ending -акедономах.
pub(crate) fn n134416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133046);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 137447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -монархомах.
pub(crate) fn n134417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133047);
	s.is_empty()
}

/// Words ending -лень-монах.
pub(crate) fn n134418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133048);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 137448);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 140083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пустобрех.
pub(crate) fn n126967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125667);
	s.is_empty()
}

/// Words ending -отех.
pub(crate) fn n17707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17607);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 41135);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 69086);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 94435);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 113431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цех.
pub(crate) fn n4130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4119);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 17608);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41136);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 69087);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94436);
									s.is_empty()
								}
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
			super::push(out, 17609);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41137);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 69088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94437);
									s.is_empty()
								}
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
			super::push(out, 17610);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41138);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69089);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94438);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113432);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 69090);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 94439);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 113433);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125668);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41139);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69091);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94440);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 113434);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17611);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чех.
pub(crate) fn n4131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4120);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17613);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41141);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 69092);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 94441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -астротрих.
pub(crate) fn n126969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125669);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 133049);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ллетотрих.
pub(crate) fn n126970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125670);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133050);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 137449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стих.
pub(crate) fn n17734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17634);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41161);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 69107);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 69108);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 94449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41162);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 69109);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94450);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 113437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 69110);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рамфоринх.
pub(crate) fn n126971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125671);
	s.is_empty()
}

/// Words ending -переполох.
pub(crate) fn n126972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125672);
	s.is_empty()
}

/// Words ending -ертополох.
pub(crate) fn n126973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125673);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 133051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -амох.
pub(crate) fn n17764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17663);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 41184);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 69127);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 94463);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 113445);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 125674);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 133052);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word пох.
pub(crate) fn n17765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17664);
	s.is_empty()
}

/// Words ending -апох.
pub(crate) fn n17766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17665);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 41185);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 69128);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рох.
pub(crate) fn n4161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4150);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 17666);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 41187);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 41188);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69131);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 94465);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 113446);
											s.is_empty()
										}
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
					super::push(out, 41189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69132);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 69133);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94466);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113447);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125675);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', 'п', '\u{1}', s @ ..] => {
			super::push(out, 17667);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сох.
pub(crate) fn n4162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4151);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 17668);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41191);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 69135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17669);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 41192);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чох.
pub(crate) fn n4163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4152);
	s.is_empty()
}

/// Words ending -аттисгарх.
pub(crate) fn n126976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125676);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 133053);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 137450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иммельбух.
pub(crate) fn n126977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125677);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 133054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -подсолнух.
pub(crate) fn n126978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125678);
	s.is_empty()
}

/// Words ending -роух.
pub(crate) fn n17800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17699);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 41226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пух.
pub(crate) fn n4178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4167);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17700);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 17701);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41227);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69155);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94479);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 113455);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125679);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17702);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рух.
pub(crate) fn n4179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4168);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17703);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 17704);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17705);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 41230);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сух.
pub(crate) fn n4180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4169);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17706);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тух.
pub(crate) fn n4181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4170);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17707);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17708);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41233);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 69158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 41234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17709);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 41235);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 41236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17710);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41237);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69160);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94480);
									s.is_empty()
								}
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
			super::push(out, 17711);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41238);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 69161);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94481);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113456);
											s.is_empty()
										}
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
					super::push(out, 41239);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69162);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 94482);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113457);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17712);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17713);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41241);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 17714);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41242);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 41243);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69165);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 94483);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113458);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дисабебец.
pub(crate) fn n126980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125680);
	s.is_empty()
}

/// Words ending -исоглебец.
pub(crate) fn n126981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125681);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 133055);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137451);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ребец.
pub(crate) fn n41704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41289);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 69198);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 94496);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 113464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 125682);
									s.is_empty()
								}
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
			super::push(out, 69199);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 94497);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лбец.
pub(crate) fn n17863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17762);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 41291);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 69202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 69203);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 94501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обец.
pub(crate) fn n17864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17763);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 41292);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 41293);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 69204);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 94502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 69205);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 94503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113465);
									s.is_empty()
								}
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
			super::push(out, 41294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -убец.
pub(crate) fn n17865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17764);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 41295);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 69206);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 94504);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 113466);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 125683);
											s.is_empty()
										}
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
					super::push(out, 69207);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 94505);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 113467);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 125684);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 133056);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41296);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 41297);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69208);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 69209);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 94506);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 113468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 69210);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 94507);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 113469);
									s.is_empty()
								}
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
			super::push(out, 41298);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69211);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 94508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 41299);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 69212);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 94509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбец.
pub(crate) fn n17866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17765);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 41300);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69213);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьбец.
pub(crate) fn n17867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17766);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41301);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69214);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 94510);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 113470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбец.
pub(crate) fn n17868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17767);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41302);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 69215);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябец.
pub(crate) fn n17869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17768);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 41303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 69216);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вец.
pub(crate) fn n4207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4196);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17769);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41304);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69217);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 94511);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 113471);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 94512);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125685);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 125686);
													match s {
														[] => true,
														['т', s @ ..] => super::t39::n134427(s, out),
														_ => false,
													}
												}
												['е', s @ ..] => {
													super::push(out, 125687);
													match s {
														[] => true,
														['р', s @ ..] => super::t39::n134428(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 125688);
													match s {
														[] => true,
														['г', s @ ..] => super::t39::n134429(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41305);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69218);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 41306);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 41307);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 69220);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 94513);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113472);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 125689);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41308);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 69221);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 69222);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94514);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113473);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125690);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134430(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41309);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41310);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69224);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41311);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 69225);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94516);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 94517);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113474);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125691);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 113475);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125692);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t39::n17871(s, out),
		['и', s @ ..] => {
			super::push(out, 17771);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41330);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 41331);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 41332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69254);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 69255);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 94554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 41333);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 69256);
							match s {
								[] => true,
								['о', s @ ..] => super::t39::n95575(s, out),
								['с', s @ ..] => {
									super::push(out, 94556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 41334);
					match s {
						[] => true,
						['е', s @ ..] => super::t39::n69982(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t39::n41750(s, out),
				['ш', s @ ..] => super::t39::n41751(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t39::n17873(s, out),
		['р', s @ ..] => super::t39::n17874(s, out),
		['т', s @ ..] => super::t39::n17875(s, out),
		['ш', s @ ..] => super::t39::n17876(s, out),
		['я', s @ ..] => super::t39::n17877(s, out),
		_ => false,
	}
}

/// Words ending -гец.
pub(crate) fn n4208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4197);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 17777);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69362);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94687);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 113617);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 125798);
													match s {
														[] => true,
														['и', s @ ..] => super::t39::n134496(s, out),
														_ => false,
													}
												}
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
							super::push(out, 69363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17778);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41357);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['е', 'т', 'е', 'г', '\u{1}', s @ ..] => {
									super::push(out, 94688);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 94689);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 113618);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125799);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41358);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69364);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 94690);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 113619);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 125800);
													match s {
														[] => true,
														['а', s @ ..] => super::t39::n134497(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17779);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 41359);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 69365);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 94691);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113620);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125801);
													match s {
														[] => true,
														['и', s @ ..] => super::t39::n134498(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дец.
pub(crate) fn n4209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4198);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17780);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41360);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 69366);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 94692);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113621);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125802);
													match s {
														[] => true,
														['м', s @ ..] => super::t39::n134499(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 17781);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41362);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 69367);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94693);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 113622);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125803);
													match s {
														[] => true,
														['а', s @ ..] => super::t39::n134500(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17782);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41363);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69368);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 41364);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 41365);
					match s {
						[] => true,
						['е', 'с', 'о', '\u{1}', s @ ..] => {
							super::push(out, 69369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 17783);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41366);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69370);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 94695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 41367);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 69371);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 94696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 41368);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69372);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94697);
									s.is_empty()
								}
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
			super::push(out, 17784);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17785);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41370);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 41371);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69374);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94698);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 113623);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 125804);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 94699);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113624);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 125805);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 94700);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 113625);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125806);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41372);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 69375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 41373);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 69376);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 94702);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113626);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125807);
													match s {
														[] => true,
														['д', s @ ..] => super::t39::n134501(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41374);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69377);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 94703);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113627);
											s.is_empty()
										}
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
					super::push(out, 41375);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t39::n17887(s, out),
		['р', s @ ..] => {
			super::push(out, 17787);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41383);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41384);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 69396);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 69397);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94724);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 94725);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113645);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125820);
													match s {
														[] => true,
														['б', s @ ..] => super::t39::n134507(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t39::n17889(s, out),
		['ь', s @ ..] => super::t39::n17890(s, out),
		['я', s @ ..] => super::t39::n17891(s, out),
		_ => false,
	}
}

/// Words ending -еец.
pub(crate) fn n4210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4199);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 17791);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17792);
			match s {
				[] => true,
				['а', 'ф', s @ ..] => {
					super::push(out, 41392);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 69406);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94733);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113650);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125824);
													match s {
														[] => true,
														['р', s @ ..] => super::t39::n134509(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17793);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41393);
					match s {
						[] => true,
						['а', 'о', 'н', 'е', 'л', 'е', 'з', '\u{1}', s @ ..] => {
							super::push(out, 69407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17794);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 17795);
			match s {
				[] => true,
				['о', 'р', 'в', 'е', 'о', 'н', s @ ..] => {
					super::push(out, 41394);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 69408);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94734);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 113651);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125825);
													match s {
														[] => true,
														['з', s @ ..] => super::t39::n134510(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17796);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 17797);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41395);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жец.
pub(crate) fn n4211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4200);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17798);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 17799);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41396);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69410);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 94735);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 113652);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125826);
													match s {
														[] => true,
														['к', s @ ..] => super::t39::n134511(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17800);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41397);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 41398);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 41399);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69412);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94736);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113653);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125827);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41400);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 69413);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94737);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 113654);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125828);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17801);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41401);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69414);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 94738);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113655);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17802);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17803);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41402);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 41403);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 69415);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 69416);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 94739);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113656);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125829);
													match s {
														[] => true,
														['в', s @ ..] => super::t39::n134512(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41404);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69417);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 94740);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 113657);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125830);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17804);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41405);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 69418);
							match s {
								[] => true,
								['о', 'м', s @ ..] => {
									super::push(out, 94741);
									match s {
										[] => true,
										['о', 'р', 'г', '-', s @ ..] => {
											super::push(out, 113658);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125831);
													match s {
														[] => true,
														['о', s @ ..] => super::t39::n134513(s, out),
														_ => false,
													}
												}
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
							super::push(out, 69419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41406);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69420);
							match s {
								[] => true,
								['е', 'т', 'е', 'г', '\u{1}', s @ ..] => {
									super::push(out, 94742);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 94743);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 113659);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125832);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41407);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 94744);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113660);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125833);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134514(s, out),
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
									super::push(out, 94745);
									s.is_empty()
								}
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
			super::push(out, 17805);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зец.
pub(crate) fn n4212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4201);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17806);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41408);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69421);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94746);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113661);
											s.is_empty()
										}
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
					super::push(out, 41409);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69422);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 94747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 69423);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94748);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 113662);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 69424);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94749);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 69425);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 94750);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113663);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125834);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41410);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69426);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41411);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69427);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 69428);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 94753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 41412);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 69429);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94754);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 113664);
											s.is_empty()
										}
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
					super::push(out, 41413);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69430);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94755);
									s.is_empty()
								}
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
			super::push(out, 17807);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41414);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 69431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17808);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41415);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69432);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 94756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41416);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69433);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94757);
									s.is_empty()
								}
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
			super::push(out, 17809);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41417);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 41418);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69434);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 69435);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94759);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 113665);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125835);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17810);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41420);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 69436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17811);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41421);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 41422);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69438);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41423);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69439);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 94761);
									s.is_empty()
								}
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
			super::push(out, 17812);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41424);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69440);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 94762);
									s.is_empty()
								}
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
			super::push(out, 17813);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41425);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 69441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 17814);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41426);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 17815);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41427);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 69443);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иец.
pub(crate) fn n4213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4202);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17816);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 17817);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 17818);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 17819);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 17820);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 17821);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 17822);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41428);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69444);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94764);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113666);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125836);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17823);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 41429);
					match s {
						[] => true,
						['и', 'л', 'о', '-', s @ ..] => {
							super::push(out, 69445);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94765);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113667);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125837);
													match s {
														[] => true,
														['с', s @ ..] => super::t39::n134515(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17824);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 41430);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69446);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94766);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 113668);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125838);
													match s {
														[] => true,
														['к', s @ ..] => super::t39::n134516(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17825);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41431);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 69447);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94767);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113669);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17826);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41432);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кец.
pub(crate) fn n4214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4203);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17827);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41433);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 41434);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69448);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94768);
									s.is_empty()
								}
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
			super::push(out, 17828);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41435);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 69449);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 94769);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 113670);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 125839);
													match s {
														[] => true,
														['е', s @ ..] => super::t39::n134517(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17829);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 17830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лец.
pub(crate) fn n4215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4204);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17831);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41436);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69450);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 69451);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 69452);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94770);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113671);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125840);
													s.is_empty()
												}
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
							super::push(out, 69453);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94771);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113672);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 69454);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 69455);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 69456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 41437);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69457);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 69458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 41438);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69459);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 94772);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113673);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 94773);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 113674);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125841);
													match s {
														[] => true,
														['о', 'д', 'з', 'и', 'п', '\u{1}', s @ ..] => super::t39::n134518(s, out),
														_ => false,
													}
												}
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
							super::push(out, 69460);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 69461);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94774);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 113675);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125842);
													s.is_empty()
												}
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
							super::push(out, 69462);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 41439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 41440);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94776);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 113676);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125843);
													match s {
														[] => true,
														['м', s @ ..] => super::t39::n134519(s, out),
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
									super::push(out, 94777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 41441);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 69464);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 69465);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 94779);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113677);
											s.is_empty()
										}
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
					super::push(out, 41442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69466);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 69467);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 94780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 69468);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 69469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 41443);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69470);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 94782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 69471);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 94783);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 113678);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t39::n41860(s, out),
				['р', s @ ..] => {
					super::push(out, 41445);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 69476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41446);
					match s {
						[] => true,
						['п', s @ ..] => super::t39::n70205(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t39::n41863(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t39::n17933(s, out),
		['г', s @ ..] => super::t39::n17934(s, out),
		['д', s @ ..] => super::t39::n17935(s, out),
		['е', s @ ..] => super::t39::n17936(s, out),
		['ж', s @ ..] => super::t39::n17937(s, out),
		['з', s @ ..] => super::t39::n17938(s, out),
		['и', s @ ..] => super::t39::n17939(s, out),
		['к', s @ ..] => super::t39::n17940(s, out),
		['о', s @ ..] => super::t39::n17941(s, out),
		['р', s @ ..] => super::t39::n17942(s, out),
		['у', s @ ..] => super::t39::n17943(s, out),
		['ф', s @ ..] => {
			super::push(out, 17843);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t39::n41905(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t39::n17945(s, out),
		['ы', s @ ..] => super::t39::n17946(s, out),
		['я', s @ ..] => super::t39::n17947(s, out),
		_ => false,
	}
}

/// Words ending -мец.
pub(crate) fn n4216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4205);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17847);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41497);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 41498);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 41499);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 69539);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 94840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 69540);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 94841);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113718);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125864);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41500);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 69541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41501);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 69542);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 69543);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 94842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69544);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 69545);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94843);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 113719);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 125865);
													s.is_empty()
												}
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
							super::push(out, 69546);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 69547);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94845);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113720);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125866);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17848);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41503);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 41504);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 69548);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 94846);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113721);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125867);
													match s {
														[] => true,
														['р', s @ ..] => super::t39::n134530(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41505);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 69549);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 69550);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 69551);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 94847);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113722);
											s.is_empty()
										}
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
					super::push(out, 41506);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69552);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94848);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113723);
											s.is_empty()
										}
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
					super::push(out, 41507);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 69553);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 94849);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113724);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125868);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 94850);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 113725);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125869);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41508);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69554);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 94851);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113726);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125870);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17849);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41509);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69555);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94852);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113727);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17850);
			match s {
				[] => true,
				['б', s @ ..] => super::t39::n41926(s, out),
				['к', s @ ..] => super::t39::n41927(s, out),
				['р', s @ ..] => super::t39::n41928(s, out),
				['т', s @ ..] => super::t39::n41929(s, out),
				['х', s @ ..] => super::t39::n41930(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t39::n17952(s, out),
		['ы', s @ ..] => {
			super::push(out, 17852);
			s.is_empty()
		}
		['ь', s @ ..] => super::t39::n17954(s, out),
		['ю', s @ ..] => super::t39::n17955(s, out),
		['я', s @ ..] => super::t39::n17956(s, out),
		_ => false,
	}
}

/// Words ending -нец.
pub(crate) fn n4217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4206);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17856);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41522);
					s.is_empty()
				}
				['в', s @ ..] => super::t39::n41939(s, out),
				['г', s @ ..] => super::t39::n41940(s, out),
				['д', s @ ..] => {
					super::push(out, 41525);
					match s {
						[] => true,
						['а', s @ ..] => super::t39::n70310(s, out),
						['и', s @ ..] => {
							super::push(out, 69582);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 94881);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t39::n114962(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => super::t39::n70312(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t39::n41942(s, out),
				['ж', s @ ..] => super::t39::n41943(s, out),
				['з', s @ ..] => super::t39::n41944(s, out),
				['и', s @ ..] => super::t39::n41945(s, out),
				['к', s @ ..] => super::t39::n41946(s, out),
				['л', s @ ..] => super::t39::n41947(s, out),
				['м', s @ ..] => super::t39::n41948(s, out),
				['н', s @ ..] => super::t39::n41949(s, out),
				['п', s @ ..] => super::t39::n41950(s, out),
				['р', s @ ..] => super::t39::n41951(s, out),
				['с', s @ ..] => super::t39::n41952(s, out),
				['т', s @ ..] => super::t39::n41953(s, out),
				['у', s @ ..] => super::t39::n41954(s, out),
				['х', s @ ..] => super::t39::n41955(s, out),
				['ч', s @ ..] => super::t39::n41956(s, out),
				['ш', s @ ..] => super::t39::n41957(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t39::n17958(s, out),
		['е', s @ ..] => super::t39::n17959(s, out),
		['ж', s @ ..] => super::t39::n17960(s, out),
		['з', s @ ..] => super::t39::n17961(s, out),
		['и', s @ ..] => super::t39::n17962(s, out),
		['й', s @ ..] => super::t39::n17963(s, out),
		['м', s @ ..] => super::t39::n17964(s, out),
		['н', s @ ..] => super::t39::n17965(s, out),
		['о', s @ ..] => super::t39::n17966(s, out),
		['р', s @ ..] => super::t39::n17967(s, out),
		['у', s @ ..] => super::t39::n17968(s, out),
		['ы', s @ ..] => super::t39::n17969(s, out),
		['ь', s @ ..] => super::t39::n17970(s, out),
		['ю', s @ ..] => super::t39::n17971(s, out),
		['я', s @ ..] => super::t39::n17972(s, out),
		_ => false,
	}
}

/// Words ending -оец.
pub(crate) fn n4218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4207);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 17872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41637);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 41638);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 69879);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95230);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 113982);
											s.is_empty()
										}
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
					super::push(out, 41639);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41640);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69880);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 69881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41641);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 69882);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95231);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 113983);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125987);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17873);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41642);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69883);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 95232);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 113984);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125988);
													s.is_empty()
												}
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
							super::push(out, 69884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 17874);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41643);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95233);
									s.is_empty()
								}
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
			super::push(out, 17875);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41644);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 69886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 17876);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41645);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пец.
pub(crate) fn n4219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4208);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 17877);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41646);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 69887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 41647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17878);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69888);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 69889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 41649);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 41650);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 41651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 17879);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 41652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17880);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69891);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 69892);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95234);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113985);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 125989);
													match s {
														[] => true,
														['о', s @ ..] => super::t39::n134576(s, out),
														_ => false,
													}
												}
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
							super::push(out, 69893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 41654);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69894);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95235);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 69895);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 69896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 41655);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 41656);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 69897);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 95236);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 69898);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95237);
									s.is_empty()
								}
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
			super::push(out, 17881);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41657);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69899);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95238);
									s.is_empty()
								}
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
			super::push(out, 17882);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41658);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 69900);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95239);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 113986);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125990);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17883);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17884);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41660);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69901);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95240);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 113987);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125991);
													match s {
														[] => true,
														['с', s @ ..] => super::t39::n134577(s, out),
														_ => false,
													}
												}
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
							super::push(out, 69902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 41661);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 69903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 17885);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 41664);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рец.
pub(crate) fn n4220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4209);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17886);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41665);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69906);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 95241);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113988);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 69907);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95242);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113989);
											s.is_empty()
										}
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
					super::push(out, 41666);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69908);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 69909);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 95243);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113990);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 69910);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 95244);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113991);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125992);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134578(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41667);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 41668);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69911);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95245);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 113992);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 125993);
													s.is_empty()
												}
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
							super::push(out, 69912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 41669);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 41670);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 41671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 41672);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69914);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 69915);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95247);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 113993);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 69916);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95248);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113994);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 69917);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 95249);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 113995);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 69918);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95250);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113996);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125994);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41673);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 41674);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 69919);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 95251);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 113997);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125995);
													match s {
														[] => true,
														['а', s @ ..] => super::t39::n134579(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41675);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69920);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95252);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113998);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 69921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 41676);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69922);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95253);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 113999);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 69923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 41677);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 41678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 17887);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41679);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69924);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 95254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 69925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 41680);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 69926);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t39::n17989(s, out),
		['д', s @ ..] => super::t39::n17990(s, out),
		['е', s @ ..] => super::t39::n17991(s, out),
		['ж', s @ ..] => {
			super::push(out, 17891);
			s.is_empty()
		}
		['и', s @ ..] => super::t39::n17993(s, out),
		['к', s @ ..] => super::t39::n17994(s, out),
		['о', s @ ..] => super::t39::n17995(s, out),
		['р', s @ ..] => {
			super::push(out, 17895);
			s.is_empty()
		}
		['т', s @ ..] => super::t39::n17997(s, out),
		['у', s @ ..] => super::t39::n17998(s, out),
		['ы', s @ ..] => super::t39::n17999(s, out),
		['ю', s @ ..] => super::t39::n18000(s, out),
		['я', s @ ..] => super::t39::n18001(s, out),
		_ => false,
	}
}

/// Words ending -сец.
pub(crate) fn n4221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4210);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17901);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41727);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69987);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95307);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 114035);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126013);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41728);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 41729);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 69988);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95308);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 114036);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126014);
													s.is_empty()
												}
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
							super::push(out, 69989);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95309);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114037);
											s.is_empty()
										}
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
					super::push(out, 41730);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69990);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 95310);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114038);
											s.is_empty()
										}
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
					super::push(out, 41731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 17902);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17903);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41733);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69992);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 95311);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 41734);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 41735);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 41736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17904);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41737);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 69996);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 95312);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 114039);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126015);
													match s {
														[] => true,
														['и', s @ ..] => super::t39::n134591(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69997);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 69998);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 95313);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114040);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 126016);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n134592(s, out),
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
									super::push(out, 95314);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 95315);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 114041);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 114042);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 126017);
													match s {
														[] => true,
														['ы', s @ ..] => super::t39::n134593(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17905);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41739);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69999);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 95316);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 95317);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 114043);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 126018);
													match s {
														[] => true,
														['е', s @ ..] => super::t39::n134594(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t39::n18007(s, out),
		['п', s @ ..] => {
			super::push(out, 17907);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41747);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70012);
							match s {
								[] => true,
								['а', s @ ..] => super::t39::n96357(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t39::n18009(s, out),
		['у', s @ ..] => super::t39::n18010(s, out),
		['ю', s @ ..] => super::t39::n18011(s, out),
		_ => false,
	}
}

/// Words ending -тец.
pub(crate) fn n4222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4211);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17911);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41755);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 41756);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70023);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 95340);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 70024);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95341);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114066);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126035);
													match s {
														[] => true,
														['п', s @ ..] => super::t39::n134605(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41757);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70025);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95342);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 114067);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126036);
													s.is_empty()
												}
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
							super::push(out, 70026);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 70027);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95343);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114068);
											s.is_empty()
										}
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
					super::push(out, 41758);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 41759);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41760);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70029);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95344);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 114069);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 126037);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41761);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70030);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95345);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 114070);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126038);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17913);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41763);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70031);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 41764);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70032);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95347);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114071);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 70033);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 41765);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 70034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 41766);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70035);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 95349);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 114072);
											s.is_empty()
										}
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
					super::push(out, 41767);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70036);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 70037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70038);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95350);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 95351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 70039);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 95352);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114073);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 126039);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41768);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 70040);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95353);
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
						['а', s @ ..] => {
							super::push(out, 70041);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95354);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114074);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 126040);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t39::n70789(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t39::n42190(s, out),
				['с', s @ ..] => super::t39::n42191(s, out),
				['ф', s @ ..] => super::t39::n42192(s, out),
				['ч', s @ ..] => super::t39::n42193(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t39::n18015(s, out),
		['й', s @ ..] => super::t39::n18016(s, out),
		['к', s @ ..] => super::t39::n18017(s, out),
		['л', s @ ..] => super::t39::n18018(s, out),
		['н', s @ ..] => super::t39::n18019(s, out),
		['о', s @ ..] => super::t39::n18020(s, out),
		['р', s @ ..] => super::t39::n18021(s, out),
		['с', s @ ..] => super::t39::n18022(s, out),
		['т', s @ ..] => super::t39::n18023(s, out),
		['у', s @ ..] => super::t39::n18024(s, out),
		['ч', s @ ..] => {
			super::push(out, 17924);
			s.is_empty()
		}
		['ш', s @ ..] => super::t39::n18026(s, out),
		['ь', s @ ..] => super::t39::n18027(s, out),
		_ => false,
	}
}

/// Words ending -уец.
pub(crate) fn n4223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4212);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17927);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41808);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70086);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 95388);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 114091);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 126052);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17928);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41809);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70087);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 95389);
									s.is_empty()
								}
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
			super::push(out, 17929);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41810);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хец.
pub(crate) fn n4224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4213);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17930);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17931);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41812);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70088);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95390);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 114092);
											s.is_empty()
										}
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
					super::push(out, 41813);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 70089);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 95391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 41814);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чец.
pub(crate) fn n4225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4214);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 17932);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41815);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70091);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 95392);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114093);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 126053);
													match s {
														[] => true,
														['г', s @ ..] => super::t39::n134611(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17933);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41816);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70092);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95393);
									s.is_empty()
								}
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
			super::push(out, 17934);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 41817);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70093);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 95394);
									match s {
										[] => true,
										['е', 'р', 'д', '-', s @ ..] => {
											super::push(out, 114094);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 126054);
													match s {
														[] => true,
														['е', s @ ..] => super::t39::n134612(s, out),
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
									super::push(out, 95395);
									s.is_empty()
								}
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
			super::push(out, 17935);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41818);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70094);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шец.
pub(crate) fn n4226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4215);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17936);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 17937);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41819);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 41820);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 17938);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41821);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95397);
									s.is_empty()
								}
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
			super::push(out, 17939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щец.
pub(crate) fn n4227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4216);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 17940);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41822);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70098);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 95398);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 114095);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17941);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41823);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 70099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыец.
pub(crate) fn n4228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4217);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 17942);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41824);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70100);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95399);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114096);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яец.
pub(crate) fn n4229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4218);
	match s {
		[] => true,
		['щ', s @ ..] => {
			super::push(out, 17943);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41825);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70101);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 95400);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114097);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нойштрелиц.
pub(crate) fn n134613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133243);
	s.is_empty()
}

/// Words ending -лец-губач.
pub(crate) fn n127356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126056);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133244);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 137528);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едь-губач.
pub(crate) fn n127357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126057);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 133245);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 137529);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вспомогач.
pub(crate) fn n127358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126058);
	s.is_empty()
}

/// Words ending -ягач.
pub(crate) fn n18102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18001);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 41872);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 70125);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 95416);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114109);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 126059);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133246);
													match s {
														[] => true,
														['д', s @ ..] => super::t39::n138949(s, out),
														_ => false,
													}
												}
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
							super::push(out, 95417);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 114110);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 126060);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 133247);
													match s {
														[] => true,
														['о', s @ ..] => super::t39::n138950(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 70126);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95418);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 114111);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126061);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дач.
pub(crate) fn n4258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4247);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18002);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18003);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жач.
pub(crate) fn n4259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4248);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41874);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 41875);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зач.
pub(crate) fn n4260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4249);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18006);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41876);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кач.
pub(crate) fn n4261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4250);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18007);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 18008);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18009);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41878);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70130);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95419);
									s.is_empty()
								}
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
			super::push(out, 18010);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41879);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70131);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 95420);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 114112);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 126062);
													match s {
														[] => true,
														['с', s @ ..] => super::t39::n134618(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18011);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41880);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 70132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18012);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41881);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 41882);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70134);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18013);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 41883);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70135);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 95421);
									s.is_empty()
								}
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
			super::push(out, 18014);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 18015);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41884);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41885);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70137);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95422);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114113);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 126063);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18016);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лач.
pub(crate) fn n4262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4251);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18017);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41886);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 41887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18018);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 41888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18019);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 18020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18021);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41891);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18022);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41892);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 18023);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41893);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 70140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мач.
pub(crate) fn n4263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4252);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18024);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 18025);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18026);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18027);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 18028);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18029);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 18030);
			match s {
				[] => true,
				['и', 'р', '\u{1}', s @ ..] => {
					super::push(out, 41897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 18031);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41898);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нач.
pub(crate) fn n4264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4253);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18032);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41899);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70141);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 41900);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 41901);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 70143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 18033);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41902);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пач.
pub(crate) fn n4265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4254);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18034);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18035);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18036);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41905);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18037);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18038);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41907);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70145);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 18039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рач.
pub(crate) fn n4266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4255);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18040);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 41908);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70146);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 95425);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 114114);
											match s {
												[] => true,
												['щ', s @ ..] => {
													super::push(out, 126064);
													match s {
														[] => true,
														['н', s @ ..] => super::t39::n134619(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41909);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 41910);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 18041);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 18042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41911);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 41912);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 41913);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70148);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41914);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41915);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70149);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 95426);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114115);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 126065);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сач.
pub(crate) fn n4267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4256);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18044);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41916);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 70150);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 95427);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 114116);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 126066);
													match s {
														[] => true,
														['е', s @ ..] => super::t39::n134620(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18045);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41918);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70151);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95428);
									s.is_empty()
								}
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
			super::push(out, 18046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тач.
pub(crate) fn n4268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4257);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18047);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 41919);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70152);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95429);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 114117);
											s.is_empty()
										}
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
					super::push(out, 41920);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 70153);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95430);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 114118);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 126067);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18048);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41921);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18049);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 41922);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70155);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 95431);
									s.is_empty()
								}
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
			super::push(out, 18050);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41923);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18051);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41924);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 18052);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41925);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фач.
pub(crate) fn n4269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4258);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 18053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хач.
pub(crate) fn n4270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4259);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18054);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 41926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18055);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41927);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 70160);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 95432);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 114119);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 126068);
													match s {
														[] => true,
														['т', s @ ..] => super::t39::n134621(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18056);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18057);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 18058);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ндвич.
pub(crate) fn n42362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41941);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 70165);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 95434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 70166);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 95435);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -евич.
pub(crate) fn n18174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18073);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 41942);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 41943);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 70167);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95436);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 70168);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95437);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 114121);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 70169);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 95438);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114122);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 70170);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95439);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 70171);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 95440);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 70172);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 95441);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114125);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 126069);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41944);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 70173);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 95442);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 114126);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 126070);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 133252);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41945);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 70174);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 95443);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 114127);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 126071);
											s.is_empty()
										}
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
					super::push(out, 70175);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 70176);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95444);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114128);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 126072);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41946);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70177);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 95445);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114129);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126073);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 95446);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114130);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 126074);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 95447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 70178);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 95448);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114131);
									s.is_empty()
								}
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
			super::push(out, 41947);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 70179);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сквич.
pub(crate) fn n42369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41948);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 70180);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 95449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нвич.
pub(crate) fn n18176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18075);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41949);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 70181);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 95450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ович.
pub(crate) fn n18177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18076);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 41950);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70182);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95451);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 95452);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 114133);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 126075);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 133253);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t39::n138955(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 70183);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 41951);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70184);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95454);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 114134);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126076);
											s.is_empty()
										}
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
					super::push(out, 70185);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 95455);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 114135);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 126077);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 133254);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 41952);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 70186);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 70187);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95456);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 114136);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 126078);
											s.is_empty()
										}
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
					super::push(out, 70188);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 70189);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95457);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114137);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 126079);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 133255);
													match s {
														[] => true,
														['с', s @ ..] => super::t39::n138956(s, out),
														_ => false,
													}
												}
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
							super::push(out, 95458);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114138);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 70190);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95459);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 114139);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 70191);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 95460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 114140);
									s.is_empty()
								}
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
			super::push(out, 41953);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70192);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 95461);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 114141);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 126080);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 133256);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 114142);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 126081);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 133257);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 70193);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95462);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 95463);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 114143);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 126082);
											s.is_empty()
										}
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
					super::push(out, 70194);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 95464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 41954);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 41955);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 41956);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 70196);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 95465);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 114144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 70197);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 95466);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 114145);
									match s {
										[] => true,
										['т', s @ ..] => super::t39::n127383(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t39::n70945(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t39::n42378(s, out),
		['р', s @ ..] => super::t39::n42379(s, out),
		['с', s @ ..] => {
			super::push(out, 41959);
			s.is_empty()
		}
		['т', s @ ..] => super::t39::n42381(s, out),
		['ф', s @ ..] => super::t39::n42382(s, out),
		['х', s @ ..] => super::t39::n42383(s, out),
		_ => false,
	}
}

/// Words ending -свич.
pub(crate) fn n18178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18077);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 41963);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 70213);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гич.
pub(crate) fn n4280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4269);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 18078);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41964);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70214);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дич.
pub(crate) fn n4281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4270);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 18079);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41965);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70215);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95486);
									s.is_empty()
								}
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
			super::push(out, 18080);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18081);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41967);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еич.
pub(crate) fn n4282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4271);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 18082);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41968);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70216);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жич.
pub(crate) fn n4283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4272);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 18083);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41969);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зич.
pub(crate) fn n4284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4273);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18084);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кич.
pub(crate) fn n4285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18085);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 18086);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18087);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 41972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18088);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41973);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лич.
pub(crate) fn n4286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4275);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18089);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18090);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70218);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 70219);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95488);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114166);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 126100);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41976);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70221);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95489);
									s.is_empty()
								}
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
			super::push(out, 18091);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18092);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 18093);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мич.
pub(crate) fn n4287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4276);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18094);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41979);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70223);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95490);
									s.is_empty()
								}
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
			super::push(out, 18095);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41980);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70224);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 18096);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 18097);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 41983);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 70225);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 70226);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 95493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нич.
pub(crate) fn n4288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4277);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18098);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 18099);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 41985);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 70227);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95494);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114167);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 126101);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пич.
pub(crate) fn n4289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4278);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18100);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41986);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18101);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 41987);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 70229);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рич.
pub(crate) fn n4290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4279);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18102);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18103);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41989);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 70230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 41990);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 18104);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 18105);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41991);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сич.
pub(crate) fn n4291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4280);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 18106);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тич.
pub(crate) fn n4292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4281);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 18107);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70233);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95496);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 114168);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18108);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 41994);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 70234);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 95497);
									s.is_empty()
								}
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
			super::push(out, 18109);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41995);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уич.
pub(crate) fn n4293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4282);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 18110);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 41996);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хич.
pub(crate) fn n4294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4283);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 18111);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чич.
pub(crate) fn n4295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4284);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18112);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 41998);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 70236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18113);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41999);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70237);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 95498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьич.
pub(crate) fn n4296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4285);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18114);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42000);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -макароныч.
pub(crate) fn n127404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126104);
	s.is_empty()
}

/// Words ending -карыч.
pub(crate) fn n42461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42040);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 70263);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 95518);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 114175);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 126105);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word нарыч.
pub(crate) fn n71011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70264);
	s.is_empty()
}

/// Words ending -сарыч.
pub(crate) fn n42463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42042);
	s.is_empty()
}

/// Words ending -ерыч.
pub(crate) fn n18261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18160);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 42043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 70265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 42044);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word могорыч.
pub(crate) fn n18262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18161);
	s.is_empty()
}

/// Words ending -хрыч.
pub(crate) fn n18263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18162);
	s.is_empty()
}

/// Words ending -сыч.
pub(crate) fn n4335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4324);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18163);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 42045);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70266);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 95519);
									s.is_empty()
								}
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
			super::push(out, 18164);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42046);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70267);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -раманмараш.
pub(crate) fn n134637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133267);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 137541);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бангладеш.
pub(crate) fn n127408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126108);
	s.is_empty()
}

/// Words ending -ак-кишмиш.
pub(crate) fn n127409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126109);
	s.is_empty()
}

/// Words ending -фут-фетиш.
pub(crate) fn n127410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126110);
	s.is_empty()
}

/// Words ending -баттерфиш.
pub(crate) fn n127411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126111);
	s.is_empty()
}

/// Words ending -чиш.
pub(crate) fn n4382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4371);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 18261);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42139);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 70331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 18262);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42140);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70332);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 95546);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шиш.
pub(crate) fn n4383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4372);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18263);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18264);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18265);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42142);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 70333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тч-реванш.
pub(crate) fn n127412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126112);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 133268);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 137542);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унтерменш.
pub(crate) fn n127413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126113);
	s.is_empty()
}

/// Words ending -вербункош.
pub(crate) fn n127414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126114);
	s.is_empty()
}

/// Words ending -омни-марш.
pub(crate) fn n127415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126115);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 133269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лыш.
pub(crate) fn n4419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4408);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18337);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42208);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70379);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 95571);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114201);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 126116);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42209);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 42210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18338);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42211);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 42212);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70380);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 95572);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 70381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мыш.
pub(crate) fn n4420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4409);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18339);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42213);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 42214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 42215);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70383);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 95573);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114203);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18340);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42216);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70384);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 42217);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70385);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95575);
									s.is_empty()
								}
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
			super::push(out, 18341);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42218);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18342);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42219);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70387);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 95576);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114204);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 95577);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 114205);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 70388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ныш.
pub(crate) fn n4421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4410);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18343);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 42220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18344);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 42221);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70390);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95578);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114206);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18345);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 18346);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 18347);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42222);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 70391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пыш.
pub(crate) fn n4422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4411);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18348);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42223);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 42224);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 70393);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 70394);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95579);
									s.is_empty()
								}
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
			super::push(out, 18349);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42225);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рыш.
pub(crate) fn n4423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4412);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18350);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 42227);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 42228);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 18351);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42229);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70399);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95580);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 114207);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18352);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42230);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 42231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18353);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42232);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 42233);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70401);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 42234);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70402);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 95582);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114208);
											s.is_empty()
										}
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
					super::push(out, 42235);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70403);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 95583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 42236);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 70404);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 95584);
									s.is_empty()
								}
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
			super::push(out, 18354);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42237);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тыш.
pub(crate) fn n4424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4413);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18355);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42238);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70406);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 70407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 42239);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18356);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42240);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 70409);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 95585);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 70410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 18357);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 70411);
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
					super::push(out, 42242);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 70412);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95586);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 114209);
											s.is_empty()
										}
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
					super::push(out, 42243);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70413);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 95587);
									s.is_empty()
								}
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
			super::push(out, 18358);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 42244);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 70414);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95588);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 114210);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18359);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42245);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 70415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18360);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 42246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оды.
pub(crate) fn n4481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4470);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18433);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 42306);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 42307);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 70460);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 95614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42308);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 70461);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 95615);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 114222);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18434);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 42309);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 70462);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95616);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 114223);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 126120);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18435);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 18436);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42310);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 42311);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 70463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рды.
pub(crate) fn n4482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4471);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18437);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42312);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 70464);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 95617);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 114224);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126121);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18438);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42314);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 70465);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 95618);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 114225);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18439);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 42315);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 70466);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 95619);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 114226);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 126122);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гуды.
pub(crate) fn n18544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18440);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 42316);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 70467);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 95620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -муды.
pub(crate) fn n18545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18441);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 42317);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 70468);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 95621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -суды.
pub(crate) fn n18546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18442);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42318);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 70469);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 95622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

