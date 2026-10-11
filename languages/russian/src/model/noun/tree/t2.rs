use flexion::Path;

/// Words ending -ов.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7314);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25367);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52612);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7315);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25368);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 52613);
									s.is_empty()
								}
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
			super::push(out, 1143);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 7316);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1144);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7317);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25370);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52614);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81108);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 104143);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7318);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7319);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25372);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25373);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52615);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81109);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 104144);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7320);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 7321);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25374);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 52616);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81110);
											s.is_empty()
										}
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
					super::push(out, 7322);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25375);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 52617);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81111);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 104145);
													match s {
														[] => true,
														['о', s @ ..] => super::t13::n121264(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1146);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7323);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25376);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 25377);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7324);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25378);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52619);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 81112);
											s.is_empty()
										}
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
					super::push(out, 7325);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7326);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25380);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 52620);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81113);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 104146);
													match s {
														[] => true,
														['в', s @ ..] => super::t13::n121265(s, out),
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
									super::push(out, 52621);
									s.is_empty()
								}
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
			super::push(out, 1147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t13::n1150(s, out),
		['л', s @ ..] => super::t13::n1151(s, out),
		['м', s @ ..] => {
			super::push(out, 1150);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7354);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25428);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 52670);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81153);
											s.is_empty()
										}
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
					super::push(out, 7355);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25429);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 52671);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81154);
											s.is_empty()
										}
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
					super::push(out, 7356);
					match s {
						[] => true,
						['л', s @ ..] => super::t13::n25599(s, out),
						['н', s @ ..] => super::t13::n25600(s, out),
						['с', s @ ..] => super::t13::n25601(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t13::n1153(s, out),
		['п', s @ ..] => super::t13::n1154(s, out),
		['р', s @ ..] => super::t13::n1155(s, out),
		['с', s @ ..] => super::t13::n1156(s, out),
		['т', s @ ..] => super::t13::n1157(s, out),
		['х', s @ ..] => super::t13::n1158(s, out),
		['ц', s @ ..] => super::t13::n1159(s, out),
		['ш', s @ ..] => super::t13::n1160(s, out),
		_ => false,
	}
}

/// Words ending -рв.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1159);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1160);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7402);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25502);
							match s {
								[] => true,
								['р', '-', s @ ..] => {
									super::push(out, 52733);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 81194);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 104192);
													match s {
														[] => true,
														['в', s @ ..] => super::t13::n121285(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7403);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ув.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1161);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7405);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7406);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25505);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 52734);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 81195);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 104193);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n121286(s, out),
														_ => false,
													}
												}
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
							super::push(out, 25506);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 52735);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81196);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 104194);
													match s {
														[] => true,
														['с', s @ ..] => super::t13::n121287(s, out),
														_ => false,
													}
												}
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
							super::push(out, 25507);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 25508);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 52737);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81197);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 104195);
													match s {
														[] => true,
														['о', s @ ..] => super::t13::n121288(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7407);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25509);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52738);
									s.is_empty()
								}
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
			super::push(out, 1162);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 1163);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7408);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 25510);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хв.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1164);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7409);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыв.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 1165);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7410);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7411);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25512);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'п', 'ы', s @ ..] => {
			super::push(out, 1166);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7412);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1167);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1168);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 7413);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьв.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эв.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1171);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юв.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1172);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25516);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25517);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 52742);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25518);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52743);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 52744);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81198);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 104196);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 52745);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 52746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гуашагаг.
pub(crate) fn n105298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104197);
	s.is_empty()
}

/// Words ending -мдаг.
pub(crate) fn n7445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7423);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25524);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 52749);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 81201);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 104198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдаг.
pub(crate) fn n7449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7427);
	s.is_empty()
}

/// Words ending -заг.
pub(crate) fn n1180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1177);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 7428);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25525);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 52750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7429);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25526);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52751);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 81202);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 104199);
											s.is_empty()
										}
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
					super::push(out, 25527);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 7430);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25528);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 52753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лаг.
pub(crate) fn n1181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1178);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7431);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25529);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52754);
							match s {
								[] => true,
								['х', 'р', 'а', '-', s @ ..] => {
									super::push(out, 81203);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 104200);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 120025);
													match s {
														[] => true,
														['т', s @ ..] => super::t13::n131230(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7432);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 7433);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 25530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 7434);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 7435);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25531);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 52755);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81204);
									s.is_empty()
								}
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
			super::push(out, 7436);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25532);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52756);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25533);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25534);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52758);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81206);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104201);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 120026);
													s.is_empty()
												}
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
							super::push(out, 52759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -маг.
pub(crate) fn n1182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1179);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7437);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 7438);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25535);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 52760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7439);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25536);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 52761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наг.
pub(crate) fn n1183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7440);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 7441);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25537);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 52762);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паг.
pub(crate) fn n1184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1181);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7442);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 25538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 7443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раг.
pub(crate) fn n1185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 7444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25539);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 7445);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25541);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52764);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 81208);
									s.is_empty()
								}
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
			super::push(out, 7446);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 7447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -таг.
pub(crate) fn n1186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7448);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 25543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 52765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7449);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 25544);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 52766);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81209);
									s.is_empty()
								}
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
			super::push(out, 7450);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 7451);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фаг.
pub(crate) fn n1187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1184);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7452);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 7453);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25545);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7454);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7455);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 7456);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7457);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25546);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25547);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 52769);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 81210);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 104202);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120027);
													match s {
														[] => true,
														['а', s @ ..] => super::t13::n131231(s, out),
														_ => false,
													}
												}
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
							super::push(out, 52770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'р', s @ ..] => {
					super::push(out, 25548);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 52771);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 81211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 52772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'л', s @ ..] => {
					super::push(out, 25549);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 52773);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 81212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25550);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 52774);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 52775);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 81214);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 104203);
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
								['м', s @ ..] => {
									super::push(out, 81215);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 81216);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 104204);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 81217);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 104205);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120028);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25551);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 52776);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81218);
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
						['и', s @ ..] => {
							super::push(out, 52777);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 81219);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 104206);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 52778);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81220);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104207);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 81221);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 104208);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 81222);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 104209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 52779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25552);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чаг.
