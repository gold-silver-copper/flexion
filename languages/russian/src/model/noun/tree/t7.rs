use flexion::Path;

/// Words ending -ьс.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3656);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16187);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 39077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 39078);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 67116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39079);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 67117);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92963);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 39080);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 67118);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92964);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 112499);
													match s {
														[] => true,
														['п', s @ ..] => super::t24::n126466(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16188);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39081);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 39082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 67119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39083);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 67120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 16189);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 39084);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 67121);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 92965);
											s.is_empty()
										}
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
					super::push(out, 16190);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 39085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эс.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3657);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3658);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3659);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 16192);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39086);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 67122);
									s.is_empty()
								}
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
			super::push(out, 3660);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16193);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 39087);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 67123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 16194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3661);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 3662);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16196);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 39088);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 67124);
									s.is_empty()
								}
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
			super::push(out, 3663);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16197);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39089);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 67125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юс.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3664);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16198);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16199);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39090);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 67126);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92966);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3665);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16200);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 39091);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 67127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16201);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 39092);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67128);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92967);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 112500);
													match s {
														[] => true,
														['о', s @ ..] => super::t24::n126467(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16202);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39093);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16203);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 39094);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 39095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16204);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 16205);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39096);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 67130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 39097);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67131);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92968);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 112501);
													s.is_empty()
												}
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
							super::push(out, 39098);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 67132);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 92969);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 112502);
													match s {
														[] => true,
														['с', s @ ..] => super::t24::n126468(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16206);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39099);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 67133);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92970);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 112503);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16207);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 39100);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 67134);
									s.is_empty()
								}
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
			super::push(out, 3666);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16208);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16209);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 16210);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39102);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 67135);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 92971);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3667);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 16211);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 39103);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67136);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 92972);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 112504);
													match s {
														[] => true,
														['ф', s @ ..] => super::t24::n126469(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яс.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 380);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3668);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 3669);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3670);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16213);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39104);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67137);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92973);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 112505);
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
										['и', s @ ..] => {
											super::push(out, 92974);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 112506);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 92975);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 112507);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3671);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 16214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39105);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39106);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 67138);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 92976);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 112508);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3672);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16215);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16216);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 67139);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92977);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 112509);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 92978);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 112510);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 67140);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92979);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 112511);
													match s {
														[] => true,
														['-', s @ ..] => super::t24::n126470(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 92980);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 112512);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 67141);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 67142);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 67143);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 92981);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3673);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16217);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39108);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 67144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ат.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3675);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16219);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 39109);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 67145);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92982);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112513);
													s.is_empty()
												}
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
							super::push(out, 39110);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 39111);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 67146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 16220);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39112);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 67147);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 92983);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 112514);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 67148);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92984);
											s.is_empty()
										}
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
					super::push(out, 16221);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39113);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 67149);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92985);
											s.is_empty()
										}
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
					super::push(out, 16222);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 39114);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 67150);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 92986);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112515);
													s.is_empty()
												}
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
							super::push(out, 39115);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 67151);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 92987);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112516);
													s.is_empty()
												}
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
							super::push(out, 39116);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 67152);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 92988);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 39117);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 67153);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 92989);
											s.is_empty()
										}
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
					super::push(out, 16223);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39118);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 67154);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92990);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 67155);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92991);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 39119);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 67156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16224);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 39120);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 67157);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 92992);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 67158);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 39121);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 67159);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 92993);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 112517);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16225);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39122);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 67160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 16226);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39123);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 67161);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 67162);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 92994);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t24::n16316(s, out),
				['с', s @ ..] => super::t24::n16317(s, out),
				['у', s @ ..] => {
					super::push(out, 16229);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 16230);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 16231);
					s.is_empty()
				}
				['я', s @ ..] => super::t24::n16321(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t24::n3687(s, out),
		['г', s @ ..] => super::t24::n3688(s, out),
		['д', s @ ..] => super::t24::n3689(s, out),
		['е', s @ ..] => super::t24::n3690(s, out),
		['ж', s @ ..] => super::t24::n3691(s, out),
		['з', s @ ..] => super::t24::n3692(s, out),
		['и', s @ ..] => super::t24::n3693(s, out),
		['к', s @ ..] => super::t24::n3694(s, out),
		['л', s @ ..] => super::t24::n3695(s, out),
		['м', s @ ..] => super::t24::n3696(s, out),
		['н', s @ ..] => super::t24::n3697(s, out),
		['о', s @ ..] => super::t24::n3698(s, out),
		['п', s @ ..] => super::t24::n3699(s, out),
		['р', s @ ..] => super::t25::n3700(s, out),
		['с', s @ ..] => super::t25::n3701(s, out),
		['т', s @ ..] => super::t25::n3702(s, out),
		['у', s @ ..] => super::t25::n3703(s, out),
		['ф', s @ ..] => super::t25::n3704(s, out),
		['х', s @ ..] => super::t25::n3705(s, out),
		['ч', s @ ..] => super::t25::n3706(s, out),
		['ш', s @ ..] => super::t25::n3707(s, out),
		_ => false,
	}
}

/// Words ending -а-космонавт.
pub(crate) fn n113841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112636);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 125223);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 132830);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 137325);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140009);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141598);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -к-космонавт.
pub(crate) fn n113842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112637);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 125224);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 132831);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 137326);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140010);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 141599);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142523);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -переливт.
pub(crate) fn n113843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112638);
	s.is_empty()
}

