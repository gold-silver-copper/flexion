use flexion::Path;

/// Words ending -сатель.
pub(crate) fn n71870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71122);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96130);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 114580);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 133483);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137691);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140211);
													match s {
														[] => true,
														['л', s @ ..] => super::t53::n143168(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 137692);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 140212);
													match s {
														[] => true,
														['н', s @ ..] => super::t53::n143169(s, out),
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
									super::push(out, 133484);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 133485);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137693);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 137694);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140213);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n143170(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96131);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 114581);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -татель.
pub(crate) fn n71871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71123);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 96132);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', 'н', s @ ..] => {
							super::push(out, 126408);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133486);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 137695);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140214);
													s.is_empty()
												}
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
							super::push(out, 126409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 114582);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 126410);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133487);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 137696);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140215);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 133488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114583);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 126411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 96133);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 114584);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 114585);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126412);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 126413);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 126414);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 133489);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137697);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 140216);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 133490);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 137698);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 140217);
													s.is_empty()
												}
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
							super::push(out, 126415);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133491);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 137699);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 140218);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143171(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 114586);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96134);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 96135);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 96136);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 96137);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 114588);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126417);
							match s {
								[] => true,
								['и', s @ ..] => super::t53::n134865(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хатель.
pub(crate) fn n71872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71124);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 96138);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цатель.
pub(crate) fn n71873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71125);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 96139);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114589);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чатель.
pub(crate) fn n71874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71126);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 96140);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114590);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 96141);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 126419);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133492);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 137702);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 126420);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133493);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137703);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 140223);
													s.is_empty()
												}
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
							super::push(out, 126421);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133494);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137704);
											s.is_empty()
										}
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
					super::push(out, 114591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 96142);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114592);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 126422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шатель.
pub(crate) fn n71875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71127);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96143);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114593);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 126423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 96144);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96145);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114596);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 126425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 96146);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114597);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 96147);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114598);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126427);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щатель.
pub(crate) fn n71876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71128);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96148);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114599);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126428);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133496);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 133497);
									s.is_empty()
								}
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
			super::push(out, 96149);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114600);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126429);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133498);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 137705);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 126430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --истребитель.
pub(crate) fn n141668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140224);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141728);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 142601);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 143071);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 143311);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 143423);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 143473);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n144947(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лог-любитель.
pub(crate) fn n141669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140225);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141729);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142602);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 143072);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 143312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лот-любитель.
pub(crate) fn n141670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140226);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141730);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142603);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -витель.
pub(crate) fn n71883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71135);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96162);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 114615);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133507);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137711);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140227);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 133508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 114616);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 114617);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 126441);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133509);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 137712);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 140228);
													match s {
														[] => true,
														['ь', s @ ..] => super::t53::n143179(s, out),
														_ => false,
													}
												}
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
							super::push(out, 126442);
							match s {
								[] => true,
								['о', 'р', s @ ..] => {
									super::push(out, 133510);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137713);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140229);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114618);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126443);
							match s {
								[] => true,
								['о', 'с', s @ ..] => {
									super::push(out, 133511);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137714);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140230);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143180(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96163);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114619);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126444);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 133512);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 137715);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 140231);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96164);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126445);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 126446);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 114622);
					match s {
						[] => true,
						['а', 'т', 'с', 'с', s @ ..] => {
							super::push(out, 126447);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133514);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 137716);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 126448);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 126449);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 114623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114624);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 126451);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133517);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137717);
											s.is_empty()
										}
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
					super::push(out, 114625);
					match s {
						[] => true,
						['о', s @ ..] => super::t53::n127756(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t53::n97192(s, out),
		['я', s @ ..] => super::t53::n97193(s, out),
		_ => false,
	}
}

/// Words ending -дитель.
pub(crate) fn n71884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71136);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96167);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114631);
					match s {
						[] => true,
						['с', 'у', '\u{1}', s @ ..] => {
							super::push(out, 126458);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 126459);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133522);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137721);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 140234);
													match s {
														[] => true,
														['у', s @ ..] => super::t53::n143183(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 137722);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126460);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133523);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 126461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 114632);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126462);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133524);
									s.is_empty()
								}
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
			super::push(out, 96168);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 114633);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126463);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133525);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137723);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 140235);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143184(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 114634);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126464);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 126465);
							match s {
								[] => true,
								['у', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137724);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 140236);
													match s {
														[] => true,
														['е', s @ ..] => super::t53::n143185(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 137725);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96169);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 114635);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126466);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133526);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137726);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t53::n97197(s, out),
		['р', s @ ..] => {
			super::push(out, 96171);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114640);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126475);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 133534);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137737);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96172);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 114641);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126476);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133535);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 137738);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 126477);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133536);
									s.is_empty()
								}
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
			super::push(out, 96173);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114642);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126478);
							match s {
								[] => true,
								['п', 'с', 'а', 'р', '-', s @ ..] => {
									super::push(out, 133537);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137739);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140250);
													match s {
														[] => true,
														['т', s @ ..] => super::t53::n143196(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -житель.
pub(crate) fn n71885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71137);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96174);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114643);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 126479);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133538);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137740);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96175);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114644);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126480);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133539);
									s.is_empty()
								}
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
			super::push(out, 96176);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114645);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 126481);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133540);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 114646);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126482);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133541);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137741);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 126483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96177);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 114647);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 114648);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 114649);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 114650);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 126486);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133542);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 133543);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137742);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140251);
													match s {
														[] => true,
														['-', s @ ..] => super::t53::n143197(s, out),
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
									super::push(out, 133544);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 137743);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 133545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 126487);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 133546);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 137744);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 140252);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114651);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 114652);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 126488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 96178);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 96179);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114653);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114654);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126490);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133547);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 137745);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140253);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96180);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 114655);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126491);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сказитель.
pub(crate) fn n127796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126492);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 133549);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 137746);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140254);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141750);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142623);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 143090);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узитель.
pub(crate) fn n97209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96182);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 114658);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 126493);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 133550);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137747);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -китель.
pub(crate) fn n71887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71139);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 96183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -литель.
pub(crate) fn n71888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71140);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96184);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114659);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 126494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 96185);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 114660);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126495);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 133551);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137748);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 140256);
													match s {
														[] => true,
														['ь', s @ ..] => super::t53::n143199(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'е', 'м', '\u{1}', s @ ..] => {
			super::push(out, 96186);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 96187);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126496);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 114662);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126497);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137749);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 137750);
											match s {
												[] => true,
												['а', 'р', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n143200(s, out),
														['-', s @ ..] => super::t53::n143201(s, out),
														['о', s @ ..] => super::t53::n143202(s, out),
														_ => false,
													}
												}
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
							super::push(out, 126498);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133553);
									match s {
										[] => true,
										['о', 'х', s @ ..] => {
											super::push(out, 137751);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 140257);
													match s {
														[] => true,
														['д', s @ ..] => super::t53::n143203(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 114663);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126499);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 126500);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 133555);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 137752);
											s.is_empty()
										}
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
					super::push(out, 114664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126501);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 126502);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133556);
									s.is_empty()
								}
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
			super::push(out, 96188);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 114665);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 126503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96189);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114666);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126504);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 133557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 114667);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 114668);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 126505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 96190);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114669);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t53::n97218(s, out),
		['т', 'с', s @ ..] => {
			super::push(out, 96192);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 114672);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 96193);
			match s {
				[] => true,
				['х', s @ ..] => super::t53::n115893(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t53::n97221(s, out),
		['ы', s @ ..] => super::t53::n97222(s, out),
		_ => false,
	}
}