pub(crate) fn n1188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1185);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 7458);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7459);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25554);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шаг.
pub(crate) fn n1189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1186);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7460);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 7461);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25555);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 52780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 7462);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7463);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25557);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 52781);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 81223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гг.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1187);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7464);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7465);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 25558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ег.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1188);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7466);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7467);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7468);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25559);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52782);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81224);
											s.is_empty()
										}
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
					super::push(out, 7469);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25560);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25561);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 52784);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81225);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 104210);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7470);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7471);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25562);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52785);
									s.is_empty()
								}
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
			super::push(out, 1189);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7472);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25563);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 7473);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25564);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 52787);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 81226);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1190);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7474);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25565);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 52788);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 81227);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 104211);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1191);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1192);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 7476);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1193);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7477);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 7478);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25568);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 52789);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81228);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1194);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7479);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 25569);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 52790);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81229);
											s.is_empty()
										}
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
					super::push(out, 7480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1195);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 7481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7482);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7483);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25571);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 52791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 7484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 7485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7486);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25574);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52792);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 52793);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 81230);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 52794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7487);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25575);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52795);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81231);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1197);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7488);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25576);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 52796);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 81232);
											s.is_empty()
										}
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
					super::push(out, 7489);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7490);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25578);
							match s {
								[] => true,
								['с', s @ ..] => super::t13::n53330(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t13::n25749(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зг.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1199);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7491);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25580);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52799);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81234);
											s.is_empty()
										}
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
					super::push(out, 7492);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25581);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25582);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 52800);
									s.is_empty()
								}
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
			super::push(out, 1200);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1201);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25583);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 25584);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 52801);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81235);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104213);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1202);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7495);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1203);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 7496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7497);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иг.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1204);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7498);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25588);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 52802);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81236);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 104214);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1205);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7499);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 7500);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25589);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25590);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 52803);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81237);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25591);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25592);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 25593);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 52805);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81238);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1206);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1207);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1208);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7502);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7503);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25596);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 52806);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81239);
											s.is_empty()
										}
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
					super::push(out, 7504);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25597);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52807);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 25598);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52808);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7505);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25599);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 7506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7507);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25601);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7508);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 7509);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1209);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1210);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7510);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25605);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 52810);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81240);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1211);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 7511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1212);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7512);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25606);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52811);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 81241);
											s.is_empty()
										}
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
					super::push(out, 7513);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 25607);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 52812);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81242);
											s.is_empty()
										}
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
					super::push(out, 7514);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25608);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52813);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81243);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1214);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7516);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25609);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52814);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25610);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 52815);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 81244);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 104215);
													match s {
														[] => true,
														['х', s @ ..] => super::t13::n121294(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => super::t13::n1218(s, out),
		['ц', s @ ..] => super::t13::n1219(s, out),
		['ч', s @ ..] => super::t13::n1220(s, out),
		['щ', s @ ..] => super::t13::n1221(s, out),
		_ => false,
	}
}

/// Words ending -йг.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7523);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 25616);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 52820);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81245);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 104216);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7524);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лг.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1220);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7525);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25618);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 52821);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 81246);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104217);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1221);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25619);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25620);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25621);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52822);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 81247);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нг.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1222);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7527);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25622);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 25623);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 25624);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 52823);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 81248);
											s.is_empty()
										}
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
					super::push(out, 7528);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 25625);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52824);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 81249);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 104218);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7529);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 25626);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 52825);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81250);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 104219);
													match s {
														[] => true,
														['в', s @ ..] => super::t13::n121295(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7530);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 7531);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25627);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 52826);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81251);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 104220);
													match s {
														[] => true,
														['м', s @ ..] => super::t13::n121296(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7532);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25628);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 7533);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25629);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 25630);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25631);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 52828);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 81252);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104221);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7534);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25632);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 52829);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 81253);
											s.is_empty()
										}
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
					super::push(out, 7535);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 7536);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25633);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52830);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 81254);
											s.is_empty()
										}
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
					super::push(out, 7537);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 52831);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 81255);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 104222);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 52832);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81256);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 104223);
													s.is_empty()
												}
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
							super::push(out, 25634);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52833);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 52834);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 81257);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104224);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7538);
					s.is_empty()
				}
				['т', s @ ..] => super::t13::n7561(s, out),
				['ф', s @ ..] => {
					super::push(out, 7540);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 7541);
					match s {
						[] => true,
						['н', s @ ..] => super::t13::n25813(s, out),
						['т', s @ ..] => {
							super::push(out, 25643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t13::n1226(s, out),
		['и', s @ ..] => super::t13::n1227(s, out),
		['о', s @ ..] => super::t13::n1228(s, out),
		['у', s @ ..] => super::t13::n1229(s, out),
		_ => false,
	}
}

/// Words ending -ог.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1227);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 7581);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25752);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52954);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81357);
											s.is_empty()
										}
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
					super::push(out, 7582);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 25753);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 52955);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81358);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1228);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7583);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25754);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 52956);
									s.is_empty()
								}
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
			super::push(out, 1229);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7584);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 7585);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25755);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 52957);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81359);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 25756);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 52958);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 81360);
											s.is_empty()
										}
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
					super::push(out, 7586);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25757);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7587);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25758);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 52960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7588);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25759);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 52961);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81361);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104301);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7589);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25760);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 52962);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 81362);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 104302);
													match s {
														[] => true,
														['й', s @ ..] => super::t13::n121342(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1230);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7590);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25761);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 52963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7591);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25762);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 52964);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 81363);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104303);
													s.is_empty()
												}
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
							super::push(out, 25763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7592);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 25764);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 52965);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 81364);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 104304);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n121343(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1231);
			s.is_empty()
		}
		['л', s @ ..] => super::t13::n1235(s, out),
		['м', s @ ..] => {
			super::push(out, 1233);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7610);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25803);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 53016);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81405);
											s.is_empty()
										}
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
					super::push(out, 7611);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25804);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 53017);
									match s {
										[] => true,
										['с', s @ ..] => super::t13::n82254(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t13::n7634(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t13::n1238(s, out),
		['р', s @ ..] => super::t13::n1239(s, out),
		['т', s @ ..] => super::t13::n1240(s, out),
		['ц', s @ ..] => super::t13::n1241(s, out),
		['ч', s @ ..] => super::t13::n1242(s, out),
		_ => false,
	}
}

/// Words ending -рг.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1240);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 7632);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25829);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 53034);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81417);
											s.is_empty()
										}
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
					super::push(out, 7633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1241);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7634);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25830);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 53035);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81418);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104336);
													s.is_empty()
												}
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
							super::push(out, 25831);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53036);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 81419);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 25832);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53037);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81420);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 104337);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 53038);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 81421);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 104338);
													match s {
														[] => true,
														['ю', 'в', s @ ..] => super::t13::n121367(s, out),
														_ => false,
													}
												}
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
							super::push(out, 25833);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53039);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 81422);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 104339);
													match s {
														[] => true,
														['а', s @ ..] => super::t13::n121368(s, out),
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
									super::push(out, 53040);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 81423);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 104340);
													s.is_empty()
												}
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
							super::push(out, 25834);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53041);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 81424);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 25835);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53042);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 81425);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 53043);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 81426);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 25836);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 53044);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81427);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 53045);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81428);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25837);
							match s {
								[] => true,
								['а', s @ ..] => super::t13::n53583(s, out),
								_ => false,
							}
						}
						['ь', s @ ..] => super::t13::n26009(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t13::n7657(s, out),
				['д', s @ ..] => super::t13::n7658(s, out),
				['п', s @ ..] => super::t13::n7659(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t13::n1245(s, out),
		['у', s @ ..] => super::t13::n1246(s, out),
		['э', s @ ..] => super::t13::n1247(s, out),
		_ => false,
	}
}