/// Words ending -айбштадт.
pub(crate) fn n113844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112639);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 125225);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенштадт.
pub(crate) fn n113845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112640);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 125226);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 132832);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 137327);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онштадт.
pub(crate) fn n94213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93214);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 112641);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 125227);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рштадт.
pub(crate) fn n68183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67485);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 93215);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 112642);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндт.
pub(crate) fn n3714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3703);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 16391);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39422);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 67486);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 93216);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112643);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125228);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ет.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3704);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 16392);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 39423);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 67487);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 93217);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 112644);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n126527(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39424);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16393);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39425);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 39426);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 67489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 39427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 16394);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39428);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67490);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 93218);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 112645);
													match s {
														[] => true,
														['к', s @ ..] => super::t25::n126528(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39429);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 39430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 16395);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16396);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39431);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 67491);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93219);
											s.is_empty()
										}
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
					super::push(out, 16397);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 16398);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39432);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67492);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93220);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 67493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16399);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 39433);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 39434);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 67494);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 93221);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112646);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3705);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16400);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16401);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16402);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39436);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 39437);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 39438);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 39439);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 39440);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67495);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 93222);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 112647);
													match s {
														[] => true,
														['р', s @ ..] => super::t25::n126529(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39441);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 67496);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 93223);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t25::n39819(s, out),
						['т', s @ ..] => super::t25::n39820(s, out),
						['у', s @ ..] => super::t25::n39821(s, out),
						['ы', s @ ..] => super::t25::n39822(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 16404);
					match s {
						[] => true,
						['о', s @ ..] => super::t25::n39823(s, out),
						_ => false,
					}
				}
				['х', s @ ..] => super::t25::n16500(s, out),
				['ц', s @ ..] => super::t25::n16501(s, out),
				['ю', s @ ..] => super::t25::n16502(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t25::n3717(s, out),
		['д', s @ ..] => super::t25::n3718(s, out),
		['ж', s @ ..] => super::t25::n3719(s, out),
		['з', s @ ..] => super::t25::n3720(s, out),
		['и', s @ ..] => super::t25::n3721(s, out),
		['к', s @ ..] => super::t25::n3722(s, out),
		['л', s @ ..] => super::t25::n3723(s, out),
		['м', s @ ..] => super::t25::n3724(s, out),
		['н', s @ ..] => super::t25::n3725(s, out),
		['п', s @ ..] => super::t25::n3726(s, out),
		['р', s @ ..] => super::t25::n3727(s, out),
		['с', s @ ..] => super::t25::n3728(s, out),
		['т', s @ ..] => super::t25::n3729(s, out),
		['ф', s @ ..] => super::t25::n3730(s, out),
		['х', s @ ..] => super::t25::n3731(s, out),
		['ц', s @ ..] => super::t25::n3732(s, out),
		['ч', s @ ..] => super::t25::n3733(s, out),
		['ш', s @ ..] => super::t25::n3734(s, out),
		['ь', s @ ..] => super::t25::n3735(s, out),
		_ => false,
	}
}

/// Words ending -ит.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 388);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3725);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16538);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39671);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 67696);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93358);
											s.is_empty()
										}
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
					super::push(out, 16539);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39672);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 67697);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93359);
											s.is_empty()
										}
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
					super::push(out, 16540);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 39673);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 67698);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 93360);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 112720);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3726);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 16541);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 16542);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 39674);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 39675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 67699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39676);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 39677);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 67700);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 93361);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 112721);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n126564(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16543);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39678);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 67701);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 39679);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 67702);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93362);
											s.is_empty()
										}
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
					super::push(out, 16544);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 39680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16545);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 39681);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67703);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93363);
											s.is_empty()
										}
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
					super::push(out, 16546);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16547);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39682);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 67704);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93364);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', 'и', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67705);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 67706);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 93365);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 39683);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 67707);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 93366);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39684);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 39685);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 67708);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 93367);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 112722);
													match s {
														[] => true,
														['ш', s @ ..] => super::t25::n126565(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16548);
					s.is_empty()
				}
				['ь', 'л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39686);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67709);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93368);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 39687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t25::n3738(s, out),
		['г', s @ ..] => super::t25::n3739(s, out),
		['д', s @ ..] => super::t25::n3740(s, out),
		['е', s @ ..] => {
			super::push(out, 3730);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16569);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39724);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 67750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16570);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => super::t25::n16668(s, out),
				['к', s @ ..] => super::t25::n16669(s, out),
				['л', s @ ..] => super::t25::n16670(s, out),
				['п', s @ ..] => super::t25::n16671(s, out),
				['т', s @ ..] => super::t25::n16672(s, out),
				['х', s @ ..] => super::t25::n16673(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t25::n3742(s, out),
		['з', s @ ..] => super::t25::n3743(s, out),
		['и', s @ ..] => super::t25::n3744(s, out),
		['й', s @ ..] => super::t25::n3745(s, out),
		['к', s @ ..] => super::t25::n3746(s, out),
		['л', s @ ..] => super::t25::n3747(s, out),
		['м', s @ ..] => super::t25::n3748(s, out),
		['н', s @ ..] => super::t25::n3749(s, out),
		['о', s @ ..] => super::t25::n3750(s, out),
		['п', s @ ..] => super::t25::n3751(s, out),
		['р', s @ ..] => super::t25::n3752(s, out),
		['с', s @ ..] => super::t25::n3753(s, out),
		['т', s @ ..] => super::t25::n3754(s, out),
		['у', s @ ..] => super::t25::n3755(s, out),
		['ф', s @ ..] => super::t25::n3756(s, out),
		['х', s @ ..] => super::t25::n3757(s, out),
		['ц', s @ ..] => super::t25::n3758(s, out),
		['ч', s @ ..] => super::t25::n3759(s, out),
		['ш', s @ ..] => super::t25::n3760(s, out),
		['щ', s @ ..] => super::t25::n3761(s, out),
		_ => false,
	}
}

/// Words ending -йт.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3751);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16698);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 67939);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 67940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 39937);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 39938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 16699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16700);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16701);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39940);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 67941);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93538);
											s.is_empty()
										}
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
					super::push(out, 16702);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 39941);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 67942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 39942);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 67943);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 93539);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112828);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16703);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 16704);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39943);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 67944);
									s.is_empty()
								}
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
			super::push(out, 3752);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16705);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 16706);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39944);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 67945);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 93540);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112829);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16707);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 39945);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 39946);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 67946);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 93541);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 112830);
													match s {
														[] => true,
														['у', s @ ..] => super::t25::n126620(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39947);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 67947);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 93542);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 112831);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n126621(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16708);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16709);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 39948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16710);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39949);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 67948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 39950);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 67949);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 93543);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 112832);
													s.is_empty()
												}
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
							super::push(out, 39951);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 67950);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 93544);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112833);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16711);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 39952);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 67951);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93545);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3753);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16712);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39953);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 67952);
									match s {
										[] => true,
										['т', s @ ..] => super::t25::n94547(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t25::n3765(s, out),
		_ => false,
	}
}