/// Words ending -митель.
pub(crate) fn n71889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71141);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 96196);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 114676);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 96197);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 96198);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114678);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 126515);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 133564);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 137756);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нитель.
pub(crate) fn n71890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71142);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96199);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114680);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126517);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 133565);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137757);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 126518);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 133566);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 137758);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140262);
													match s {
														[] => true,
														['г', s @ ..] => super::t53::n143206(s, out),
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
									super::push(out, 133567);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137759);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140263);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143207(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 137760);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140264);
													match s {
														[] => true,
														['т', s @ ..] => super::t53::n143208(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96200);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114681);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126519);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 114682);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 96201);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114683);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126521);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 126522);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 126523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 114684);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126524);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 114685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 96202);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 114686);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126525);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 133569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 114687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 114688);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126526);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 133570);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137761);
											s.is_empty()
										}
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
					super::push(out, 114689);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126527);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 133571);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137762);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96203);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114690);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 126528);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 114691);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133573);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 137763);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ъ', 'б', s @ ..] => {
									super::push(out, 133574);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137764);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140265);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114692);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 126529);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 133575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 114693);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 126530);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 126531);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 133577);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t53::n97232(s, out),
		['м', s @ ..] => {
			super::push(out, 96205);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114695);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126532);
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
				['г', s @ ..] => {
					super::push(out, 114696);
					s.is_empty()
				}
				['л', s @ ..] => super::t53::n115917(s, out),
				['р', s @ ..] => super::t53::n115918(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t53::n97235(s, out),
		['р', s @ ..] => super::t53::n97236(s, out),
		['с', s @ ..] => super::t53::n97237(s, out),
		['т', s @ ..] => super::t53::n97238(s, out),
		_ => false,
	}
}

/// Words ending -оитель.
pub(crate) fn n71891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71143);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 96210);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 114703);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 126540);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133589);
									s.is_empty()
								}
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
			super::push(out, 96211);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133590);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 137772);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 140272);
													match s {
														[] => true,
														['а', 'м', '-', s @ ..] => super::t53::n143215(s, out),
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
									super::push(out, 133591);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 137773);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 137774);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 140273);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n143216(s, out),
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
									super::push(out, 133592);
									match s {
										[] => true,
										['е', 'л', 'о', s @ ..] => {
											match s {
												[] => true,
												['м', 'а', 'с', '\u{1}', s @ ..] => {
													super::push(out, 140274);
													s.is_empty()
												}
												['т', 'р', 'е', 'в', '\u{1}', s @ ..] => {
													super::push(out, 140275);
													s.is_empty()
												}
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
							super::push(out, 126541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -питель.
pub(crate) fn n71892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71144);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96212);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 96213);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114705);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 126543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96214);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114706);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133593);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 133594);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137775);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140276);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n143217(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 137776);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140277);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114707);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 96215);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114708);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126545);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ритель.
pub(crate) fn n71893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71145);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 114709);
					s.is_empty()
				}
				['п', 'с', s @ ..] => {
					super::push(out, 114710);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 126546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 114711);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126547);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133596);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 137777);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96216);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114712);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 126548);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 133597);
									s.is_empty()
								}
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
			super::push(out, 96217);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114713);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 114714);
					match s {
						[] => true,
						['з', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133598);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 133599);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 137778);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 140278);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96218);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 114716);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 114717);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126551);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133600);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137779);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96219);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['о', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133601);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 133602);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 137780);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 140279);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143218(s, out),
														_ => false,
													}
												}
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
								['о', s @ ..] => {
									super::push(out, 133603);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 137781);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140280);
													match s {
														[] => true,
														['л', s @ ..] => super::t53::n143219(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 137782);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 133604);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137783);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140281);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114718);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126552);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133605);
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
						['о', s @ ..] => {
							super::push(out, 126553);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 126554);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 133607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 114719);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 114720);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 96220);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114721);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 126557);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 133608);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137784);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 140282);
													match s {
														[] => true,
														['и', s @ ..] => super::t53::n143220(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96221);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 96222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ситель.
pub(crate) fn n71894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71146);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96223);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 114722);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126558);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 114723);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 126559);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133610);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137785);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 140283);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114724);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114725);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 126561);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133611);
									s.is_empty()
								}
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
			super::push(out, 96224);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 114726);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133612);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 133613);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 137786);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140284);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143221(s, out),
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
									super::push(out, 133614);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 137787);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140285);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143222(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 137788);
											s.is_empty()
										}
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
					super::push(out, 114727);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 126562);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 133615);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137789);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 140286);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96225);
			match s {
				[] => true,
				['н', s @ ..] => super::t53::n115952(s, out),
				['р', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126567);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 126568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 96226);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114728);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126569);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133631);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137808);
											match s {
												[] => true,
												['й', s @ ..] => super::t53::n141743(s, out),
												_ => false,
											}
										}
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
					super::push(out, 114729);
					match s {
						[] => true,
						['б', s @ ..] => super::t53::n127883(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -титель.
pub(crate) fn n71895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71147);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96227);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 96228);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 96229);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 114731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 114732);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 114733);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 126571);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133632);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 126572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 96230);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 96231);
			match s {
				[] => true,
				['а', 'л', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126573);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 126574);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 133633);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137810);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 140299);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114736);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 126575);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133634);
									match s {
										[] => true,
										['з', '-', s @ ..] => {
											super::push(out, 137811);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140300);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n143232(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 114737);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 126576);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 114738);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 126577);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133636);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137812);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 133637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 126578);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133638);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137813);
											s.is_empty()
										}
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
					super::push(out, 114739);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 114740);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126580);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 133639);
									s.is_empty()
								}
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
			super::push(out, 96232);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 114741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 96233);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -читель.