/// Words ending -сг.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1245);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7659);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уг.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1246);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1247);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7660);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1248);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7661);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7662);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25884);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53094);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 81470);
											s.is_empty()
										}
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
					super::push(out, 7663);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25885);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 53095);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81471);
											s.is_empty()
										}
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
					super::push(out, 7664);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 7665);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25886);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53096);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 81472);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1249);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7666);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25887);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53097);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81473);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104366);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7667);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25888);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 53098);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81474);
											s.is_empty()
										}
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
					super::push(out, 7668);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 7669);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25890);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 53100);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81475);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 25891);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53101);
									s.is_empty()
								}
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
			super::push(out, 1250);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25892);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 25893);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 53102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25894);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25895);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 25896);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53104);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81476);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 104367);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t13::n121380(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7672);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25897);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 53105);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 81477);
											s.is_empty()
										}
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
					super::push(out, 7673);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25898);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53106);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 81478);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 81479);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1251);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 7674);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25899);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53107);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81480);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7675);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 7676);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 25900);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53108);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 81481);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1253);
			match s {
				[] => true,
				['б', s @ ..] => super::t13::n7699(s, out),
				['н', s @ ..] => super::t13::n7700(s, out),
				['с', s @ ..] => super::t13::n7701(s, out),
				['ь', s @ ..] => super::t13::n7702(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t13::n1257(s, out),
		['ш', s @ ..] => super::t13::n1258(s, out),
		_ => false,
	}
}