/// Words ending -кт.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3755);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16714);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39955);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67953);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 93547);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 112835);
													match s {
														[] => true,
														['в', s @ ..] => super::t25::n126623(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39956);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 67954);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 93548);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112836);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16715);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 39957);
							match s {
								[] => true,
								['о', 'к', '-', s @ ..] => {
									super::push(out, 67955);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 93549);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 112837);
													match s {
														[] => true,
														['т', s @ ..] => super::t25::n126624(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16716);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16717);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 16718);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39958);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67956);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 93550);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112838);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3756);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 16719);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16720);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 16721);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39959);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67957);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 67958);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 93551);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 112839);
													s.is_empty()
												}
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
							super::push(out, 39960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3757);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16722);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 39961);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 67959);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93552);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 39962);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 67960);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 93553);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 39963);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 67961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16723);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 39964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16725);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 39966);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 67962);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 93554);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 112840);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n126625(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3758);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 16726);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 16727);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 39967);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67963);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 93555);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 112841);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лт.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3760);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16728);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 39968);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 67964);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 93556);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 112842);
													match s {
														[] => true,
														['о', s @ ..] => super::t25::n126626(s, out),
														_ => false,
													}
												}
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
							super::push(out, 39969);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 67965);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 93557);
											s.is_empty()
										}
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
					super::push(out, 16729);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 39970);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 16730);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 39971);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67966);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93558);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3761);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 16731);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 39972);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 67967);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 93559);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3762);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16732);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3763);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39973);
							s.is_empty()
						}
						['-', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 67968);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 93560);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 112843);
													match s {
														[] => true,
														['ш', s @ ..] => super::t25::n126627(s, out),
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
									super::push(out, 67969);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 93561);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 112844);
													s.is_empty()
												}
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
							super::push(out, 39974);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 67970);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 93562);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 112845);
													match s {
														[] => true,
														['п', s @ ..] => super::t25::n126628(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16735);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 16736);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 39975);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 67971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мт.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3764);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 16737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3765);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нт.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['а', s @ ..] => super::t25::n3777(s, out),
		['е', s @ ..] => super::t25::n3778(s, out),
		['и', s @ ..] => {
			super::push(out, 3768);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 16773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 16774);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16775);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68221);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 40139);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 40140);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 40141);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 68223);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93821);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 40142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16776);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 40143);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 68224);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 93822);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 113043);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16777);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40144);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40145);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 68225);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 40146);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68226);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16778);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 16779);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 16780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 16781);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40148);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68227);
									s.is_empty()
								}
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
			super::push(out, 3769);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 16782);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3770);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 16783);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16784);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40150);
							match s {
								[] => true,
								['и', 'р', 'е', 'п', s @ ..] => {
									super::push(out, 68228);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93824);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 68229);
									s.is_empty()
								}
								['р', s @ ..] => super::t25::n68946(s, out),
								['т', s @ ..] => {
									super::push(out, 68231);
									s.is_empty()
								}
								['ф', s @ ..] => super::t25::n68948(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t25::n16884(s, out),
				['и', s @ ..] => super::t25::n16885(s, out),
				['к', s @ ..] => super::t25::n16886(s, out),
				['м', s @ ..] => super::t25::n16887(s, out),
				['п', s @ ..] => super::t25::n16888(s, out),
				['р', s @ ..] => super::t25::n16889(s, out),
				['ф', s @ ..] => super::t25::n16890(s, out),
				['х', s @ ..] => super::t25::n16891(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t25::n3782(s, out),
		['я', s @ ..] => super::t25::n3783(s, out),
		_ => false,
	}
}

/// Words ending -от.
pub(crate) fn n395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3773);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 16802);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40180);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 68267);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 40181);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 16803);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40182);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 40183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 16804);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40184);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68270);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 93854);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 113062);
													s.is_empty()
												}
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
							super::push(out, 40185);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68271);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 93855);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 40186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16805);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40187);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 40188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16806);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40189);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 68273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68274);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 68275);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 68276);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93856);
											s.is_empty()
										}
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
					super::push(out, 16807);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40190);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68277);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 93857);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 113063);
													s.is_empty()
												}
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
							super::push(out, 40191);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 68278);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 93858);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113064);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 68279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 40192);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68280);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 93859);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 40193);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68281);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 93860);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 113065);
													match s {
														[] => true,
														['п', s @ ..] => super::t25::n126758(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16808);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40195);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 68283);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 93861);
											s.is_empty()
										}
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
					super::push(out, 16809);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40196);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68284);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 40197);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68285);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 93862);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 113066);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16810);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40198);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68286);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 40199);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 68287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => super::t25::n16910(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t25::n3785(s, out),
		['г', s @ ..] => super::t25::n3786(s, out),
		['д', s @ ..] => super::t25::n3787(s, out),
		['е', s @ ..] => super::t25::n3788(s, out),
		['з', s @ ..] => super::t25::n3789(s, out),
		['и', s @ ..] => super::t25::n3790(s, out),
		['й', s @ ..] => super::t25::n3791(s, out),
		['к', s @ ..] => super::t25::n3792(s, out),
		['л', s @ ..] => super::t25::n3793(s, out),
		['м', s @ ..] => super::t25::n3794(s, out),
		['н', s @ ..] => super::t25::n3795(s, out),
		['п', s @ ..] => super::t25::n3796(s, out),
		['р', s @ ..] => super::t25::n3797(s, out),
		['с', s @ ..] => super::t25::n3798(s, out),
		['т', s @ ..] => super::t25::n3799(s, out),
		['ф', s @ ..] => {
			super::push(out, 3789);
			match s {
				[] => true,
				['а', s @ ..] => super::t25::n16997(s, out),
				['о', s @ ..] => super::t25::n16998(s, out),
				_ => false,
			}
		}
		['х', s @ ..] => super::t25::n3801(s, out),
		['ш', s @ ..] => super::t25::n3802(s, out),
		_ => false,
	}
}