pub(crate) fn n71896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71148);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96234);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 96235);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 114743);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 126581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 96236);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 114744);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 96237);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 126583);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133640);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137814);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 126584);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 133641);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 137815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 126585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 114745);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126586);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133642);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 137816);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96238);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 114746);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 96239);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 114747);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 114748);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126588);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133643);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 137817);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140301);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114749);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 114750);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126589);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 126590);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133644);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 137818);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140302);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114751);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 96240);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 114752);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126592);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 133645);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137819);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 140303);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шитель.
pub(crate) fn n71897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71149);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['р', 'з', s @ ..] => {
					super::push(out, 114753);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126593);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 114754);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 126594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114755);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126595);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133647);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 114756);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126596);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133648);
									s.is_empty()
								}
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
			super::push(out, 96241);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114757);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 96242);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 114758);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 114759);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 126598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126599);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 126600);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133649);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 126601);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 126602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 114761);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126603);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 133650);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 137820);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 126604);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 133651);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137821);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 140304);
													match s {
														[] => true,
														['ь', s @ ..] => super::t53::n143233(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 114762);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126605);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133652);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 137822);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140305);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 96243);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 114763);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126606);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 133653);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 137823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щитель.
pub(crate) fn n71898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71150);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 96244);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114764);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126607);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оуровневость.
pub(crate) fn n141770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140325);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ивость.
pub(crate) fn n72808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72056);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 96968);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 115154);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126802);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133749);
									s.is_empty()
								}
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
			super::push(out, 96969);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 115155);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126803);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 133750);
									s.is_empty()
								}
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
			super::push(out, 96970);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115156);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 115157);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126805);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133751);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137868);
											s.is_empty()
										}
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
					super::push(out, 115158);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126806);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 133752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 115159);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 96971);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t53::n98007(s, out),
		['н', s @ ..] => {
			super::push(out, 96973);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115180);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 115181);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 115182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 96974);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 115183);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 126853);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 96975);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115184);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126854);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 133827);
									s.is_empty()
								}
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
			super::push(out, 96976);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 115185);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 115186);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 126856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 96977);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 115187);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 115188);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 126857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 115189);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 115190);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126858);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 137927);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 140356);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 137928);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140357);
													match s {
														[] => true,
														['м', s @ ..] => super::t53::n143260(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115191);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 115192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126859);
							match s {
								[] => true,
								['т', 'с', 'у', 'о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137929);
											match s {
												[] => true,
												['о', 'л', s @ ..] => {
													super::push(out, 140358);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143261(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 137930);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 140359);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115193);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 115194);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 115195);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126861);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133828);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 115196);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 115197);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 115198);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 115199);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126862);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133829);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137931);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96978);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115200);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 115201);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овость.
pub(crate) fn n72809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72057);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 96979);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 115202);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 126864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 96980);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115203);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 126865);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 115204);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126866);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 133831);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 137932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 126867);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 133832);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 137933);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 140360);
													s.is_empty()
												}
												_ => false,
											}
										}
										['й', s @ ..] => {
											super::push(out, 137934);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140361);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143262(s, out),
														_ => false,
													}
												}
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
							super::push(out, 126868);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133833);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137935);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 140362);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115205);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126869);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133834);
									s.is_empty()
								}
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
				['е', s @ ..] => {
					super::push(out, 115206);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 126870);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 126871);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133835);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 137936);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 126872);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 133836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 115207);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 126873);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 133837);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137937);
											s.is_empty()
										}
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
					super::push(out, 115208);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 126874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 96981);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 115209);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 126875);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133838);
									s.is_empty()
								}
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
			super::push(out, 96982);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 96983);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115211);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 126876);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 115212);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126877);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 133840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 115213);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126878);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133841);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137938);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 140363);
													s.is_empty()
												}
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
							super::push(out, 126879);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 133842);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 126880);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 133843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 115214);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126881);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 133844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 115215);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126882);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126883);
							match s {
								[] => true,
								['х', s @ ..] => super::t53::n135228(s, out),
								_ => false,
							}
						}
						['ч', s @ ..] => super::t53::n128206(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t53::n116447(s, out),
				['т', s @ ..] => super::t53::n116448(s, out),
				['ч', s @ ..] => super::t53::n116449(s, out),
				['ы', s @ ..] => super::t53::n116450(s, out),
				['я', s @ ..] => super::t53::n116451(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t53::n98020(s, out),
		['м', s @ ..] => super::t53::n98021(s, out),
		['н', s @ ..] => super::t53::n98022(s, out),
		['п', s @ ..] => super::t53::n98023(s, out),
		['р', s @ ..] => super::t53::n98024(s, out),
		['с', s @ ..] => super::t53::n98025(s, out),
		['т', s @ ..] => super::t53::n98026(s, out),
		['ц', s @ ..] => super::t53::n98027(s, out),
		_ => false,
	}
}

/// Words ending -иперплоскость.
pub(crate) fn n143265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141817);
	s.is_empty()
}

/// Words ending -лесневелость.
pub(crate) fn n141816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140371);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141818);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142658);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 143112);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 143338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -выветрелость.
pub(crate) fn n141817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140372);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141819);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ваемость.
pub(crate) fn n116505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115274);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126926);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 133879);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 137954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 126927);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 133880);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137955);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 140373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 133881);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 137956);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140374);
									s.is_empty()
								}
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
			super::push(out, 126928);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133882);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137957);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140375);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141820);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 137958);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140376);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141821);
											s.is_empty()
										}
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
					super::push(out, 133883);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 137959);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140377);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141822);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142659);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 133884);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 137960);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 140378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 137961);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140379);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141823);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142660);
													s.is_empty()
												}
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
							super::push(out, 137962);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 133885);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 133886);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 133887);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 133888);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 133889);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 137964);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140381);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 141824);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 142661);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n144566(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 126929);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 133890);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137965);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140382);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133891);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137966);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140383);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 137967);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 133892);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 133893);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 137968);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140385);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 141825);
											s.is_empty()
										}
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
					super::push(out, 133894);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 137969);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 137970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 133895);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 133896);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137971);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 140386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 137972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цаемость.
pub(crate) fn n116516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115285);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 126935);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чаемость.
pub(crate) fn n116517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115286);
	s.is_empty()
}

/// Words ending -шаемость.
pub(crate) fn n116518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115287);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 126936);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щаемость.
pub(crate) fn n116519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115288);
	s.is_empty()
}