/// Words ending -ыг.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1256);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7686);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 7687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1257);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 7688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1258);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 7689);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 25909);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53116);
									s.is_empty()
								}
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
			super::push(out, 1259);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 7690);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25910);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53117);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81486);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эг.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1260);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1261);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 1262);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1263);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юг.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1264);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 7691);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25911);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 53118);
									s.is_empty()
								}
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
			super::push(out, 1266);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7692);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25912);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53119);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 81487);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1267);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 7693);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 25913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1268);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7694);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7695);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7696);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 25916);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 7697);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яг.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1269);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7698);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25917);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53121);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 25918);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53122);
									s.is_empty()
								}
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
			super::push(out, 1270);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 7699);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 25919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1271);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7700);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 7701);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25921);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 53123);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81489);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 53124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 7702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7703);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7704);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25923);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 7705);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 7706);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 25924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53126);
									s.is_empty()
								}
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
			super::push(out, 1273);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 7707);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 25925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ад.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1274);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7708);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 25926);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53127);
									s.is_empty()
								}
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
			super::push(out, 1275);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 1276);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7709);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7710);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 7711);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25928);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53128);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 81490);
											s.is_empty()
										}
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
					super::push(out, 7712);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 25929);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53129);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 81491);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 104371);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1277);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7714);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1278);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7716);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1279);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7717);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7718);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 25933);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53130);
									s.is_empty()
								}
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
			super::push(out, 1280);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1281);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7720);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 7721);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25936);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53131);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 53132);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 81492);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1282);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7722);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 7723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 25937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7724);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 25938);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53133);
									match s {
										[] => true,
										['а', 'м', '\u{1}', s @ ..] => {
											super::push(out, 81493);
											s.is_empty()
										}
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
					super::push(out, 7725);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25939);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 25940);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25941);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 53134);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 81494);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 104372);
													match s {
														[] => true,
														['т', s @ ..] => super::t13::n121384(s, out),
														_ => false,
													}
												}
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
							super::push(out, 25942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53135);
									s.is_empty()
								}
								['-', s @ ..] => super::t13::n53673(s, out),
								['в', s @ ..] => {
									super::push(out, 53137);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81496);
											match s {
												[] => true,
												['з', s @ ..] => super::t13::n105476(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => super::t13::n53675(s, out),
								['о', s @ ..] => super::t13::n53676(s, out),
								['т', s @ ..] => super::t13::n53677(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t13::n7748(s, out),
				['с', s @ ..] => super::t13::n7749(s, out),
				['у', s @ ..] => super::t13::n7750(s, out),
				['х', s @ ..] => super::t13::n7751(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t13::n1286(s, out),
		['н', s @ ..] => super::t13::n1287(s, out),
		['п', s @ ..] => super::t13::n1288(s, out),
		['р', s @ ..] => super::t13::n1289(s, out),
		['с', s @ ..] => super::t13::n1290(s, out),
		['т', s @ ..] => super::t13::n1291(s, out),
		['у', s @ ..] => super::t13::n1292(s, out),
		['х', s @ ..] => super::t13::n1293(s, out),
		['ч', s @ ..] => super::t13::n1294(s, out),
		['ш', s @ ..] => super::t13::n1295(s, out),
		_ => false,
	}
}

/// Words ending -ед.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 7771);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26003);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 53186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7773);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7774);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26004);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 53187);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81528);
											s.is_empty()
										}
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
					super::push(out, 7775);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26005);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53188);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 81529);
											match s {
												[] => true,
												['н', 'а', '-', s @ ..] => {
													super::push(out, 104393);
													match s {
														[] => true,
														['к', s @ ..] => super::t13::n121399(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1298);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 7776);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26006);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7777);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26007);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 53190);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81530);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1299);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7778);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26008);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53191);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81531);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1300);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 7779);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 7780);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26009);
							match s {
								[] => true,
								['о', 'к', '\u{1}', s @ ..] => {
									super::push(out, 53192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 7781);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26010);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 26011);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 53193);
									match s {
										[] => true,
										['п', '-', s @ ..] => {
											super::push(out, 81532);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 104394);
													match s {
														[] => true,
														['у', s @ ..] => super::t13::n121400(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1301);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 7782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7784);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7785);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 26013);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53194);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 7786);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 26014);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53195);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81533);
											s.is_empty()
										}
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
					super::push(out, 7787);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26015);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53196);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81534);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 104395);
													s.is_empty()
												}
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
							super::push(out, 26016);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53197);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 81535);
											s.is_empty()
										}
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
					super::push(out, 7788);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26017);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53198);
									match s {
										[] => true,
										['о', s @ ..] => super::t13::n82384(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t13::n7811(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t13::n1307(s, out),
		['о', s @ ..] => super::t13::n1308(s, out),
		['п', s @ ..] => super::t13::n1309(s, out),
		['р', s @ ..] => super::t13::n1310(s, out),
		['с', s @ ..] => super::t13::n1311(s, out),
		['т', s @ ..] => super::t13::n1312(s, out),
		['ф', s @ ..] => super::t13::n1313(s, out),
		['х', s @ ..] => super::t13::n1314(s, out),
		['ш', s @ ..] => super::t13::n1315(s, out),
		['ь', s @ ..] => super::t13::n1316(s, out),
		_ => false,
	}
}

/// Words ending -ль-поезд.
pub(crate) fn n105517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104415);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 120145);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 129955);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 135735);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 139109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ид.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7845);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 7846);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 26096);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 53265);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 81577);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 104416);
													match s {
														[] => true,
														['п', s @ ..] => super::t13::n121411(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26097);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 53266);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81578);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26098);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 53267);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81579);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1318);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 7847);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26099);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 26100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7848);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26101);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 53268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 7849);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1319);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7850);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7851);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26103);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26104);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 53270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 7852);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26105);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53271);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7853);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26106);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 53272);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81580);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 26107);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 53273);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81581);
											s.is_empty()
										}
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
					super::push(out, 7854);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 7855);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26109);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 53274);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 81582);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 53275);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 81583);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 26110);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 7856);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 7857);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 7858);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26112);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53278);
									s.is_empty()
								}
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
			super::push(out, 1320);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7859);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7860);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26113);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 26114);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 7861);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26115);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 53281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 7862);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26116);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53282);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 81584);
											s.is_empty()
										}
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
					super::push(out, 7863);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26117);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53283);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81585);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t14::n1324(s, out),
		['е', s @ ..] => {
			super::push(out, 1322);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 7867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t14::n7890(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t14::n1326(s, out),
		['з', s @ ..] => super::t14::n1327(s, out),
		['й', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['е', s @ ..] => super::t14::n7899(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t14::n1329(s, out),
		['л', s @ ..] => super::t14::n1330(s, out),
		['м', s @ ..] => super::t14::n1331(s, out),
		['н', s @ ..] => super::t14::n1332(s, out),
		['о', s @ ..] => super::t14::n1333(s, out),
		['п', s @ ..] => super::t14::n1334(s, out),
		['р', s @ ..] => super::t14::n1335(s, out),
		['с', s @ ..] => super::t14::n1336(s, out),
		['т', s @ ..] => super::t14::n1337(s, out),
		['у', s @ ..] => super::t14::n1338(s, out),
		['ф', s @ ..] => super::t14::n1339(s, out),
		['х', s @ ..] => super::t14::n1340(s, out),
		['ц', s @ ..] => super::t14::n1341(s, out),
		['ш', s @ ..] => super::t14::n1342(s, out),
		['щ', s @ ..] => super::t14::n1343(s, out),
		['ю', s @ ..] => super::t14::n1344(s, out),
		_ => false,
	}
}

/// Words ending -айд.
pub(crate) fn n1345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1342);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 7951);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7952);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 7953);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26240);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 26241);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 53392);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 81666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ейд.
pub(crate) fn n1346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1343);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 7954);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 26242);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 53393);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 81667);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 104468);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 120168);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7955);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 7956);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 26243);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 53394);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 81668);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 104469);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120169);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лойд.
pub(crate) fn n7984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7958);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 26244);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 53395);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нойд.
pub(crate) fn n7985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7959);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эйд.
pub(crate) fn n1348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1345);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 7960);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 26246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лд.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1346);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7961);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 7962);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26247);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 53396);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 81669);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 26248);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26249);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 53398);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 81670);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104470);
													s.is_empty()
												}
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
							super::push(out, 26250);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53399);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81671);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 104471);
													match s {
														[] => true,
														['д', s @ ..] => super::t14::n121435(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26251);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 53400);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81672);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104472);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1348);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 7963);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 26252);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 53401);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81673);
											s.is_empty()
										}
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
					super::push(out, 7964);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 7965);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нд.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 7966);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 26253);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 53402);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81674);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 104473);
													s.is_empty()
												}
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
							super::push(out, 26254);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53403);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81675);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104474);
													s.is_empty()
												}
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
							super::push(out, 26255);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53404);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81676);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 104475);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n121436(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7967);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 7968);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26256);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 26257);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 53405);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 81677);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 104476);
													match s {
														[] => true,
														['е', s @ ..] => super::t14::n121437(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 7969);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 7970);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 7971);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26258);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53406);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 81678);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 26259);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53407);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 81679);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 104477);
													match s {
														[] => true,
														['о', s @ ..] => super::t14::n121438(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 81680);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 26260);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 53408);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81681);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 104478);
													match s {
														[] => true,
														['ф', s @ ..] => super::t14::n121439(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26261);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53409);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81682);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 104479);
													match s {
														[] => true,
														['е', s @ ..] => super::t14::n121440(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 81683);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 26262);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53410);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 81684);
											s.is_empty()
										}
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
					super::push(out, 7972);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26263);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53411);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t14::n7999(s, out),
				['р', s @ ..] => super::t14::n8000(s, out),
				['с', s @ ..] => {
					super::push(out, 7975);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26267);
							match s {
								[] => true,
								['е', s @ ..] => super::t14::n53957(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t14::n8002(s, out),
				['х', s @ ..] => super::t14::n8003(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t14::n1353(s, out),
		['и', s @ ..] => super::t14::n1354(s, out),
		['й', s @ ..] => super::t14::n1355(s, out),
		['о', s @ ..] => super::t14::n1356(s, out),
		['у', s @ ..] => super::t14::n1357(s, out),
		['э', s @ ..] => super::t14::n1358(s, out),
		_ => false,
	}
}

/// Words ending -од.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1356);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1357);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8007);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 8008);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 8009);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 8010);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26313);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t14::n8037(s, out),
				['з', s @ ..] => super::t14::n8038(s, out),
				['и', s @ ..] => super::t14::n8039(s, out),
				['о', s @ ..] => super::t14::n8040(s, out),
				['с', s @ ..] => {
					super::push(out, 8015);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t14::n8042(s, out),
				['у', s @ ..] => {
					super::push(out, 8017);
					s.is_empty()
				}
				['х', s @ ..] => super::t14::n8044(s, out),
				['ы', s @ ..] => super::t14::n8045(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t14::n1361(s, out),
		['д', s @ ..] => super::t14::n1362(s, out),
		['е', s @ ..] => super::t14::n1363(s, out),
		['з', s @ ..] => super::t14::n1364(s, out),
		['и', s @ ..] => super::t14::n1365(s, out),
		['й', s @ ..] => super::t14::n1366(s, out),
		['к', s @ ..] => super::t14::n1367(s, out),
		['л', s @ ..] => super::t14::n1368(s, out),
		['м', s @ ..] => super::t14::n1369(s, out),
		['н', s @ ..] => super::t14::n1370(s, out),
		['п', s @ ..] => super::t14::n1371(s, out),
		['р', s @ ..] => super::t14::n1372(s, out),
		['с', s @ ..] => super::t14::n1373(s, out),
		['т', s @ ..] => super::t14::n1374(s, out),
		['ф', s @ ..] => super::t14::n1375(s, out),
		['х', s @ ..] => super::t14::n1376(s, out),
		_ => false,
	}
}

/// Words ending -рд.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1374);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8079);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 26424);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 53589);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81822);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 26425);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53590);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26426);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 53591);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81824);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104565);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8080);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26427);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 53592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26428);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 53593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26429);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53594);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 81825);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 53595);
									match s {
										[] => true,
										['ф', s @ ..] => super::t14::n82674(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => super::t14::n8107(s, out),
				['и', s @ ..] => super::t14::n8108(s, out),
				['к', s @ ..] => super::t14::n8109(s, out),
				['л', s @ ..] => super::t14::n8110(s, out),
				['н', s @ ..] => super::t14::n8111(s, out),
				['п', s @ ..] => super::t14::n8112(s, out),
				['с', s @ ..] => {
					super::push(out, 8087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t14::n8114(s, out),
				['у', s @ ..] => super::t14::n8115(s, out),
				['ф', s @ ..] => super::t14::n8116(s, out),
				['х', s @ ..] => super::t14::n8117(s, out),
				['ч', s @ ..] => super::t14::n8118(s, out),
				['ш', s @ ..] => super::t14::n8119(s, out),
				['ю', s @ ..] => super::t14::n8120(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t14::n1378(s, out),
		['и', s @ ..] => super::t14::n1379(s, out),
		['о', s @ ..] => super::t14::n1380(s, out),
		['у', s @ ..] => super::t14::n1381(s, out),
		['я', s @ ..] => super::t14::n1382(s, out),
		_ => false,
	}
}

/// Words ending -уд.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1380);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8113);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26489);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 53652);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 8114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1381);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8115);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26491);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 53653);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81874);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1382);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8116);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 26492);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53654);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 26493);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 53655);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81876);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 104596);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8117);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26494);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53656);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 81877);
											s.is_empty()
										}
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
					super::push(out, 8118);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26495);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53657);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81878);
											s.is_empty()
										}
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
					super::push(out, 8119);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26496);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 53658);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81879);
											s.is_empty()
										}
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
					super::push(out, 8120);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 26497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53659);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8121);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26498);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 53660);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81880);
											s.is_empty()
										}
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
					super::push(out, 8122);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 26499);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53661);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 81881);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 104597);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n121501(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1383);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8123);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1384);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1385);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1386);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26502);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26503);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 53662);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 8126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1387);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8127);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26504);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 53663);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8128);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26505);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 53664);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81882);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1388);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1389);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8130);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 26506);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 53665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 26507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8131);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 8132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t14::n1394(s, out),
		['с', s @ ..] => super::t14::n1395(s, out),
		['т', s @ ..] => {
			super::push(out, 1393);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8148);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 1394);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 8150);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8151);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26527);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t14::n8178(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t14::n1398(s, out),
		_ => false,
	}
}