/// Words ending -пт.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3792);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 3793);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 16908);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40323);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 16909);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 16910);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 16911);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40324);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 68398);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93930);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3794);
			match s {
				[] => true,
				['р', 'к', 'с', 'о', s @ ..] => {
					super::push(out, 16912);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40325);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 68399);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93931);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'к', s @ ..] => {
			super::push(out, 3795);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3796);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 16914);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рт.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3797);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 16915);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40327);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 40328);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 40329);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68400);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 93932);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 113112);
													match s {
														[] => true,
														['а', s @ ..] => super::t25::n126780(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40330);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 68401);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 93933);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 113113);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n126781(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40331);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68402);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 93934);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113114);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40332);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40333);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 68403);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93935);
											s.is_empty()
										}
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
					super::push(out, 16917);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40334);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68404);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 93936);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 113115);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t25::n126782(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 16918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40335);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 40336);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 16919);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 16920);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 16921);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40337);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 68406);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93937);
											s.is_empty()
										}
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
					super::push(out, 16922);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 16923);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 16924);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 16925);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 16926);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40340);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68407);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93938);
											s.is_empty()
										}
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
					super::push(out, 16927);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40341);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68408);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93939);
											s.is_empty()
										}
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
					super::push(out, 16928);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40342);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68409);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 93940);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t25::n3809(s, out),
		['и', s @ ..] => {
			super::push(out, 3799);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 16940);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 16941);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 40366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 16942);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 16943);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68427);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 68428);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 93961);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 93962);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 93963);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t26::n3811(s, out),
		['у', s @ ..] => super::t26::n3812(s, out),
		['ы', s @ ..] => {
			super::push(out, 3802);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 16961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t26::n3814(s, out),
		_ => false,
	}
}