/// Words ending -лемость.
pub(crate) fn n98068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97031);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 115289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уемость.
pub(crate) fn n98069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97032);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 115290);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 115291);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 126937);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 133897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 126938);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 115292);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 126939);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 133899);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 137973);
									s.is_empty()
								}
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
			super::push(out, 115293);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 126940);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 133900);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 137974);
									s.is_empty()
								}
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
			super::push(out, 115294);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 126941);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 133901);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137975);
									s.is_empty()
								}
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
			super::push(out, 115295);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 126942);
					match s {
						[] => true,
						['л', 'у', s @ ..] => {
							super::push(out, 133902);
							match s {
								[] => true,
								['г', 'е', s @ ..] => {
									super::push(out, 137976);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											super::push(out, 140387);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 133903);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137977);
									match s {
										[] => true,
										['г', 'и', 'ф', 'н', 'о', 'к', '\u{1}', s @ ..] => {
											super::push(out, 140388);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 133904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 115296);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 126943);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 133905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 126944);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 133906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яемость.
pub(crate) fn n98070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97033);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 115297);
			match s {
				[] => true,
				['в', 'а', s @ ..] => {
					super::push(out, 126945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 115298);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', 'о', s @ ..] => {
									super::push(out, 137978);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 140389);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 137979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 133907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 115299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -имость.
pub(crate) fn n72850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72098);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 97034);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 97035);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 97036);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 115300);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126946);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133908);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 137980);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 133909);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 137981);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 140390);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n143274(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115301);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126947);
							match s {
								[] => true,
								['о', 'р', 'п', 'о', s @ ..] => {
									super::push(out, 133910);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137982);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 140391);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n143275(s, out),
														_ => false,
													}
												}
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
							super::push(out, 126948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 115302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 97037);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115303);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 126949);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133911);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 115304);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126950);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 133912);
									match s {
										[] => true,
										['е', 'н', 'и', s @ ..] => {
											super::push(out, 137983);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 140392);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n143276(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['и', s @ ..] => {
											super::push(out, 137984);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 140393);
													s.is_empty()
												}
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
							super::push(out, 126951);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 133913);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137985);
											s.is_empty()
										}
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
					super::push(out, 115305);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 115306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 97038);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 97039);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115307);
					s.is_empty()
				}
				['с', 'ы', s @ ..] => {
					super::push(out, 115308);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 126953);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133914);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 137986);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140394);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97040);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 97041);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 115309);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126954);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133915);
									s.is_empty()
								}
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
			super::push(out, 97042);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 115310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 97043);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 97044);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 115311);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126955);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133916);
									match s {
										[] => true,
										['з', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140395);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 140396);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115312);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 126956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 97045);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 115313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 97046);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115314);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 126957);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133917);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137987);
											s.is_empty()
										}
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
					super::push(out, 115315);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 115316);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 126959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t53::n98084(s, out),
		_ => false,
	}
}

/// Words ending -омость.
pub(crate) fn n72851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72099);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 97048);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115321);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 97049);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115322);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 115323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 97050);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115324);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126964);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юмость.
pub(crate) fn n72852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72100);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 97051);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 115325);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 126965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ямость.
pub(crate) fn n72853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72101);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 97052);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 115326);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 126966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ненадобность.
pub(crate) fn n141842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140397);
	s.is_empty()
}

/// Words ending -зкопробность.
pub(crate) fn n141843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140398);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141829);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142663);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word платежеспособность.
pub(crate) fn n141844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140399);
	s.is_empty()
}

/// The word неплатежеспособность.
pub(crate) fn n141845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140400);
	s.is_empty()
}

/// The word лежкоспособность.
pub(crate) fn n141846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140401);
	s.is_empty()
}

/// Words ending -тоспособность.
pub(crate) fn n141847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140402);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141830);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 142664);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивность.
pub(crate) fn n98100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97062);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 115337);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 126978);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 133925);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137992);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'г', 'а', s @ ..] => {
							super::push(out, 133926);
							s.is_empty()
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
				['а', 'р', 'е', s @ ..] => {
					super::push(out, 126979);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 133927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 126980);
					s.is_empty()
				}
				['к', 'е', s @ ..] => {
					super::push(out, 126981);
					match s {
						[] => true,
						['п', 'с', 'р', s @ ..] => {
							super::push(out, 133928);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 133929);
							match s {
								[] => true,
								['б', 'о', s @ ..] => {
									super::push(out, 137993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овность.
pub(crate) fn n98101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97063);
	match s {
		[] => true,
		['л', 'с', 'о', s @ ..] => {
			super::push(out, 115338);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 126982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 115339);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 126983);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137994);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 140403);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 137995);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 140404);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 141831);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дность.
pub(crate) fn n72857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72105);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97065);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 115340);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126984);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133930);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137996);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140405);
													match s {
														[] => true,
														['п', s @ ..] => super::t53::n143280(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 97066);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 115342);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 115343);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126985);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133931);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137997);
											s.is_empty()
										}
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
					super::push(out, 115344);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126986);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 133932);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 137998);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140406);
													s.is_empty()
												}
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
							super::push(out, 126987);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 126988);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 133933);
									s.is_empty()
								}
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
			super::push(out, 97067);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 97068);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115345);
					match s {
						[] => true,
						['е', 'ч', s @ ..] => {
							super::push(out, 126989);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 133934);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137999);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140407);
													match s {
														[] => true,
														['д', s @ ..] => super::t53::n143281(s, out),
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
									super::push(out, 133935);
									s.is_empty()
								}
								['н', 'ь', s @ ..] => {
									super::push(out, 133936);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 138000);
											s.is_empty()
										}
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
					super::push(out, 115346);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 115347);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126990);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133937);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 138001);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 126991);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 126992);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133939);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 138002);
											s.is_empty()
										}
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
					super::push(out, 115348);
					match s {
						[] => true,
						['о', 'б', 'е', 'м', 'а', '\u{1}', s @ ..] => {
							super::push(out, 126993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 97069);
			match s {
				[] => true,
				['в', 'о', 'р', 'п', 'о', 'р', 'т', 'к', 'е', 'л', 'э', 'о', s @ ..] => {
					super::push(out, 115349);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 126994);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133940);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 138003);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 140408);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n143282(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115350);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126995);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 126996);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133941);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 138004);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 126997);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 133942);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138005);
											s.is_empty()
										}
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
					super::push(out, 115351);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126998);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 133943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 115352);
					s.is_empty()
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126999);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 127000);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 127001);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 127002);
							match s {
								[] => true,
								['в', s @ ..] => super::t53::n135329(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t53::n98108(s, out),
		['у', s @ ..] => super::t53::n98109(s, out),
		['ю', s @ ..] => super::t53::n98110(s, out),
		['я', s @ ..] => super::t53::n98111(s, out),
		_ => false,
	}
}

/// Words ending -еность.
pub(crate) fn n72858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72106);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 97074);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 97075);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 115356);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 115357);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127005);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 133946);
									s.is_empty()
								}
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
			super::push(out, 97076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жность.
pub(crate) fn n72859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72107);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97077);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115358);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 127006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 115359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 97078);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 115360);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 127007);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 115361);
					match s {
						[] => true,
						['а', 'н', 'з', s @ ..] => {
							super::push(out, 127008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 115362);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 115363);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127009);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 133948);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 138007);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140410);
													match s {
														[] => true,
														['б', s @ ..] => super::t53::n143284(s, out),
														_ => false,
													}
												}
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
							super::push(out, 127010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 115364);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 127011);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 127012);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 133949);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138008);
											s.is_empty()
										}
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
					super::push(out, 115365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 97079);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 97080);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 115366);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 127013);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133950);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 133951);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138009);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 97081);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 115367);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127014);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133952);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138010);
											s.is_empty()
										}
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
					super::push(out, 115368);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 115369);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 127015);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 127016);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 133953);
									match s {
										[] => true,
										['е', 'в', 'о', s @ ..] => {
											super::push(out, 138011);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 140411);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n143285(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115370);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127017);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 133954);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 138012);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 140412);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n143286(s, out),
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
									super::push(out, 133955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 115371);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 127018);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133956);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 138013);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 97082);
			match s {
				[] => true,
				['е', 'д', 'у', s @ ..] => {
					super::push(out, 115372);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 127019);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133957);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 138014);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140413);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97083);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 115373);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127020);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133958);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138015);
											s.is_empty()
										}
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
					super::push(out, 115374);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зность.