/// Words ending -ыд.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1396);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8156);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1397);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8157);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 8158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьд.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8159);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26534);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 53684);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81892);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 26535);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 53685);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 53686);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81893);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 26536);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53687);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26537);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8160);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26538);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53689);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81894);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 104600);
													s.is_empty()
												}
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
							super::push(out, 26539);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 53690);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 26540);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53691);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81895);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 104601);
													match s {
														[] => true,
														['б', s @ ..] => super::t14::n121503(s, out),
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
									super::push(out, 53692);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81896);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 104602);
													match s {
														[] => true,
														['о', s @ ..] => super::t14::n121504(s, out),
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
									super::push(out, 53693);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 81897);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104603);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 53694);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81898);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104604);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n121505(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26541);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53695);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81899);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 104605);
													match s {
														[] => true,
														['э', s @ ..] => super::t14::n121506(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26542);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 53696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8161);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26543);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 53697);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 81900);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104606);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t14::n8188(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эд.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1400);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8163);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 8164);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26552);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53710);
									s.is_empty()
								}
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
			super::push(out, 1401);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 8165);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26553);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1402);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8166);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юд.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8167);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 8168);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26554);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 53711);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81914);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26555);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8169);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26556);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8170);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26557);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 26558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1404);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8171);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 8172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яд.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1406);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1407);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8173);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26559);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 53714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 8174);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 26560);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 53715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 8175);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26561);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 53716);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81915);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1408);
			match s {
				[] => true,
				['г', 'о', s @ ..] => {
					super::push(out, 8176);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26562);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 26563);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53717);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81916);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 104613);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1409);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8177);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26564);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 53718);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81917);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1410);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8178);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 8179);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26565);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 26566);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 53719);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 81918);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 104614);
													match s {
														[] => true,
														['е', s @ ..] => super::t14::n121508(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8180);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8181);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8182);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26569);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 53720);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 81919);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104615);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n121509(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8183);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26570);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 53721);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81920);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 53722);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81921);
											s.is_empty()
										}
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
					super::push(out, 8184);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8185);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26572);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 53723);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 81922);
											s.is_empty()
										}
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
					super::push(out, 8186);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26573);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53724);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81923);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 104616);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -астильбе.