/// Words ending -ст.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3804);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 16965);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 16966);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40414);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68474);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94004);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 113161);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 16967);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40415);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 68475);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 94005);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113162);
													match s {
														[] => true,
														['к', s @ ..] => super::t26::n126810(s, out),
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
									super::push(out, 68476);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94006);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 113163);
													match s {
														[] => true,
														['э', '-', s @ ..] => super::t26::n126811(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40416);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68477);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 68478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 16968);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 16969);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40418);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 40419);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 68480);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 94007);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 40420);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68481);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 68482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 16970);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 40421);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68483);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 94008);
											s.is_empty()
										}
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
					super::push(out, 16971);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 16972);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40423);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68484);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 68485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 40424);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 68486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 40425);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68487);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 94009);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113164);
													s.is_empty()
												}
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
							super::push(out, 40426);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 68488);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 94010);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 40427);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68489);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 94011);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113165);
													s.is_empty()
												}
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
							super::push(out, 40428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t26::n17072(s, out),
				['х', s @ ..] => super::t26::n17073(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t26::n3816(s, out),
		['е', s @ ..] => super::t26::n3817(s, out),
		['и', s @ ..] => super::t26::n3818(s, out),
		['й', s @ ..] => super::t26::n3819(s, out),
		['к', s @ ..] => super::t26::n3820(s, out),
		['л', s @ ..] => {
			super::push(out, 3810);
			match s {
				[] => true,
				['о', s @ ..] => super::t26::n17115(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t26::n3822(s, out),
		['о', s @ ..] => super::t26::n3823(s, out),
		['р', s @ ..] => super::t26::n3824(s, out),
		['у', s @ ..] => super::t26::n3825(s, out),
		['ы', s @ ..] => super::t26::n3826(s, out),
		['э', s @ ..] => super::t26::n3827(s, out),
		['ю', s @ ..] => super::t26::n3828(s, out),
		['я', s @ ..] => super::t26::n3829(s, out),
		_ => false,
	}
}

/// Words ending -тт.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3819);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40583);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 40584);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3820);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17055);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40586);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68659);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94138);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113255);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17056);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17057);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40588);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 68660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 40589);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68661);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94139);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 113256);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3821);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17060);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17061);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3822);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17062);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 17063);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ут.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3823);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17064);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17065);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 17066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40594);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 40595);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68662);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 94140);
											s.is_empty()
										}
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
					super::push(out, 17067);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40596);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 68663);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94141);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 40597);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 68664);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 68665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 40598);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 40599);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68666);
									s.is_empty()
								}
								['й', s @ ..] => {
									super::push(out, 68667);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 94142);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 113257);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 68668);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94143);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 113258);
													match s {
														[] => true,
														['г', s @ ..] => super::t26::n126861(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40600);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 68669);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94144);
											s.is_empty()
										}
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
					super::push(out, 17068);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17069);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40601);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68670);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94145);
											s.is_empty()
										}
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
					super::push(out, 17070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17071);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 3824);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17073);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40603);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68671);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 94146);
											s.is_empty()
										}
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
					super::push(out, 17074);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40604);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68672);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 40605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17075);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40606);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68673);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94147);
											s.is_empty()
										}
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
					super::push(out, 17076);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40607);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68674);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 94148);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3825);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17077);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40608);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68675);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 94149);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3826);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 17078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17079);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 40610);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68676);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 94150);
											s.is_empty()
										}
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
					super::push(out, 17080);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40611);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68677);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 40612);
							match s {
								[] => true,
								['е', s @ ..] => super::t26::n69401(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t26::n17180(s, out),
				['т', s @ ..] => super::t26::n17181(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t26::n3838(s, out),
		['е', s @ ..] => super::t26::n3839(s, out),
		['ж', s @ ..] => super::t26::n3840(s, out),
		['з', s @ ..] => super::t26::n3841(s, out),
		['и', s @ ..] => super::t26::n3842(s, out),
		['к', s @ ..] => super::t26::n3843(s, out),
		['л', s @ ..] => super::t26::n3844(s, out),
		['м', s @ ..] => super::t26::n3845(s, out),
		['н', s @ ..] => super::t26::n3846(s, out),
		['о', s @ ..] => super::t26::n3847(s, out),
		['п', s @ ..] => super::t26::n3848(s, out),
		['р', s @ ..] => super::t26::n3849(s, out),
		['с', s @ ..] => super::t26::n3850(s, out),
		['т', s @ ..] => super::t26::n3851(s, out),
		['ф', s @ ..] => super::t26::n3852(s, out),
		['ш', s @ ..] => super::t26::n3853(s, out),
		['я', s @ ..] => super::t26::n3854(s, out),
		_ => false,
	}
}

/// Words ending -фт.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3844);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17138);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40675);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 40676);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68730);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 94182);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113270);
													match s {
														[] => true,
														['м', s @ ..] => super::t26::n126867(s, out),
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
									super::push(out, 68731);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94183);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 113271);
													match s {
														[] => true,
														['о', s @ ..] => super::t26::n126868(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 17139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3845);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17140);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68732);
									s.is_empty()
								}
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
			super::push(out, 3846);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17141);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17142);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 40678);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17143);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 40679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68734);
									s.is_empty()
								}
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
			super::push(out, 3847);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40680);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 40681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хт.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3848);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17145);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40682);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 68735);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 94184);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 113272);
													match s {
														[] => true,
														['л', s @ ..] => super::t26::n126869(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 17146);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40683);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 68736);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94185);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 113273);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n126870(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 17147);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 40684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3849);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17148);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40685);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17149);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40686);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 68738);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 94186);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113274);
													s.is_empty()
												}
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
							super::push(out, 40687);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68739);
									s.is_empty()
								}
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
			super::push(out, 3850);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17150);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40688);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 68740);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 94187);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 40689);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68741);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94188);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113275);
													match s {
														[] => true,
														['м', s @ ..] => super::t26::n126871(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3851);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17151);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40690);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 68742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3852);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 17153);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чт.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 3853);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17154);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шт.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3854);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17155);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40694);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 68743);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 94189);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3855);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3856);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17157);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17158);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40695);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68744);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94190);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыт.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17159);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17160);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40697);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17161);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 40699);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 68746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', 'н', 'а', 'ч', '\u{1}', s @ ..] => {
			super::push(out, 3858);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 3859);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17162);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40700);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68747);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94191);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3860);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40701);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 40702);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 68748);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 94192);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 113276);
													s.is_empty()
												}
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
							super::push(out, 40703);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68749);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 94193);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113277);
													s.is_empty()
												}
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
							super::push(out, 40704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьт.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 405);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3861);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40705);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 40706);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68751);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94194);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 40707);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 40708);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 68753);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 94195);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113278);
													s.is_empty()
												}
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
							super::push(out, 40709);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17165);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40710);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 68755);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 94196);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 113279);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n126872(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17166);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17167);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68757);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 68758);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 68759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17168);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40713);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 40714);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 40715);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 40716);
							match s {
								[] => true,
								['н', 'о', s @ ..] => {
									super::push(out, 68760);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94197);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 113280);
													match s {
														[] => true,
														['и', s @ ..] => super::t26::n126873(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эт.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 406);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 3862);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 3863);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 3864);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 17169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3865);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3866);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17171);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 40718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ют.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3868);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 3869);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17172);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40719);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 68761);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 94198);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 113281);
													match s {
														[] => true,
														['л', s @ ..] => super::t26::n126874(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3870);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3871);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17174);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 17175);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40721);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68762);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94199);
											s.is_empty()
										}
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
					super::push(out, 17176);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40722);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 40723);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 68764);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94200);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3872);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17177);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 40724);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68765);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94201);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113282);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3873);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 3874);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17178);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 17179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 3875);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17180);
					match s {
						[] => true,
						['р', 'у', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 40727);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 40728);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 68766);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94202);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113283);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ят.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3876);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 3877);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17182);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17183);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40730);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68768);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94203);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 113284);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 68769);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 68770);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 94204);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 113285);
													match s {
														[] => true,
														['п', s @ ..] => super::t26::n126875(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3878);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17184);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3879);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17185);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 17186);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'л', s @ ..] => {
					super::push(out, 17187);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40733);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68771);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94205);
											s.is_empty()
										}
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
					super::push(out, 17188);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17189);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 17190);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17191);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 40734);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68772);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 94206);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3880);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17192);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3881);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17193);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17194);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17195);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40737);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68773);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 94207);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113286);
													match s {
														[] => true,
														['ы', s @ ..] => super::t26::n126876(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40738);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 68774);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94208);
											s.is_empty()
										}
										['ф', s @ ..] => {
											super::push(out, 94209);
											s.is_empty()
										}
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
					super::push(out, 17196);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3882);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17197);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 3883);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17198);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 40741);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 68775);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 94210);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 113287);
													s.is_empty()
												}
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
							super::push(out, 40742);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68776);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94211);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3884);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17199);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40743);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 40744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17200);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40745);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68777);
									s.is_empty()
								}
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
			super::push(out, 3885);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17201);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40746);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68778);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 94212);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3886);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17202);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрстенау.