pub(crate) fn n72860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72108);
	match s {
		[] => true,
		['а', 'р', 'б', 'о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 115375);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 115376);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127021);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									super::push(out, 133959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 115377);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 127022);
							match s {
								[] => true,
								['е', 'л', s @ ..] => {
									super::push(out, 133960);
									s.is_empty()
								}
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
			super::push(out, 97084);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 97085);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 97086);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115379);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 127023);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133961);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 138016);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140414);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97087);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 115380);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 127024);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 133962);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138017);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 97088);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иность.
pub(crate) fn n72861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72109);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 97089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йность.
pub(crate) fn n72862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72110);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97090);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115381);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127025);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 133963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 115382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 97091);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 97092);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 97093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кность.
pub(crate) fn n72863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72111);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 97094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мность.
pub(crate) fn n72864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72112);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 97095);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 97096);
			s.is_empty()
		}
		['о', 'т', s @ ..] => {
			super::push(out, 97097);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115383);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 127026);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133964);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 138018);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 127027);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 133965);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138019);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 140415);
													match s {
														[] => true,
														['м', s @ ..] => super::t53::n143287(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нность.
pub(crate) fn n72865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72113);
	match s {
		[] => true,
		['а', s @ ..] => super::t53::n98138(s, out),
		['е', s @ ..] => super::t53::n98139(s, out),
		['и', s @ ..] => {
			super::push(out, 97101);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115419);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127175);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 134205);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 127176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', 'ю', s @ ..] => {
					super::push(out, 115420);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 127177);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 134206);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 138290);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140642);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 127178);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134207);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 138291);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 127179);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 115421);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 115422);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 115423);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 127180);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 134209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 115424);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 115425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 97102);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 115426);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 115427);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 115428);
					s.is_empty()
				}
				['р', 'о', s @ ..] => {
					super::push(out, 115429);
					match s {
						[] => true,
						['т', 'с', 'ю', s @ ..] => {
							super::push(out, 127181);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 134210);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138292);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 140643);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n143423(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97103);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 115430);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 127182);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 134211);
									s.is_empty()
								}
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
			super::push(out, 97104);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 115431);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 127183);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 134212);
									s.is_empty()
								}
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
			super::push(out, 97105);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115432);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 127184);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 127185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 115433);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127186);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 134213);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 138293);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 140644);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115434);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 127187);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 127188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 115435);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 127189);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 134214);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 138294);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 127190);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134215);
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
						['е', s @ ..] => {
							super::push(out, 127191);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 127192);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 134216);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 138295);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 140645);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115436);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 127193);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 134217);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 138296);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 140646);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115437);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 127194);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 134218);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138297);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 140647);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115438);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127195);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пность.
pub(crate) fn n72866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72114);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 97106);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рность.
pub(crate) fn n72867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72115);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97107);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 115439);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 127196);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 134220);
									match s {
										[] => true,
										['б', '\u{1}', s @ ..] => {
											super::push(out, 138298);
											s.is_empty()
										}
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
					super::push(out, 115440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 97108);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 115441);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127197);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 134221);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 138299);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 140648);
													s.is_empty()
												}
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
							super::push(out, 127198);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 134222);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 115442);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127199);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 134223);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 138300);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 127200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 115443);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 127201);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 134224);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 138301);
											s.is_empty()
										}
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
					super::push(out, 115444);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 127202);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 134225);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 138302);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140649);
													s.is_empty()
												}
												['и', s @ ..] => {
													super::push(out, 140650);
													match s {
														[] => true,
														['б', s @ ..] => super::t53::n143424(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 140651);
													s.is_empty()
												}
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
							super::push(out, 127203);
							match s {
								[] => true,
								['ъ', s @ ..] => {
									super::push(out, 134226);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 138303);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140652);
													match s {
														[] => true,
														['б', s @ ..] => super::t53::n143425(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115445);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 127204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 134227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 115446);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 127205);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 134228);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138304);
											s.is_empty()
										}
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
					super::push(out, 115447);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127206);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 134229);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 138305);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140653);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n143426(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115448);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127207);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 134230);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138306);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 127208);
							s.is_empty()
						}
						['о', 'н', '\u{1}', s @ ..] => {
							super::push(out, 127209);
							s.is_empty()
						}
						['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 127210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 115449);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 115450);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 127211);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134231);
									match s {
										[] => true,
										['р', 'а', 'х', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140654);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 140655);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t53::n128544(s, out),
						['ю', s @ ..] => super::t53::n128545(s, out),
						_ => false,
					}
				}
				['ф', 'у', s @ ..] => super::t53::n116688(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 97109);
			s.is_empty()
		}
		['о', s @ ..] => super::t53::n98149(s, out),
		['у', s @ ..] => super::t53::n98150(s, out),
		['ю', s @ ..] => {
			super::push(out, 97112);
			s.is_empty()
		}
		['я', s @ ..] => super::t53::n98152(s, out),
		_ => false,
	}
}