pub(crate) fn n105719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104617);
	s.is_empty()
}

/// Words ending -ерлингве.
pub(crate) fn n105720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104618);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 120245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кюве.
pub(crate) fn n8229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8203);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 26589);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ге.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8204);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 26590);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 53736);
									s.is_empty()
								}
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
			super::push(out, 1425);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 8205);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26591);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 53737);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 26592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8206);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26593);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53738);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 26594);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8207);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1427);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8208);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26596);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 53740);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81931);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104619);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1428);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8209);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26597);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -де.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8210);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26598);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 53741);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81932);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 104620);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1430);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8211);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26599);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53742);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 81933);
											s.is_empty()
										}
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
					super::push(out, 8212);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26600);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53743);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81934);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1431);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8213);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26601);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8214);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26602);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 53745);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 81935);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1432);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1433);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8216);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1434);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8217);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26604);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8218);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 26605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1435);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1436);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8220);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8221);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 26607);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 53747);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81936);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 104621);
													match s {
														[] => true,
														['а', s @ ..] => super::t14::n121511(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1437);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ее.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1438);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8223);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26609);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53748);
									s.is_empty()
								}
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
			super::push(out, 1439);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8224);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26610);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53749);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81937);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 104622);
													match s {
														[] => true,
														['и', s @ ..] => super::t14::n121512(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1440);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8225);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26611);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26612);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 53751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 8226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26613);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 53752);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 81938);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 104623);
													s.is_empty()
												}
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
							super::push(out, 26614);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 53753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 8227);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26615);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53754);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 81939);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1441);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8228);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 8229);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26617);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53755);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 81940);
											s.is_empty()
										}
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
					super::push(out, 8230);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26618);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53756);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 81941);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1442);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 8231);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26619);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 53757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 8232);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26620);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 53758);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 81942);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 53759);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 81943);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 104624);
													match s {
														[] => true,
														['р', s @ ..] => super::t14::n121513(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8233);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26621);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53760);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 81944);
											s.is_empty()
										}
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
					super::push(out, 8234);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 8235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => super::t14::n1447(s, out),
		_ => false,
	}
}