pub(crate) fn n114495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113288);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 125577);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сау.
pub(crate) fn n3906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3895);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 17220);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40763);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тау.
pub(crate) fn n3907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3896);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17221);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40764);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 40765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17222);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 17223);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40766);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 68791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 17224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чау.
pub(crate) fn n3908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3897);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 17225);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 40767);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68792);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 94217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шау.
pub(crate) fn n3909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3898);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17226);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40768);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 68793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17227);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 40769);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 68794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17228);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40770);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 68795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17229);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40771);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бу.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3899);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17230);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17231);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40773);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3900);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3901);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17235);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40775);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68798);
									s.is_empty()
								}
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
			super::push(out, 3902);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17236);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ву.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 3903);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17237);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40776);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68799);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94218);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3904);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17238);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40777);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68800);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94219);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 113289);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гу.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3905);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17239);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40778);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17240);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17241);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17242);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40780);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 68802);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94220);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 113290);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17243);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40781);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68803);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 94221);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 113291);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3906);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3907);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17246);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 40783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3908);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17247);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17248);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40784);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 68804);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 94222);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 113292);
													match s {
														[] => true,
														['р', s @ ..] => super::t26::n126878(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3909);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17249);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40785);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 68805);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94223);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 113293);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17250);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40786);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68806);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94224);
											s.is_empty()
										}
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
					super::push(out, 17251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3910);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17252);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40787);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 68807);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94225);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3911);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17253);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40788);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 68808);
									s.is_empty()
								}
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
			super::push(out, 3912);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 17254);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ду.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3913);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17255);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 17256);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40790);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68809);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94226);
											s.is_empty()
										}
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
					super::push(out, 17257);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40791);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 17258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3914);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 17259);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40792);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68811);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94227);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3915);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17260);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17261);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40793);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 68812);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 94228);
											s.is_empty()
										}
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
					super::push(out, 17262);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40794);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 68813);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 94229);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 113294);
													match s {
														[] => true,
														['д', s @ ..] => super::t26::n126879(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3916);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17263);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40795);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 68814);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 94230);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113295);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3917);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17264);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40796);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68815);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94231);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3918);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17265);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 40797);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 40798);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68816);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94232);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 113296);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17266);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 40799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3919);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17267);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40800);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68817);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3920);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17269);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17271);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жу.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3921);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17272);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40802);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 40803);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68818);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94233);
											s.is_empty()
										}
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
					super::push(out, 17273);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40804);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68819);
									s.is_empty()
								}
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
			super::push(out, 3922);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17274);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 40805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17275);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зу.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3923);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17276);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 3924);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17277);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40808);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 68820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17278);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40809);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 68821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17279);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 17280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3925);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17281);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40811);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68822);
									s.is_empty()
								}
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
			super::push(out, 3926);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17282);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17283);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 3927);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17284);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иу.
pub(crate) fn n417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 416);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3928);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17285);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3929);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3930);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17287);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40815);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68823);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 94234);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113297);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3931);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17288);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40816);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68824);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94235);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3932);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17289);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 40817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3933);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ку.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 417);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3934);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 17291);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40819);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68825);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 94236);
											s.is_empty()
										}
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
					super::push(out, 17292);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40820);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68826);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94237);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 113298);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17293);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40821);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 17294);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40823);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68827);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 94238);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3935);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17295);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40824);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68828);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94239);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3936);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17296);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 40825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 3937);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17297);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 40826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3938);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17298);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40827);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17299);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40828);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 17300);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40829);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 68831);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 94240);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3939);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17301);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 3940);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17302);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40831);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 68832);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 94241);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лу.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 418);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3941);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17303);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 17304);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40832);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 17305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17306);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40834);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68834);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94242);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 113299);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3942);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3943);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 17308);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3944);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17309);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40836);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68835);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 94243);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113300);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3945);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17310);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40837);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68836);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 94244);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 113301);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17311);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17312);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -му.