/// Words ending -сность.
pub(crate) fn n72868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72116);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97114);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115464);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 115465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 97115);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115466);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 115467);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 115468);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 127232);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 134250);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138319);
											s.is_empty()
										}
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
					super::push(out, 115469);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 127233);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134251);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 138320);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140663);
													s.is_empty()
												}
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
							super::push(out, 127234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 97116);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 115470);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127235);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 127236);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 134253);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 138321);
											s.is_empty()
										}
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
					super::push(out, 115471);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 97117);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115472);
					match s {
						[] => true,
						['л', 'п', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 134254);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 134255);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 138322);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 134256);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 138323);
											s.is_empty()
										}
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
					super::push(out, 115473);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 127238);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134257);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 138324);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 97118);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115474);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 115475);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 134258);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 134259);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 138325);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 140664);
													match s {
														[] => true,
														['л', s @ ..] => super::t53::n143430(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', 'о', s @ ..] => {
									super::push(out, 134260);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 138326);
											s.is_empty()
										}
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
					super::push(out, 115476);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127239);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 134261);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138327);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 97119);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115477);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 127240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 115478);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 127241);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134262);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138328);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 134263);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138329);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 140665);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97120);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115479);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 127242);
							match s {
								[] => true,
								['к', 'о', s @ ..] => {
									super::push(out, 134264);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 138330);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140666);
													match s {
														[] => true,
														['е', s @ ..] => super::t53::n143431(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t53::n98160(s, out),
		['ю', s @ ..] => {
			super::push(out, 97122);
			match s {
				[] => true,
				['л', s @ ..] => super::t53::n116723(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t53::n98162(s, out),
		_ => false,
	}
}

/// Words ending -тность.
pub(crate) fn n72869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72117);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97124);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115487);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 115488);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 127252);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134271);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 138334);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140667);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 115489);
					match s {
						[] => true,
						['в', 'о', s @ ..] => {
							super::push(out, 127253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 115490);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 127254);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134272);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 138335);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140668);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97125);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115491);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 115492);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 115493);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127255);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 134273);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 138336);
											s.is_empty()
										}
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
					super::push(out, 115494);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 115495);
					match s {
						[] => true,
						['и', 'р', 'о', s @ ..] => {
							super::push(out, 127256);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134274);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 138337);
											s.is_empty()
										}
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
					super::push(out, 115496);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 127257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 134275);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 134276);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 138338);
											s.is_empty()
										}
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
					super::push(out, 115497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 97126);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 97127);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115498);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 115499);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 127258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 97128);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115500);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 127259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 127260);
							match s {
								[] => true,
								['а', 'в', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 138339);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 140669);
													match s {
														[] => true,
														['м', s @ ..] => super::t53::n143432(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
											super::push(out, 138340);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 127261);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134277);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138341);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140670);
													match s {
														[] => true,
														['п', s @ ..] => super::t53::n143433(s, out),
														_ => false,
													}
												}
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
							super::push(out, 127262);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 127263);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134278);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138342);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 140671);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143434(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t53::n98168(s, out),
		['р', s @ ..] => {
			super::push(out, 97130);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115510);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => super::t53::n128611(s, out),
						['с', s @ ..] => {
							super::push(out, 127277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t53::n98170(s, out),
		['у', s @ ..] => {
			super::push(out, 97132);
			s.is_empty()
		}
		['ы', s @ ..] => super::t53::n98172(s, out),
		['ю', s @ ..] => {
			super::push(out, 97134);
			s.is_empty()
		}
		['я', s @ ..] => super::t53::n98174(s, out),
		_ => false,
	}
}

/// Words ending -фность.
pub(crate) fn n72870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72118);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 97136);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 115526);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хность.
pub(crate) fn n72871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72119);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 97137);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 115527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чность.
pub(crate) fn n72872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72120);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97138);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 115528);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127301);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 138363);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 140681);
													match s {
														[] => true,
														['ь', s @ ..] => super::t53::n143441(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 138364);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 127302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 115529);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 127303);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134312);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 115530);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 127304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 134313);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 134314);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138365);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 140682);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n143442(s, out),
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
									super::push(out, 134315);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138366);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', 'у', s @ ..] => {
							super::push(out, 127305);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 134316);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138367);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 97139);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115531);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 115532);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 127306);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134317);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 138368);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140683);
													s.is_empty()
												}
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
							super::push(out, 127307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 115533);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127308);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 134318);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 138369);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 140684);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n143443(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 115534);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 115535);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127309);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 134319);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138370);
											s.is_empty()
										}
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
					super::push(out, 115536);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 127310);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 134320);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 138371);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 140685);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n143444(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97140);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115537);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127311);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 134321);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138372);
											s.is_empty()
										}
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
					super::push(out, 115538);
					match s {
						[] => true,
						['д', s @ ..] => super::t53::n128649(s, out),
						['о', s @ ..] => {
							super::push(out, 127313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 115539);
					s.is_empty()
				}
				['ф', s @ ..] => super::t53::n116783(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t53::n98180(s, out),
		['о', s @ ..] => super::t53::n98181(s, out),
		['у', s @ ..] => super::t53::n98182(s, out),
		['ы', s @ ..] => super::t53::n98183(s, out),
		_ => false,
	}
}

/// Words ending -шность.
pub(crate) fn n72873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72121);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97145);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 97146);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 115554);
					s.is_empty()
				}
				['р', 'г', 'з', s @ ..] => {
					super::push(out, 115555);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127341);
							match s {
								[] => true,
								['б', 'е', s @ ..] => {
									super::push(out, 134355);
									s.is_empty()
								}
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
			super::push(out, 97147);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115556);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 115557);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 127342);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134356);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 138396);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 140700);
													match s {
														[] => true,
														['в', s @ ..] => super::t53::n143452(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97148);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 97149);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 115558);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 127343);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 134357);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 138397);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140701);
													match s {
														[] => true,
														['п', s @ ..] => super::t53::n143453(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щность.
pub(crate) fn n72874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72122);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 97150);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 115559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 97151);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 115560);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127344);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 134358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 115561);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127345);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 134359);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 138398);
											s.is_empty()
										}
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
					super::push(out, 115562);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127346);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 134360);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 138399);
											s.is_empty()
										}
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
					super::push(out, 115563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 97152);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 115564);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127347);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 134361);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138400);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 127348);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 134362);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138401);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140702);
													match s {
														[] => true,
														['н', s @ ..] => super::t53::n143454(s, out),
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
									super::push(out, 134363);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 138402);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 140703);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 134364);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 138403);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140704);
													match s {
														[] => true,
														['б', s @ ..] => super::t53::n143455(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97153);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 115565);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 127349);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 134365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 115566);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 127350);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 134366);
									s.is_empty()
								}
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
			super::push(out, 97154);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 115567);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьность.
pub(crate) fn n72875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72123);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 97155);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115568);
					match s {
						[] => true,
						['д', 'о', s @ ..] => {
							super::push(out, 127351);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 134367);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 127352);
							match s {
								[] => true,
								['п', s @ ..] => super::t53::n135770(s, out),
								['ц', s @ ..] => {
									super::push(out, 134369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 127353);
							s.is_empty()
						}
						['м', s @ ..] => super::t53::n128693(s, out),
						['н', s @ ..] => super::t53::n128694(s, out),
						['р', s @ ..] => super::t53::n128695(s, out),
						['с', s @ ..] => super::t53::n128696(s, out),
						['т', s @ ..] => super::t53::n128697(s, out),
						['у', s @ ..] => super::t53::n128698(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t53::n116813(s, out),
				['и', s @ ..] => super::t53::n116814(s, out),
				['л', s @ ..] => super::t53::n116815(s, out),
				['о', s @ ..] => super::t53::n116816(s, out),
				['у', s @ ..] => super::t53::n116817(s, out),
				['ы', s @ ..] => super::t53::n116818(s, out),
				['я', s @ ..] => super::t53::n116819(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юность.
pub(crate) fn n72876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72124);
	s.is_empty()
}

/// Words ending -яность.
pub(crate) fn n72877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72125);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 97156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -род-крепость.
pub(crate) fn n142180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140735);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142029);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142770);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реливчатость.
pub(crate) fn n142181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140736);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 142030);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142771);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word выемчатость.
pub(crate) fn n142182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140737);
	s.is_empty()
}