/// Words ending -же.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1444);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8241);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1445);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8242);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26634);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53774);
									s.is_empty()
								}
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
			super::push(out, 1446);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8243);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26635);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 26636);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8244);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26637);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 53777);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 81958);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1447);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8245);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 8246);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26638);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53778);
									s.is_empty()
								}
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
			super::push(out, 1448);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8247);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1449);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8248);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26640);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 53779);
									s.is_empty()
								}
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
			super::push(out, 1450);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8249);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26641);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8250);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 26642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 53781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зе.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1451);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8251);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26643);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53782);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 81959);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 104637);
													match s {
														[] => true,
														['к', '-', s @ ..] => super::t14::n121525(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8252);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8253);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26645);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 53783);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 81960);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 104638);
													match s {
														[] => true,
														['ж', s @ ..] => super::t14::n121526(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1452);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8255);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8256);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26648);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 53784);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 81961);
											s.is_empty()
										}
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
					super::push(out, 8257);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26649);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 53785);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81962);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 104639);
													match s {
														[] => true,
														['б', s @ ..] => super::t14::n121527(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1453);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8258);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26650);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 53786);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 81963);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 104640);
													match s {
														[] => true,
														['т', s @ ..] => super::t14::n121528(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1454);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1455);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8260);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26651);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 53787);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 81964);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 104641);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8261);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26652);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 53788);
									s.is_empty()
								}
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
			super::push(out, 1456);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 8262);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26653);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 53789);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 81965);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ие.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1457);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1458);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8263);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 8264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8265);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 26654);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 26655);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 53790);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 53791);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81966);
											match s {
												[] => true,
												['щ', s @ ..] => {
													super::push(out, 104642);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 81967);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 104643);
													match s {
														[] => true,
														['а', s @ ..] => super::t14::n121529(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8266);
					match s {
						[] => true,
						['л', 'с', 'о', 'л', s @ ..] => {
							super::push(out, 26656);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 53792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8267);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53793);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 81968);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 104644);
													match s {
														[] => true,
														['т', s @ ..] => super::t14::n121530(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 81969);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 53794);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 53795);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 81970);
											match s {
												[] => true,
												['р', 'т', 'с', '-', s @ ..] => {
													super::push(out, 104645);
													match s {
														[] => true,
														['н', s @ ..] => super::t14::n121531(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1460);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8268);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26657);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 53796);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 53797);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81971);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 104646);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 53798);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 81972);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 104647);
													match s {
														[] => true,
														['р', s @ ..] => super::t14::n121532(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1461);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8269);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 26658);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 53799);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 81973);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 104648);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t14::n121533(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8270);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8271);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 8272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t14::n1466(s, out),
		['з', s @ ..] => super::t14::n1467(s, out),
		['к', s @ ..] => super::t14::n1468(s, out),
		['л', s @ ..] => super::t14::n1469(s, out),
		['м', s @ ..] => super::t14::n1470(s, out),
		['н', s @ ..] => super::t14::n1471(s, out),
		['п', s @ ..] => {
			super::push(out, 1468);
			match s {
				[] => true,
				['е', s @ ..] => super::t14::n8325(s, out),
				['о', s @ ..] => super::t14::n8326(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t14::n1473(s, out),
		['с', s @ ..] => super::t14::n1474(s, out),
		['т', s @ ..] => super::t14::n1475(s, out),
		['ч', s @ ..] => super::t14::n1476(s, out),
		['ш', s @ ..] => super::t14::n1477(s, out),
		['щ', s @ ..] => super::t14::n1478(s, out),
		_ => false,
	}
}

/// Words ending -йе.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1475);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 8325);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26759);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 54077);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82395);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1476);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8326);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26760);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54078);
									s.is_empty()
								}
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
			super::push(out, 1477);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 8327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8328);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26761);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54079);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 82396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ке.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1479);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8329);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26762);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54080);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82397);
											s.is_empty()
										}
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
					super::push(out, 8330);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26763);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54081);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82398);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 105185);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1480);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8333);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8334);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 26765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 8335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1481);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8336);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26766);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 54082);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82399);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 105186);
													match s {
														[] => true,
														['о', s @ ..] => super::t14::n122046(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1482);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8337);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1483);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8338);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26768);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54083);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 82400);
											s.is_empty()
										}
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
					super::push(out, 8339);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 26769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 8340);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1484);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8341);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1485);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 1486);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 8343);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8344);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 26773);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 54084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ле.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1488);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 8345);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 26774);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54085);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 82401);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 105187);
													match s {
														[] => true,
														['г', s @ ..] => super::t14::n122047(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8346);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 8347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26775);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 26776);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54086);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 82402);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 105188);
													match s {
														[] => true,
														['а', s @ ..] => super::t14::n122048(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26777);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54087);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 82403);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 105189);
													match s {
														[] => true,
														['т', s @ ..] => super::t14::n122049(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8348);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 26778);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 54088);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 82404);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 105190);
													match s {
														[] => true,
														['у', s @ ..] => super::t14::n122050(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26779);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54089);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 82405);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 105191);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 105192);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ь', s @ ..] => {
											super::push(out, 82406);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 105193);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8349);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26780);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 54090);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82407);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 105194);
													match s {
														[] => true,
														['т', s @ ..] => super::t14::n122051(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8350);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26781);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54091);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82408);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 105195);
													match s {
														[] => true,
														['о', s @ ..] => super::t14::n122052(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1489);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1490);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8353);
					match s {
						[] => true,
						['а', s @ ..] => super::t14::n26970(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => super::t14::n8380(s, out),
				['л', s @ ..] => super::t14::n8381(s, out),
				['п', s @ ..] => super::t14::n8382(s, out),
				['р', s @ ..] => super::t14::n8383(s, out),
				['т', s @ ..] => super::t14::n8384(s, out),
				['ц', s @ ..] => super::t14::n8385(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t14::n1495(s, out),
		['и', s @ ..] => super::t14::n1496(s, out),
		['к', s @ ..] => super::t14::n1497(s, out),
		['л', s @ ..] => super::t14::n1498(s, out),
		['о', s @ ..] => super::t14::n1499(s, out),
		['с', s @ ..] => super::t14::n1500(s, out),
		['т', s @ ..] => super::t14::n1501(s, out),
		['у', s @ ..] => super::t14::n1502(s, out),
		['ф', s @ ..] => super::t14::n1503(s, out),
		_ => false,
	}
}

/// Words ending -ме.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8386);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8387);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26817);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 54120);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 82433);
											s.is_empty()
										}
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
					super::push(out, 8388);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26818);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54121);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82434);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1501);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 8389);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8390);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 26820);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 54122);
									s.is_empty()
								}
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
			super::push(out, 1502);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8391);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 26821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 54123);
									s.is_empty()
								}
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
			super::push(out, 1503);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1504);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8393);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1505);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8394);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26823);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 54124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 8395);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26824);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 54125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8396);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26825);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54126);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 82435);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1506);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8397);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 26826);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 54127);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 82436);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 105208);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8398);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 26827);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 54128);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 82437);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 105209);
													match s {
														[] => true,
														['р', s @ ..] => super::t14::n122062(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1507);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8399);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26828);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 54129);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 82438);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 105210);
													match s {
														[] => true,
														['с', s @ ..] => super::t14::n122063(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1508);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8400);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26829);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 54130);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 82439);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1509);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 8401);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26830);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 54131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -не.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1510);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8402);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26831);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 54132);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82440);
											s.is_empty()
										}
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
					super::push(out, 8403);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 26832);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 54133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8404);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26833);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 54134);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82441);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 105211);
													match s {
														[] => true,
														['а', s @ ..] => super::t14::n122064(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1511);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8405);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26834);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 54135);
									s.is_empty()
								}
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
			super::push(out, 1512);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8406);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26835);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54136);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 82442);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 105212);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8407);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 8408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1513);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8409);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26836);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54137);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 82443);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 26837);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 54138);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 82444);
											s.is_empty()
										}
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
					super::push(out, 8410);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 26838);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 54139);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 82445);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 105213);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8411);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26839);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54140);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 82446);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 105214);
													match s {
														[] => true,
														['к', s @ ..] => super::t14::n122065(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8412);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26840);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 54141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26841);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 54142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8413);
					match s {
						[] => true,
						['а', s @ ..] => super::t14::n27029(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t14::n8440(s, out),
				['у', s @ ..] => super::t14::n8441(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t14::n1518(s, out),
		['к', s @ ..] => super::t14::n1519(s, out),
		['н', s @ ..] => super::t14::n1520(s, out),
		['о', s @ ..] => super::t14::n1521(s, out),
		['р', s @ ..] => super::t14::n1522(s, out),
		['с', s @ ..] => super::t14::n1523(s, out),
		['ш', s @ ..] => super::t14::n1524(s, out),
		['э', s @ ..] => super::t14::n1525(s, out),
		_ => false,
	}
}

/// Words ending -ое.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1522);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8439);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26870);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 54166);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 82464);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 105230);
													match s {
														[] => true,
														['а', s @ ..] => super::t14::n122075(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26871);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54167);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 82465);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 105231);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8440);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 26872);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 54168);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 82466);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 26873);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 54169);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 82467);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 105232);
													match s {
														[] => true,
														['т', s @ ..] => super::t14::n122076(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26874);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 26875);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54170);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 82468);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 105233);
													match s {
														[] => true,
														['л', s @ ..] => super::t14::n122077(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26876);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54171);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 82469);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 105234);
													match s {
														[] => true,
														['г', s @ ..] => super::t14::n122078(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26877);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 54172);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 82470);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 105235);
													match s {
														[] => true,
														['т', s @ ..] => super::t14::n122079(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8441);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26878);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 54173);
									s.is_empty()
								}
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
			super::push(out, 1523);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 8442);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26879);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 54174);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 82471);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 105236);
													s.is_empty()
												}
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
							super::push(out, 26880);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 54175);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 82472);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 105237);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t14::n54735(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8443);
					match s {
						[] => true,
						['р', s @ ..] => super::t14::n27068(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t14::n1528(s, out),
		['к', s @ ..] => super::t14::n1529(s, out),
		['л', s @ ..] => super::t14::n1530(s, out),
		['м', s @ ..] => super::t14::n1531(s, out),
		['н', s @ ..] => super::t14::n1532(s, out),
		['п', s @ ..] => super::t14::n1533(s, out),
		['р', s @ ..] => super::t14::n1534(s, out),
		['т', s @ ..] => super::t14::n1535(s, out),
		['х', s @ ..] => super::t14::n1536(s, out),
		['ш', s @ ..] => super::t14::n1537(s, out),
		_ => false,
	}
}