pub(crate) fn n420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 3946);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17313);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 40838);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 68837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40839);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 3947);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17315);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40840);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ну.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3948);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17316);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 40841);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68839);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 94245);
											s.is_empty()
										}
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
					super::push(out, 17317);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 3949);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17318);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40843);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 68840);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 94246);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 113302);
													match s {
														[] => true,
														['и', s @ ..] => super::t26::n126880(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3950);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17319);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3951);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17320);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40845);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68841);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 94247);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 113303);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n126881(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3952);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17321);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40846);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68842);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 94248);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113304);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3953);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17322);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17323);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 40848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3954);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17324);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 40849);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68843);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 94249);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 113305);
													match s {
														[] => true,
														['в', s @ ..] => super::t26::n126882(s, out),
														['ч', s @ ..] => super::t26::n126883(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3955);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17325);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 3956);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17326);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 3957);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17327);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40852);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68844);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94250);
											s.is_empty()
										}
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
					super::push(out, 17328);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40853);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68845);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94251);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 113306);
													match s {
														[] => true,
														['у', s @ ..] => super::t26::n126884(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оу.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 3958);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17329);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40854);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68846);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 94252);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 113307);
													match s {
														[] => true,
														['п', s @ ..] => super::t26::n126885(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 3959);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17330);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40856);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 68847);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94253);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 113308);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n126886(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40857);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68848);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 94254);
											s.is_empty()
										}
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
					super::push(out, 17331);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40858);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68849);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 94255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 40859);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 68850);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 94256);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3960);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 17332);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40860);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 68851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17333);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 40861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3961);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17334);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40862);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68852);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 94257);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 113309);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n126887(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40863);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 40864);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68854);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 94258);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 113310);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n126888(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 17335);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 40865);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 68855);
									s.is_empty()
								}
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
			super::push(out, 3962);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17336);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17337);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40866);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 68856);
									s.is_empty()
								}
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
			super::push(out, 3963);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17338);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40867);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68857);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94259);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113311);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 3964);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17340);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пу.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 422);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3965);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3966);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17342);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 17343);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ру.
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 3967);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40869);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 68858);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94260);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3968);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17345);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3969);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3970);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17347);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40872);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68859);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 40873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3971);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17348);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40874);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68860);
									s.is_empty()
								}
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
			super::push(out, 3972);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17349);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 17350);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17351);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 40876);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68861);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94261);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 113312);
													match s {
														[] => true,
														['р', s @ ..] => super::t26::n126889(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40877);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 68862);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94262);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 113313);
													match s {
														[] => true,
														['к', s @ ..] => super::t26::n126890(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3973);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -су.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3974);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17353);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 40878);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68863);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94263);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 113314);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n126891(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3975);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3976);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17355);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40879);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68864);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 94264);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 113315);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3977);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3978);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3979);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 17358);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40880);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 68865);
									s.is_empty()
								}
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
			super::push(out, 3980);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17359);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40881);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68866);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 94265);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3981);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17360);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 40882);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 68867);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 94266);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 113316);
													match s {
														[] => true,
														['и', s @ ..] => super::t26::n126892(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3982);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17361);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ту.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 425);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3983);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17362);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 17363);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17364);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 40883);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 68868);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94267);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 113317);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17365);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40884);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68869);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 94268);
											s.is_empty()
										}
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
					super::push(out, 17366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 3984);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17367);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 17368);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40885);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68870);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94269);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 113318);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3985);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17369);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 40886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 3986);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17370);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40887);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68871);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94270);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 113319);
													s.is_empty()
												}
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
							super::push(out, 40888);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 40889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 40890);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 68873);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 94271);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 113320);
													match s {
														[] => true,
														['и', s @ ..] => super::t26::n126893(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 17371);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 40891);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 68874);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 94272);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 113321);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3987);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17372);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40892);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68875);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94273);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 113322);
													s.is_empty()
												}
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
							super::push(out, 40893);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 40894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17373);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 40895);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 68877);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94274);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 113323);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n126894(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3988);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 17374);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 40896);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68878);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 94275);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3989);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17375);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 40897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уу.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 426);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 3990);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 17376);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 40898);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 68879);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 94276);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113324);
													match s {
														[] => true,
														['в', s @ ..] => super::t26::n126895(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фу.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 3991);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17377);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40899);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68880);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94277);
											s.is_empty()
										}
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
					super::push(out, 17378);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40900);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68881);
									s.is_empty()
								}
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
			super::push(out, 3992);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 3993);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17380);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 3994);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17381);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40902);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68882);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94278);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3995);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17382);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 40903);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 40904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3996);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ху.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3997);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17384);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17385);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40905);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68883);
									s.is_empty()
								}
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
			super::push(out, 3998);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17386);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17387);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40907);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68884);
									s.is_empty()
								}
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
			super::push(out, 3999);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17388);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40908);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68885);
									s.is_empty()
								}
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
			super::push(out, 4000);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17389);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 40909);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68886);
									s.is_empty()
								}
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
			super::push(out, 4001);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 17390);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4002);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 4003);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 17392);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40910);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 68887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цу.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4004);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40911);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 40912);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68888);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 94279);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 113325);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17394);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 40913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 17395);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17396);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40915);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 68889);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 17397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4005);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17399);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40918);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 68890);
									s.is_empty()
								}
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
			super::push(out, 4006);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17400);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40919);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 68891);
									s.is_empty()
								}
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
			super::push(out, 4007);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17401);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40920);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чу.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 430);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4008);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17402);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40921);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 68892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шу.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4009);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 17403);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 40922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 17404);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 40923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 17405);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 40924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4010);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 17406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4011);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17407);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40925);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68893);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 40926);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 68894);
									s.is_empty()
								}
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
			super::push(out, 4012);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 17408);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 40927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4013);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17409);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40928);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 68895);
									s.is_empty()
								}
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
			super::push(out, 4014);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17410);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40929);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 68896);
									s.is_empty()
								}
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
			super::push(out, 4015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яу.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 432);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4016);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 17411);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40930);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 68897);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 94280);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аф.
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 433);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4017);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17412);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 40931);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 68898);
									s.is_empty()
								}
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
			super::push(out, 4018);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 4019);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 4020);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17413);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40932);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68899);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94281);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 40933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 17414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40934);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 40935);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68900);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94282);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 113326);
													match s {
														[] => true,
														['т', s @ ..] => super::t26::n126896(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4021);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4022);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17416);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 40937);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 68901);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17417);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 40938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4023);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17418);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 40939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68902);
									s.is_empty()
								}
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
			super::push(out, 4024);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17419);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 40940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t26::n4036(s, out),
		['с', s @ ..] => {
			super::push(out, 4026);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4027);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17426);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 40959);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 68943);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 94350);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 113393);
													match s {
														[] => true,
														['б', s @ ..] => super::t26::n126947(s, out),
														_ => false,
													}
												}
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
							super::push(out, 40960);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 68944);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94351);
											s.is_empty()
										}
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
					super::push(out, 17427);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 40961);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 68945);
									s.is_empty()
								}
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
			super::push(out, 4028);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17428);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40962);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68946);
									s.is_empty()
								}
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
			super::push(out, 4029);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иф.
pub(crate) fn n436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 435);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4039);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 17446);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40972);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68952);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94354);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4040);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17447);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 40973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4041);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17448);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 40974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4042);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17449);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 40975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4043);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 17450);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4044);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40978);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 40979);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 40980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 17453);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68953);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 94355);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 68954);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94356);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 40981);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68955);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94357);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 40982);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68956);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 94358);
											s.is_empty()
										}
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
					super::push(out, 17454);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 40983);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 68957);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94359);
											s.is_empty()
										}
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
					super::push(out, 17455);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 17456);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 17457);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40985);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68958);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 94360);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 40986);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68959);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94361);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4045);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17458);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 40987);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 68960);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 94362);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 113394);
													match s {
														[] => true,
														['р', s @ ..] => super::t26::n126948(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4046);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17459);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 40988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68961);
									s.is_empty()
								}
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
			super::push(out, 4047);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 17460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 4048);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17461);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 40990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4049);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17462);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 40991);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68962);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 94363);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 40992);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 40993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 17463);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 17464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 40994);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => super::t26::n69687(s, out),
								['п', s @ ..] => super::t26::n69688(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t26::n17564(s, out),
				['к', s @ ..] => super::t26::n17565(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t26::n4061(s, out),
		['т', s @ ..] => super::t26::n4062(s, out),
		['ф', s @ ..] => super::t26::n4063(s, out),
		['ч', s @ ..] => super::t26::n4064(s, out),
		['ш', s @ ..] => super::t26::n4065(s, out),
		_ => false,
	}
}