/// Words ending -стость.
pub(crate) fn n72895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72141);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 97176);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 115597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 97177);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 115598);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127400);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 134417);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 138461);
											s.is_empty()
										}
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
					super::push(out, 115599);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 127401);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 127402);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 134418);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 138462);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140738);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', 'о', 'н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 138463);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 138464);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140739);
													s.is_empty()
												}
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
							super::push(out, 127403);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 134419);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 138465);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 127404);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 134420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 115600);
					match s {
						[] => true,
						['а', s @ ..] => super::t53::n128745(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t53::n116846(s, out),
				['ж', s @ ..] => super::t53::n116847(s, out),
				['з', s @ ..] => super::t53::n116848(s, out),
				['л', s @ ..] => super::t53::n116849(s, out),
				['м', s @ ..] => super::t53::n116850(s, out),
				['н', s @ ..] => super::t53::n116851(s, out),
				['о', s @ ..] => super::t53::n116852(s, out),
				['п', s @ ..] => super::t53::n116853(s, out),
				['р', s @ ..] => super::t53::n116854(s, out),
				['с', s @ ..] => super::t53::n116855(s, out),
				['т', s @ ..] => super::t53::n116856(s, out),
				['ч', s @ ..] => super::t53::n116857(s, out),
				['ш', s @ ..] => super::t53::n116858(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t53::n98218(s, out),
		['у', s @ ..] => super::t53::n98219(s, out),
		['я', s @ ..] => super::t53::n98220(s, out),
		_ => false,
	}
}

/// Words ending -дутость.
pub(crate) fn n98221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97181);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115615);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 127448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 115616);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127449);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 127450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нутость.
pub(crate) fn n98222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97182);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 115617);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 127451);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134474);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138502);
									s.is_empty()
								}
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
			super::push(out, 115618);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 115619);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 115620);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 127452);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 134475);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 138503);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 140753);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142035);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 115621);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытость.
pub(crate) fn n72897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72143);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 97184);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 115623);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 127453);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134476);
									s.is_empty()
								}
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
			super::push(out, 97185);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятость.
pub(crate) fn n72899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72145);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 97187);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 115624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 97188);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115625);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 127454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 97189);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 97190);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 115626);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 127455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 115627);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 115628);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 127457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -возлюбленная.
pub(crate) fn n142206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140761);
	s.is_empty()
}

/// Words ending -реставленная.
pub(crate) fn n142207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140762);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 142039);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оеннопленная.
pub(crate) fn n142208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140763);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 142040);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рменная.
pub(crate) fn n98416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97373);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 115770);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 127545);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 115771);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127546);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 134526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льменная.
pub(crate) fn n117019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115772);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127547);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 134527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ненная.
pub(crate) fn n73115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72361);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 97375);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 115773);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 127548);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134528);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 138524);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренная.
pub(crate) fn n73116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72362);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 97376);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115774);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127549);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 134529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тенная.
pub(crate) fn n73117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72363);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 97377);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 115775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченная.
pub(crate) fn n73118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72364);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 97378);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 115776);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127550);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 134530);
									s.is_empty()
								}
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
			super::push(out, 97379);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 115777);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127551);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 134531);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 138525);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140764);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97380);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115778);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 127552);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 134532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шенная.
pub(crate) fn n73119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72365);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 97381);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115779);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 127553);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 134533);
									s.is_empty()
								}
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
			super::push(out, 97382);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 115780);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 127554);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 134534);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 138526);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенная.
pub(crate) fn n73120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72366);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 97383);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 115781);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 127555);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 134535);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 138527);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word аэроклиматология.
pub(crate) fn n144232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142779);
	s.is_empty()
}

/// The word тромбофилия.
pub(crate) fn n142235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140790);
	s.is_empty()
}

/// Words ending -ериканофилия.
pub(crate) fn n142236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140791);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 142053);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142782);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инофилия.
pub(crate) fn n117321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116072);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127727);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 134638);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 138575);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140792);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142054);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127728);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ехнофилия.
pub(crate) fn n129078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127730);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 134640);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -янофилия.
pub(crate) fn n117324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116075);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 127731);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 134641);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138576);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оофилия.
pub(crate) fn n98810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97757);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 116076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рофилия.
pub(crate) fn n98811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97758);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 116077);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 127732);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -софилия.
pub(crate) fn n98812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97759);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 116078);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 127733);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тофилия.
pub(crate) fn n98813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97760);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 116079);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127734);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 134643);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138577);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 140794);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142055);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ринебактерия.
pub(crate) fn n142249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140804);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142058);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 143171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зотобактерия.
pub(crate) fn n142250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140805);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142059);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142784);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упербактерия.
pub(crate) fn n142251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140806);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 142060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фтерия.
pub(crate) fn n73826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73053);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 98063);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 116317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 127871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word орнитохория.
pub(crate) fn n142253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140808);
	s.is_empty()
}

/// Words ending -иметрия.
pub(crate) fn n99185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98118);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 116362);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 116363);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 127900);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 134721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138612);
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
						['д', s @ ..] => {
							super::push(out, 134722);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138613);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 138614);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 140809);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 142062);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 138615);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 140810);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 142063);
													s.is_empty()
												}
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
							super::push(out, 134723);
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
				['а', s @ ..] => {
					super::push(out, 127901);
					s.is_empty()
				}
				['о', 'п', s @ ..] => {
					super::push(out, 127902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134724);
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
				['а', s @ ..] => {
					super::push(out, 127903);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 127904);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 134725);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138616);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 127905);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 134726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 127906);
					match s {
						[] => true,
						['л', 'о', 'п', 'о', s @ ..] => {
							super::push(out, 134727);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 138617);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 140811);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 142064);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n144239(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 116364);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 116365);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 116366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ометрия.
pub(crate) fn n99187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98120);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 116368);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 116369);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 116370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 127908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 134728);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 134729);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 134730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 116371);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 116372);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127909);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 134731);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 138618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 127910);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 127911);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134732);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 138619);
									s.is_empty()
								}
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
			super::push(out, 116373);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 127912);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 127913);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 127914);
					match s {
						[] => true,
						['е', 'т', 'с', 'е', s @ ..] => {
							super::push(out, 134733);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 138620);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 140812);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 116374);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 116375);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 127915);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 127916);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 134734);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 134735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 127917);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 134736);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 138622);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 140813);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 134737);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 138623);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 140814);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127918);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 134738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 127919);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 134739);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 138624);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 140815);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142065);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 127920);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 127921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 116376);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 127922);
					s.is_empty()
				}
				['о', 'с', s @ ..] => {
					super::push(out, 127923);
					match s {
						[] => true,
						['к', 'е', s @ ..] => {
							super::push(out, 134740);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 138625);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 140816);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 142066);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n144240(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 116377);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 116378);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 127924);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 134741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 138626);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 127925);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 127926);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 134742);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138627);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 138628);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 140817);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142067);
													s.is_empty()
												}
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
							super::push(out, 134743);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 138629);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 140818);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142068);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => super::t53::n136149(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 127927);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 134745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t53::n129278(s, out),
				['у', s @ ..] => super::t53::n129279(s, out),
				['х', s @ ..] => super::t53::n129280(s, out),
				['э', s @ ..] => super::t53::n129281(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t53::n117633(s, out),
		['т', s @ ..] => super::t53::n117634(s, out),
		['х', s @ ..] => super::t53::n117635(s, out),
		_ => false,
	}
}