/// Words ending -пе.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1534);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8489);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 8490);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26970);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54295);
									s.is_empty()
								}
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
			super::push(out, 1535);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8491);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26971);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 54296);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82580);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 105313);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1536);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8492);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26972);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 54297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 26973);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 54298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 8493);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26974);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54299);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82581);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 105314);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1537);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8494);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26975);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54300);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 8495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1538);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8496);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26977);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 54301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 8497);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26978);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 54302);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 82582);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 105315);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1539);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8498);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 26979);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 54303);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82583);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 105316);
													match s {
														[] => true,
														['ш', s @ ..] => super::t14::n122125(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ре.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1540);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8499);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 26980);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 54304);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82584);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 105317);
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 82585);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 105318);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8500);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26981);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 8501);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26982);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 54306);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 82586);
											s.is_empty()
										}
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
					super::push(out, 8502);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 8503);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8504);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26984);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 54307);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26985);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 8505);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26986);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 26987);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 26988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 8506);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 26989);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 54309);
									s.is_empty()
								}
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
			super::push(out, 1541);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 8507);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 26990);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 26991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1542);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8508);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 26992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1543);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 8509);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 8510);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26993);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54310);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 82587);
											s.is_empty()
										}
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
					super::push(out, 8511);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26994);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 54311);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 82588);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 105319);
													match s {
														[] => true,
														['л', s @ ..] => super::t14::n122126(s, out),
														_ => false,
													}
												}
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
							super::push(out, 26995);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 54312);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 82589);
											s.is_empty()
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 54313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 26996);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 8512);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26997);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 54315);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 8513);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26998);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 54316);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 82590);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 105320);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1544);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8514);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 26999);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 1545);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 8516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1546);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 8517);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 27000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1547);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 8518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1548);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 8519);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 8520);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 8521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t14::n1553(s, out),
		['т', s @ ..] => super::t14::n1554(s, out),
		['у', s @ ..] => super::t14::n1555(s, out),
		['ф', s @ ..] => super::t14::n1556(s, out),
		['э', s @ ..] => super::t14::n1557(s, out),
		['ю', s @ ..] => super::t14::n1558(s, out),
		_ => false,
	}
}

/// Words ending -се.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1555);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 8531);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 27009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 8532);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 27010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1556);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1557);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 8534);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 27011);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54321);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 82593);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 27012);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54322);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 82594);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1558);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8535);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 27013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 8536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1559);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8537);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 27014);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 54323);
									s.is_empty()
								}
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
			super::push(out, 1560);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 8538);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 27015);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 54324);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 82595);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 27016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 8539);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 8540);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 27018);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 54325);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 82596);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 105323);
													match s {
														[] => true,
														['а', s @ ..] => super::t14::n122128(s, out),
														_ => false,
													}
												}
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
							super::push(out, 27019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 8541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 27020);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 27021);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 54326);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 82597);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 105324);
													match s {
														[] => true,
														['ч', s @ ..] => super::t14::n122129(s, out),
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
									super::push(out, 54327);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 82598);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 105325);
													match s {
														[] => true,
														['о', s @ ..] => super::t14::n122130(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 8542);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 27022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 1561);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 8543);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 27023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