/// Words ending -ллоунайф.
pub(crate) fn n114608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113398);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 125651);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 133043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аканф.
pub(crate) fn n41423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41010);
	s.is_empty()
}

/// Words ending -санф.
pub(crate) fn n17581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17482);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41011);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 68977);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 94376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инф.
pub(crate) fn n4072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4061);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 17483);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41012);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 68978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17484);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41013);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 68979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41014);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 68980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оф.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4062);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17485);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41015);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 68981);
									s.is_empty()
								}
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
			super::push(out, 4063);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17486);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41016);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68982);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94377);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 113399);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4064);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 4065);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17487);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41017);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 41018);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68983);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94378);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 113400);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4066);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17488);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17489);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41020);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68984);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 94379);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 113401);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n126952(s, out),
														_ => false,
													}
												}
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
							super::push(out, 41021);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 41022);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68985);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 94380);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113402);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n126953(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4067);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 17490);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41023);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 68986);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 41024);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 68987);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 94381);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 41025);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 68988);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94382);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 113403);
													match s {
														[] => true,
														['н', s @ ..] => super::t26::n126954(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4068);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17491);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41026);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68989);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94383);
											s.is_empty()
										}
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
					super::push(out, 17492);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41027);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 68990);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94384);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 113404);
													s.is_empty()
												}
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
							super::push(out, 41028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 4069);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 17493);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41029);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пф.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 4070);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17494);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 17495);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 41030);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 17496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рф.
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4072);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 17497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4073);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 17498);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 41031);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 68993);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94385);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 113405);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4074);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 17499);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 41032);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 68994);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 17500);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41033);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 68995);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 94386);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 113406);
													match s {
														[] => true,
														['д', s @ ..] => super::t26::n126955(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 17501);
					match s {
						[] => true,
						['и', 'р', s @ ..] => {
							super::push(out, 41034);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 68996);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 94387);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41035);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 68997);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 94388);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 113407);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 68998);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 68999);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 69000);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 94389);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113408);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 69001);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 94390);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 113409);
													match s {
														[] => true,
														['т', s @ ..] => super::t26::n126956(s, out),
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
									super::push(out, 69002);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94391);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 113410);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 69003);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94392);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 113411);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41036);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 41037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4075);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 17503);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41038);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17504);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 17505);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уф.
pub(crate) fn n445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4076);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4077);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 4078);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17508);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 17509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4079);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17510);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41042);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69005);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 94393);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 113412);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17511);
					match s {
						[] => true,
						['к', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 69006);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 69007);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94394);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4080);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 4081);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 17512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4082);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 17513);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41043);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 69008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17514);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 17515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4083);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 17516);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фф.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4084);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17517);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 17518);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 17519);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17520);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4085);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 17521);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41046);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69009);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 94395);
											s.is_empty()
										}
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
					super::push(out, 17522);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 17523);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41047);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 69010);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 94396);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 113413);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4086);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17525);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 41048);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69011);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 94397);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 113414);
													s.is_empty()
												}
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
							super::push(out, 41049);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 69012);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 94398);
											s.is_empty()
										}
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
					super::push(out, 17526);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4087);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41050);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 41051);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 69013);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 94399);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 113415);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьф.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4088);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 17528);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 17529);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41053);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 69014);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 41054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 17530);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 17531);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41056);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 41057);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 69015);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 69016);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 94400);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 113416);
													match s {
														[] => true,
														['м', s @ ..] => super::t26::n126957(s, out),
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
									super::push(out, 69017);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 94401);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 41058);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 69018);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 69019);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 94402);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 113417);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n126958(s, out),
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
									super::push(out, 69020);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 94403);
											s.is_empty()
										}
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
					super::push(out, 17532);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41059);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 69021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 17533);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эф.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 4089);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4090);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 17534);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юф.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 4091);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 17535);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41060);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 69022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дзах.
pub(crate) fn n17641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17542);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41066);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ках.
pub(crate) fn n4106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4095);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17543);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41067);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лах.
pub(crate) fn n4107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4096);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17544);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 17545);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 41068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 41069);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69027);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 94405);
									s.is_empty()
								}
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
			super::push(out, 17546);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17547);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41071);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69028);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 69029);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 94406);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 113418);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125659);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n134415(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 41072);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 69030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41073);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69031);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 94407);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113419);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125660);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41074);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 69032);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 94408);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113420);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 125661);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17548);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41075);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41076);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 69034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'а', 'ч', '\u{1}', s @ ..] => {
			super::push(out, 17550);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 17551);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 41077);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 69035);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мах.
pub(crate) fn n4108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4097);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17552);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 41078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 17553);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 17554);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41079);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69036);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 41080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 17555);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 41081);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 69037);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 94411);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 113421);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125662);
													match s {
														[] => true,
														['а', s @ ..] => super::t26::n134416(s, out),
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
									super::push(out, 94412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41082);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 69038);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41083);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 69039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 41084);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 69040);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 94414);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113422);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 125663);
													match s {
														[] => true,
														['м', s @ ..] => super::t26::n134417(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 17556);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 17557);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 41085);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нах.
pub(crate) fn n4109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4098);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 17558);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 41087);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 69042);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 94415);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 113423);
											s.is_empty()
										}
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
					super::push(out, 41088);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 41089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 69043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 41090);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 69044);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 94416);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 113424);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 17559);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41091);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 69045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 17560);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41092);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 69046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 17561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41093);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 69047);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 94417);
									s.is_empty()
								}
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
			super::push(out, 17562);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 41094);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 69048);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 94418);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 113425);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125664);
													match s {
														[] => true,
														['л', s @ ..] => super::t26::n134418(s, out),
														_ => false,
													}
												}
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
							super::push(out, 69049);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 69050);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 94419);
									s.is_empty()
								}
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
			super::push(out, 17563);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41095);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 69051);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 94420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