/// Words ending -красногоровка.
pub(crate) fn n142317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140872);
	s.is_empty()
}

/// Words ending -перламутровка.
pub(crate) fn n142318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140873);
	s.is_empty()
}

/// Words ending -ончик-бытовка.
pub(crate) fn n142319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140874);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 142103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вагон-бытовка.
pub(crate) fn n142320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140875);
	s.is_empty()
}

/// Words ending -нтгенодиагностика.
pub(crate) fn n144259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142806);
	s.is_empty()
}

/// Words ending -лектродиагностика.
pub(crate) fn n144260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142807);
	s.is_empty()
}

/// The word первопоселенка.
pub(crate) fn n144263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142810);
	s.is_empty()
}

/// The word стретч-пленка.
pub(crate) fn n142356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140911);
	s.is_empty()
}

/// Words ending -ресс-масленка.
pub(crate) fn n142357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140912);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 142128);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --четырехклинка.
pub(crate) fn n143581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142130);
	match s {
		[] => true,
		['а', 'к', 'б', 'ю', '\u{1}', s @ ..] => {
			super::push(out, 142813);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -амма-оболочка.
pub(crate) fn n142386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140941);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142146);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142823);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ийца-одиночка.
pub(crate) fn n142387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140942);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 142147);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 142824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -волк-одиночка.
pub(crate) fn n142388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140943);
	s.is_empty()
}

/// Words ending -отец-одиночка.
pub(crate) fn n142389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140944);
	s.is_empty()
}

/// Words ending -мать-одиночка.
pub(crate) fn n142390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140945);
	s.is_empty()
}

/// Words ending -вопрос-ловушка.
pub(crate) fn n143605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142154);
	s.is_empty()
}

/// Words ending -ловка-пеструшка.
pub(crate) fn n144279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142826);
	s.is_empty()
}

/// Words ending -хлестаковщина.
pub(crate) fn n142420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140975);
	s.is_empty()
}

/// Words ending -кружковщина.
pub(crate) fn n136807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135401);
	s.is_empty()
}

/// Words ending -иковщина.
pub(crate) fn n103729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102639);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 119027);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 129286);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 135402);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 138946);
									s.is_empty()
								}
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
			super::push(out, 119028);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 129287);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 135403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лковщина.
pub(crate) fn n103730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102640);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 119029);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 129288);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 135404);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 138947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нковщина.
pub(crate) fn n103731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102641);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 119030);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 129289);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 129290);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оковщина.
pub(crate) fn n103732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102642);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 119031);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 129291);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 135405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьковщина.
pub(crate) fn n103733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102643);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 119032);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 129292);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 135406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яковщина.
pub(crate) fn n103734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102644);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 119033);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 129293);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 135407);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 119034);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 129294);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 135408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловщина.
pub(crate) fn n79993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79163);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 102645);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 119035);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129295);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 135409);
									s.is_empty()
								}
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
			super::push(out, 102646);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 119036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129296);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 129297);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 129298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 119037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 102647);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119038);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мовщина.
pub(crate) fn n79994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79164);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 102648);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119039);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 129299);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135410);
									s.is_empty()
								}
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
			super::push(out, 102649);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119040);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 129300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 119041);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 129301);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 135411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новщина.
pub(crate) fn n79995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79165);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 102650);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 102651);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 119042);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 129302);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 135412);
									s.is_empty()
								}
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
			super::push(out, 102652);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 119043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 102653);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 102654);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119045);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 129303);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повщина.
pub(crate) fn n79996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79166);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 102655);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119046);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 119047);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 129304);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 135414);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 138948);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 102656);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 119048);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129305);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 135415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровщина.
pub(crate) fn n79997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79167);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 102657);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 119049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129306);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 129307);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135416);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 138949);
											s.is_empty()
										}
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
					super::push(out, 119050);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129308);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 135417);
									s.is_empty()
								}
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
			super::push(out, 102658);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 119051);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 129309);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135418);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 138950);
											s.is_empty()
										}
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
					super::push(out, 119052);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 129310);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135419);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 138951);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 102659);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119053);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 129311);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135420);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 138952);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 102660);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 119054);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 129312);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135421);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 138953);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 140976);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 102661);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119055);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 129313);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 135422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совщина.
pub(crate) fn n79998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79168);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 102662);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119056);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 102663);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 119057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 102664);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 119058);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 129315);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 135423);
									s.is_empty()
								}
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
			super::push(out, 102665);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 119059);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товщина.
pub(crate) fn n79999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79169);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 102666);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 119060);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 129317);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 135424);
									s.is_empty()
								}
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
			super::push(out, 102667);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119061);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 129318);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135425);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 138954);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 102668);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119062);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 129319);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 135426);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 138955);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 102669);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 119063);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 129320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 102670);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 119064);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 129321);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 119065);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 129322);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 135428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 119066);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 129323);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 135429);
									s.is_empty()
								}
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
			super::push(out, 102671);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 119067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фовщина.
pub(crate) fn n80000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79170);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 102672);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 119068);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховщина.
pub(crate) fn n80001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79171);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 102673);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119069);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129325);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 135430);
									s.is_empty()
								}
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
			super::push(out, 102674);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 119070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 102675);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 119071);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цовщина.
pub(crate) fn n80002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79172);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 102676);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119072);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 129326);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 135431);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 138956);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140977);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отечественница.
pub(crate) fn n143627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142176);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142834);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 143186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ана-участница.
pub(crate) fn n142455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141010);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 142179);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142836);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осочница.
pub(crate) fn n104804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103711);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 119700);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 129668);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 135579);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 139021);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141011);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усочница.
pub(crate) fn n104805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103712);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 119701);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 129669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясочница.
pub(crate) fn n104806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103713);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 119702);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 129670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -точница.
pub(crate) fn n81410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80569);
	match s {
		[] => true,
		['а', 'з', s @ ..] => {
			super::push(out, 103714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 119703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 103715);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 119704);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 129671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 119705);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 119706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 103716);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 103717);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 119707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 119709);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 103718);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 119710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 103719);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119711);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 129673);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 135580);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 139022);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 141012);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 103720);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 119712);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шочница.
pub(crate) fn n81411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80570);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 103721);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 119713);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

