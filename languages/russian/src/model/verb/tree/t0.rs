use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['и', s @ ..] => super::t0::n1(s, out),
		['ь', s @ ..] => super::t0::n2(s, out),
		['я', s @ ..] => super::t0::n3(s, out),
		_ => false,
	}
}

/// Words ending -и.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['т', s @ ..] => super::t0::n4(s, out),
		['щ', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60);
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

/// Words ending -ь.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['с', s @ ..] => super::t0::n6(s, out),
		['т', s @ ..] => super::t0::n7(s, out),
		['ч', s @ ..] => super::t0::n8(s, out),
		_ => false,
	}
}

/// Words ending -я.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['с', s @ ..] => super::t0::n9(s, out),
		_ => false,
	}
}

/// Words ending -ти.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 36);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 172);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 173);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 174);
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
						['ъ', s @ ..] => {
							super::push(out, 175);
							match s {
								[] => true,
								['з', s @ ..] => {
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 176);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 1187);
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
					super::push(out, 37);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 177);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 1188);
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
				['я', s @ ..] => {
					super::push(out, 38);
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
		['г', s @ ..] => {
			super::push(out, 11);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 39);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 178);
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
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 179);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 180);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 1189);
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
				['ы', s @ ..] => {
					super::push(out, 41);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 181);
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
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42);
					match s {
						[] => true,
						['в', s @ ..] => super::t0::n185(s, out),
						['л', s @ ..] => {
							super::push(out, 183);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 1196);
									match s {
										[] => true,
										['в', s @ ..] => {
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
						_ => false,
					}
				}
				['л', s @ ..] => super::t0::n43(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t0::n14(s, out),
		['й', s @ ..] => super::t0::n15(s, out),
		['с', s @ ..] => super::t0::n16(s, out),
		['т', s @ ..] => super::t0::n17(s, out),
		['ы', s @ ..] => super::t0::n18(s, out),
		['я', s @ ..] => super::t0::n19(s, out),
		_ => false,
	}
}

/// Words ending -сь.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['и', s @ ..] => super::t0::n21(s, out),
		_ => false,
	}
}

/// Words ending -ть.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['а', s @ ..] => super::t0::n22(s, out),
		['е', s @ ..] => super::t0::n23(s, out),
		['з', s @ ..] => {
			super::push(out, 24);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 103);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 612);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2561);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 2562);
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
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 2563);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											super::push(out, 7206);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 2564);
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
									super::push(out, 2565);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 7207);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 14538);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 14539);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t0::n34912(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 104);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 613);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 2566);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 105);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 614);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 2567);
									match s {
										[] => true,
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
				['ы', s @ ..] => {
					super::push(out, 106);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 615);
							match s {
								[] => true,
								['г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7208);
											s.is_empty()
										}
										['а', s @ ..] => {
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => super::t0::n19803(s, out),
												['н', '\u{1}', s @ ..] => super::t0::n19804(s, out),
												_ => false,
											}
										}
										['б', 'о', '\u{1}', s @ ..] => super::t0::n8750(s, out),
										['в', '\u{1}', s @ ..] => super::t0::n8751(s, out),
										['д', s @ ..] => super::t0::n8752(s, out),
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t0::n8753(s, out),
										['з', s @ ..] => super::t0::n8754(s, out),
										['о', s @ ..] => super::t0::n8755(s, out),
										['с', '\u{1}', s @ ..] => super::t0::n8756(s, out),
										['т', 'о', '\u{1}', s @ ..] => super::t0::n8757(s, out),
										['у', '\u{1}', s @ ..] => super::t0::n8758(s, out),
										['ы', s @ ..] => super::t0::n8759(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t0::n25(s, out),
		['о', s @ ..] => super::t0::n26(s, out),
		['с', s @ ..] => super::t0::n27(s, out),
		['у', s @ ..] => super::t0::n28(s, out),
		['ы', s @ ..] => super::t0::n29(s, out),
		['я', s @ ..] => super::t0::n30(s, out),
		_ => false,
	}
}

/// Words ending -чь.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['е', s @ ..] => super::t0::n31(s, out),
		['и', s @ ..] => {
			super::push(out, 32);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 165);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 1165);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10567);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 10568);
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
										['о', s @ ..] => {
											super::push(out, 10569);
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
												['с', '\u{1}', s @ ..] => {
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
											super::push(out, 10570);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 18094);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n40289(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 166);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 1166);
							match s {
								[] => true,
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 167);
					match s {
						[] => true,
						['о', s @ ..] => super::t0::n1207(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t0::n168(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t0::n34(s, out),
		_ => false,
	}
}

/// Words ending -ся.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['ь', s @ ..] => super::t0::n35(s, out),
		_ => false,
	}
}

/// Words ending -везти.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1190);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1191);
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
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
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
		['з', s @ ..] => {
			super::push(out, 1192);
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1193);
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
				['п', s @ ..] => {
					super::push(out, 4310);
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
			super::push(out, 1194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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
			super::push(out, 1195);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4311);
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

/// Words ending -лзти.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4312);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4313);
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 4314);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 11283);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 19981);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11284);
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
						['с', s @ ..] => {
							super::push(out, 4316);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4317);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11285);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ити.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 44);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 185);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 186);
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

/// Words ending -йти.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 187);
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
				['н', s @ ..] => {
					super::push(out, 188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1197);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1198);
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
		['е', s @ ..] => {
			super::push(out, 46);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 191);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 1201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 193);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 194);
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
				['з', s @ ..] => {
					super::push(out, 195);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 1202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4318);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 11286);
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
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4319);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 4320);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11287);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 11288);
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
									super::push(out, 4321);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 11289);
											s.is_empty()
										}
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
					super::push(out, 196);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 197);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 1203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4322);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 4323);
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
				['с', s @ ..] => {
					super::push(out, 198);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 49);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 50);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 200);
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
				['з', s @ ..] => {
					super::push(out, 201);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 1204);
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

/// Words ending -сти.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1205);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 1206);
							match s {
								[] => true,
								['з', s @ ..] => {
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
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 1207);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 1208);
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
							super::push(out, 1209);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
							super::push(out, 1210);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4324);
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
					super::push(out, 203);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1211);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 1212);
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
										['о', 'п', '\u{1}', s @ ..] => {
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
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 1213);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 1214);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 4325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 1215);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1216);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 1217);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4326);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'о', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1218);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 1219);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 4327);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 1220);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 4328);
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
		['е', s @ ..] => super::t0::n53(s, out),
		['у', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 210);
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
		['ю', s @ ..] => super::t0::n55(s, out),
		['я', s @ ..] => super::t0::n56(s, out),
		_ => false,
	}
}

/// Words ending -тти.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыти.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 215);
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

/// Words ending -яти.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 216);
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

/// Words ending -ись.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['т', s @ ..] => super::t0::n61(s, out),
		_ => false,
	}
}

/// Words ending -ать.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['б', s @ ..] => super::t0::n62(s, out),
		['в', s @ ..] => super::t0::n63(s, out),
		['г', s @ ..] => super::t0::n64(s, out),
		['д', s @ ..] => super::t0::n65(s, out),
		['ж', s @ ..] => super::t0::n66(s, out),
		['з', s @ ..] => super::t0::n67(s, out),
		['к', s @ ..] => super::t0::n68(s, out),
		['л', s @ ..] => {
			super::push(out, 69);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 328);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 1744);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13223);
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
										['о', s @ ..] => {
											super::push(out, 13224);
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
						_ => false,
					}
				}
				['е', s @ ..] => super::t0::n333(s, out),
				['з', s @ ..] => {
					super::push(out, 330);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 1747);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 5795);
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
				['и', s @ ..] => super::t0::n335(s, out),
				['й', s @ ..] => super::t0::n336(s, out),
				['к', s @ ..] => super::t0::n337(s, out),
				['м', s @ ..] => super::t0::n338(s, out),
				['о', s @ ..] => super::t0::n339(s, out),
				['с', s @ ..] => super::t0::n340(s, out),
				['т', s @ ..] => super::t0::n341(s, out),
				['ы', s @ ..] => super::t0::n342(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t0::n70(s, out),
		['н', s @ ..] => super::t0::n71(s, out),
		['п', s @ ..] => super::t0::n72(s, out),
		['р', s @ ..] => super::t0::n73(s, out),
		['с', s @ ..] => super::t1::n74(s, out),
		['т', s @ ..] => super::t1::n75(s, out),
		['ф', s @ ..] => super::t1::n76(s, out),
		['х', s @ ..] => super::t1::n77(s, out),
		['ц', s @ ..] => super::t1::n78(s, out),
		['ч', s @ ..] => super::t1::n79(s, out),
		['ш', s @ ..] => super::t1::n80(s, out),
		['щ', s @ ..] => super::t1::n81(s, out),
		_ => false,
	}
}

/// Words ending -еть.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 82);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 83);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 465);
					match s {
						[] => true,
						['л', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6676);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 6677);
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
					super::push(out, 466);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2193);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 6678);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 14072);
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
						['р', s @ ..] => {
							super::push(out, 2194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6679);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 6680);
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
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 467);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 2195);
							match s {
								[] => true,
								['в', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 14073);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 22487);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n51284(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2196);
							match s {
								[] => true,
								['к', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 14074);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 22488);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n51285(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 468);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6681);
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
						['з', s @ ..] => {
							super::push(out, 2197);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 6682);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 14075);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 2198);
							match s {
								[] => true,
								['о', 'г', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14076);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 14077);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
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
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6683);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 6684);
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
				['ы', s @ ..] => {
					super::push(out, 469);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 2199);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 6685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 470);
					match s {
						[] => true,
						['р', s @ ..] => super::t1::n2326(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t1::n85(s, out),
		['г', s @ ..] => super::t1::n86(s, out),
		['д', s @ ..] => super::t1::n87(s, out),
		['ж', s @ ..] => super::t1::n88(s, out),
		['з', s @ ..] => super::t1::n89(s, out),
		['л', s @ ..] => super::t1::n90(s, out),
		['м', s @ ..] => super::t1::n91(s, out),
		['н', s @ ..] => super::t1::n92(s, out),
		['о', s @ ..] => super::t1::n93(s, out),
		['п', s @ ..] => super::t1::n94(s, out),
		['р', s @ ..] => super::t1::n95(s, out),
		['с', s @ ..] => super::t1::n96(s, out),
		['т', s @ ..] => super::t1::n97(s, out),
		['у', s @ ..] => super::t1::n98(s, out),
		['х', s @ ..] => super::t1::n99(s, out),
		['ч', s @ ..] => super::t1::n100(s, out),
		['ш', s @ ..] => super::t1::n101(s, out),
		['ы', s @ ..] => super::t1::n102(s, out),
		_ => false,
	}
}

/// The word повылезть.
pub(crate) fn n34912(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word загрызть.
pub(crate) fn n19803(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word нагрызть.
pub(crate) fn n19804(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word обгрызть.
pub(crate) fn n8750(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word вгрызть.
pub(crate) fn n8751(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -дгрызть.
pub(crate) fn n8752(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word перегрызть.
pub(crate) fn n8753(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -згрызть.
pub(crate) fn n8754(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -огрызть.
pub(crate) fn n8755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7209);
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

/// The word сгрызть.
pub(crate) fn n8756(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отгрызть.
pub(crate) fn n8757(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word угрызть.
pub(crate) fn n8758(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыгрызть.
pub(crate) fn n8759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7210);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14540);
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

/// Words ending -ить.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 2569);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 7211);
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
					super::push(out, 618);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7212);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 7213);
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
									super::push(out, 7214);
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 7215);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 14541);
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
				['т', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2571);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 2572);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 2573);
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
						['у', s @ ..] => {
							super::push(out, 2574);
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
		['б', s @ ..] => super::t1::n108(s, out),
		['в', s @ ..] => super::t1::n109(s, out),
		['д', s @ ..] => super::t1::n110(s, out),
		['е', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 669);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 2750);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 7655);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 670);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 2751);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 7656);
									match s {
										[] => true,
										['о', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 671);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 2752);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7657);
									s.is_empty()
								}
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => super::t1::n20560(s, out),
										['н', '\u{1}', s @ ..] => super::t1::n20561(s, out),
										_ => false,
									}
								}
								['б', 'о', '\u{1}', s @ ..] => super::t1::n9288(s, out),
								['в', '\u{1}', s @ ..] => super::t1::n9289(s, out),
								['д', s @ ..] => super::t1::n9290(s, out),
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => super::t1::n9291(s, out),
								['и', 'р', 'п', '\u{1}', s @ ..] => super::t1::n9292(s, out),
								['о', s @ ..] => super::t1::n9293(s, out),
								['с', s @ ..] => super::t1::n9294(s, out),
								['т', 'о', '\u{1}', s @ ..] => super::t1::n9295(s, out),
								['у', '\u{1}', s @ ..] => super::t1::n9296(s, out),
								['ы', s @ ..] => super::t1::n9297(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t1::n2942(s, out),
						_ => false,
					}
				}
				['п', 'о', 'р', 'в', 'е', 'о', '\u{1}', s @ ..] => super::t1::n682(s, out),
				['ф', s @ ..] => super::t1::n683(s, out),
				['ч', s @ ..] => super::t1::n684(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t1::n112(s, out),
		['з', s @ ..] => super::t1::n113(s, out),
		['л', s @ ..] => super::t1::n114(s, out),
		['м', s @ ..] => super::t1::n115(s, out),
		['н', s @ ..] => super::t1::n116(s, out),
		['о', s @ ..] => super::t1::n117(s, out),
		['п', s @ ..] => super::t1::n118(s, out),
		['р', s @ ..] => super::t1::n119(s, out),
		['с', s @ ..] => super::t1::n120(s, out),
		['т', s @ ..] => super::t1::n121(s, out),
		['у', s @ ..] => super::t1::n122(s, out),
		['ф', s @ ..] => super::t1::n123(s, out),
		['ч', s @ ..] => super::t2::n124(s, out),
		['ш', s @ ..] => super::t2::n125(s, out),
		['щ', s @ ..] => super::t2::n126(s, out),
		_ => false,
	}
}

/// Words ending -оть.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3695);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9798);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9799);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 17310);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24568);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 17311);
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
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									match s {
										[] => true,
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											super::push(out, 17312);
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
									super::push(out, 9800);
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
								['с', s @ ..] => {
									super::push(out, 9801);
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
								['т', s @ ..] => {
									super::push(out, 9802);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 9803);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17313);
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
							super::push(out, 3696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9804);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9805);
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
									super::push(out, 9806);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', s @ ..] => super::t2::n24111(s, out),
										_ => false,
									}
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => super::t2::n11804(s, out),
								['о', s @ ..] => super::t2::n11805(s, out),
								['с', s @ ..] => super::t2::n11806(s, out),
								['т', 'о', '\u{1}', s @ ..] => super::t2::n11807(s, out),
								['ы', s @ ..] => super::t2::n11808(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t2::n3893(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t2::n128(s, out),
		_ => false,
	}
}

/// Words ending -сть.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 899);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 3700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9823);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 9824);
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
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9825);
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
									super::push(out, 9826);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17319);
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
											super::push(out, 17320);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 17321);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 24569);
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
								['ы', s @ ..] => {
									super::push(out, 9827);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 17322);
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
					super::push(out, 900);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3701);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3702);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 9828);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 9829);
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
							super::push(out, 3703);
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
							super::push(out, 3704);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						['с', s @ ..] => {
							super::push(out, 3705);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
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
							super::push(out, 3706);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 9830);
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
				['р', s @ ..] => super::t2::n914(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t2::n130(s, out),
		['я', s @ ..] => super::t2::n131(s, out),
		_ => false,
	}
}

/// Words ending -уть.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 919);
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
		['д', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 920);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 921);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3737);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 3738);
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
				['б', s @ ..] => {
					super::push(out, 922);
					match s {
						[] => true,
						['о', s @ ..] => {
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
				['в', s @ ..] => {
					super::push(out, 923);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 925);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 3739);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3740);
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
					super::push(out, 927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 929);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 9865);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 17338);
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
		['з', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3743);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t2::n135(s, out),
		_ => false,
	}
}

/// Words ending -ыть.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 3902);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
								['о', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
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
				['е', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3903);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 10304);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 17889);
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
									super::push(out, 10305);
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
					super::push(out, 962);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 3904);
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
					super::push(out, 963);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 3905);
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
					super::push(out, 964);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 3906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 10306);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 17890);
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
						['п', s @ ..] => {
							super::push(out, 3907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 3908);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10307);
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
					super::push(out, 965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 966);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 3909);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 967);
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
				['ы', s @ ..] => {
					super::push(out, 968);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 3910);
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
			super::push(out, 137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 971);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 972);
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
		['л', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['п', s @ ..] => super::t2::n991(s, out),
				['с', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3921);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3922);
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
		['м', s @ ..] => super::t2::n139(s, out),
		['н', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 981);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['з', s @ ..] => super::t2::n4184(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => super::t2::n1007(s, out),
				['о', s @ ..] => super::t2::n1008(s, out),
				['у', s @ ..] => super::t2::n1009(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t2::n141(s, out),
		['т', s @ ..] => super::t2::n142(s, out),
		_ => false,
	}
}

/// Words ending -ять.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 143);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3940);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 3941);
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
					super::push(out, 995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3942);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3943);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 3944);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 3945);
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
				['к', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3947);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 3948);
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
							super::push(out, 3949);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 10317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 3950);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 10318);
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
							super::push(out, 3951);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 10319);
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
							super::push(out, 3952);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10320);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 10321);
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
							super::push(out, 3953);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 10322);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 10323);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 10324);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 17892);
											match s {
												[] => true,
												['у', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 3954);
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
				['м', s @ ..] => super::t2::n1028(s, out),
				['п', s @ ..] => super::t2::n1029(s, out),
				['р', s @ ..] => super::t2::n1030(s, out),
				['т', s @ ..] => super::t2::n1031(s, out),
				['х', s @ ..] => super::t2::n1032(s, out),
				['ч', s @ ..] => super::t2::n1033(s, out),
				['ш', s @ ..] => super::t2::n1034(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t2::n145(s, out),
		['е', s @ ..] => super::t2::n146(s, out),
		['з', s @ ..] => super::t2::n147(s, out),
		['и', s @ ..] => super::t2::n148(s, out),
		['л', s @ ..] => super::t2::n149(s, out),
		['м', s @ ..] => super::t2::n150(s, out),
		['н', s @ ..] => super::t2::n151(s, out),
		['о', s @ ..] => super::t2::n152(s, out),
		['п', s @ ..] => super::t2::n153(s, out),
		['р', s @ ..] => super::t2::n154(s, out),
		['с', s @ ..] => super::t2::n155(s, out),
		['у', s @ ..] => super::t2::n156(s, out),
		['ъ', s @ ..] => super::t2::n157(s, out),
		_ => false,
	}
}

/// Words ending -ечь.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1104);
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
			super::push(out, 159);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1105);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1106);
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
					super::push(out, 1107);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4160);
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
					super::push(out, 1108);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4161);
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
					super::push(out, 1109);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1110);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4162);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 4163);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4164);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10553);
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
					super::push(out, 1111);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4165);
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
				['о', s @ ..] => {
					super::push(out, 1112);
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
					super::push(out, 1113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4166);
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
					super::push(out, 1115);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4167);
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
			super::push(out, 160);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1116);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1117);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 4168);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
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
				['б', 'о', '\u{1}', s @ ..] => {
					super::push(out, 1118);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4169);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4170);
							match s {
								[] => true,
								['з', s @ ..] => super::t2::n13100(s, out),
								['н', '\u{1}', s @ ..] => super::t2::n13101(s, out),
								_ => false,
							}
						}
						['з', s @ ..] => super::t2::n4561(s, out),
						['и', 'р', 'п', '\u{1}', s @ ..] => super::t2::n4562(s, out),
						['о', s @ ..] => super::t2::n4563(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t2::n4564(s, out),
						['у', s @ ..] => super::t2::n4565(s, out),
						_ => false,
					}
				}
				['д', s @ ..] => super::t2::n1153(s, out),
				['е', s @ ..] => super::t2::n1154(s, out),
				['з', s @ ..] => super::t2::n1155(s, out),
				['и', s @ ..] => super::t2::n1156(s, out),
				['о', s @ ..] => super::t2::n1157(s, out),
				['с', s @ ..] => super::t2::n1158(s, out),
				['т', s @ ..] => super::t2::n1159(s, out),
				['у', s @ ..] => super::t2::n1160(s, out),
				['ы', s @ ..] => super::t2::n1161(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t2::n161(s, out),
		['р', s @ ..] => super::t2::n162(s, out),
		['с', s @ ..] => super::t2::n163(s, out),
		['т', s @ ..] => super::t2::n164(s, out),
		_ => false,
	}
}

/// The word выстричь.
pub(crate) fn n40289(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -олочь.
pub(crate) fn n1207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1167);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4206);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4207);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10571);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 10572);
					match s {
						[] => true,
						['з', s @ ..] => {
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
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 10573);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['з', s @ ..] => {
					super::push(out, 10574);
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
				['и', s @ ..] => {
					super::push(out, 10575);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 18095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10576);
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
					super::push(out, 10577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18096);
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
					super::push(out, 10578);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18097);
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
			super::push(out, 4208);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10579);
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
					super::push(out, 10580);
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
					super::push(out, 10581);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
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

/// Words ending -мочь.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1168);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['н', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1171);
			match s {
				[] => true,
				['о', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10582);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 18098);
									s.is_empty()
								}
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
			super::push(out, 1172);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10583);
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
		['с', s @ ..] => {
			super::push(out, 1173);
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

/// Words ending -ячь.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4209);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18099);
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18100);
											s.is_empty()
										}
										['д', s @ ..] => {
											match s {
												[] => true,
												['е', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 4210);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10584);
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
						['и', s @ ..] => {
							super::push(out, 4211);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 10585);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 18101);
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
							super::push(out, 4212);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 10586);
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
							super::push(out, 4214);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 10587);
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

/// Words ending -ься.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['т', s @ ..] => super::t2::n170(s, out),
		['ч', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['е', s @ ..] => super::t2::n1224(s, out),
				['и', s @ ..] => super::t2::n1225(s, out),
				['о', s @ ..] => super::t2::n1226(s, out),
				['я', s @ ..] => super::t2::n1227(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ести.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1222);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
				['б', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1225);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1226);
					match s {
						[] => true,
						['р', 'е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
				['з', s @ ..] => {
					super::push(out, 1227);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'о', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1228);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1229);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', s @ ..] => {
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
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 1230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4329);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 11290);
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
				['т', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 1233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4330);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4331);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 11291);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19982);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4332);
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
						['с', s @ ..] => {
							super::push(out, 4333);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 11292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4334);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 11293);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19983);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 4335);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 19984);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 19985);
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
				['ы', s @ ..] => super::t2::n1309(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t2::n209(s, out),
		['м', s @ ..] => super::t2::n210(s, out),
		['н', s @ ..] => super::t2::n211(s, out),
		['р', s @ ..] => super::t2::n212(s, out),
		_ => false,
	}
}

/// Words ending -юсти.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4378);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4379);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 4380);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясти.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4382);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 4383);
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
						['б', 'о', s @ ..] => {
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
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 4384);
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
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', s @ ..] => {
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 4386);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11319);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тись.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1269);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 218);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1270);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ъ', 'з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 1271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4388);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 4389);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 11320);
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
			super::push(out, 220);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1272);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11321);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 11322);
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
								['ы', s @ ..] => {
									super::push(out, 11323);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 19993);
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
				['л', s @ ..] => {
					super::push(out, 1273);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4391);
							match s {
								[] => true,
								['п', 'с', s @ ..] => {
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
				['р', s @ ..] => {
					super::push(out, 1274);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 4392);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 11324);
									match s {
										[] => true,
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
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1275);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 4393);
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							super::push(out, 4394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1276);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4395);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 11325);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							super::push(out, 4396);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 4397);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 11326);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28397);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 28398);
													match s {
														[] => true,
														['п', s @ ..] => super::t2::n61654(s, out),
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
									super::push(out, 11327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 4398);
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							super::push(out, 4399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t2::n226(s, out),
		_ => false,
	}
}

/// Words ending -бать.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 4411);
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
					super::push(out, 1285);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 4412);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 224);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1286);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1287);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 4413);
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
						['н', s @ ..] => {
							super::push(out, 4414);
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
					super::push(out, 1288);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 4415);
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
				['л', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 4416);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 11364);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 20029);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 20030);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 28413);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n61672(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 20031);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 28414);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n61673(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 20032);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 28415);
													match s {
														[] => true,
														['р', s @ ..] => super::t2::n61674(s, out),
														_ => false,
													}
												}
												['в', s @ ..] => super::t2::n45839(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => super::t2::n4964(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t2::n1370(s, out),
				['р', s @ ..] => super::t2::n1371(s, out),
				['т', s @ ..] => super::t2::n1372(s, out),
				['у', s @ ..] => super::t2::n1373(s, out),
				['ъ', s @ ..] => super::t2::n1374(s, out),
				['ы', s @ ..] => super::t2::n1375(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t2::n230(s, out),
		['л', s @ ..] => super::t2::n231(s, out),
		['о', s @ ..] => super::t2::n232(s, out),
		['р', s @ ..] => super::t2::n233(s, out),
		['у', s @ ..] => super::t2::n234(s, out),
		['ы', s @ ..] => super::t2::n235(s, out),
		['ю', s @ ..] => super::t2::n236(s, out),
		['я', s @ ..] => super::t2::n237(s, out),
		_ => false,
	}
}

/// Words ending -вать.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['д', s @ ..] => super::t2::n1392(s, out),
				['л', s @ ..] => {
					super::push(out, 1312);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 4459);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11417);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 11418);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 20054);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 20055);
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
									super::push(out, 11419);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 20056);
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
									super::push(out, 11420);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => super::t2::n28331(s, out),
										['р', 'п', '\u{1}', s @ ..] => super::t2::n28332(s, out),
										_ => false,
									}
								}
								['с', s @ ..] => super::t2::n14265(s, out),
								['т', s @ ..] => super::t2::n14266(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t2::n1394(s, out),
				['н', s @ ..] => super::t2::n1395(s, out),
				['т', s @ ..] => super::t2::n1396(s, out),
				['х', s @ ..] => super::t2::n1397(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t2::n239(s, out),
		['з', s @ ..] => super::t2::n240(s, out),
		['и', s @ ..] => super::t2::n241(s, out),
		['о', s @ ..] => super::t2::n242(s, out),
		['р', s @ ..] => super::t2::n243(s, out),
		['у', s @ ..] => super::t2::n244(s, out),
		['ы', s @ ..] => super::t2::n245(s, out),
		['ю', s @ ..] => super::t2::n246(s, out),
		['я', s @ ..] => super::t2::n247(s, out),
		_ => false,
	}
}

/// Words ending -гать.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1402);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5017);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['з', s @ ..] => {
							super::push(out, 5018);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['в', 'и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['к', 'у', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 21401);
											match s {
												[] => true,
												['а', 'р', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n65123(s, out),
														['д', 'е', 'р', 'п', '\u{1}', s @ ..] => super::t2::n65124(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 21402);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 30019);
													match s {
														[] => true,
														['и', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => super::t2::n65125(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
							super::push(out, 5019);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 1403);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5020);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5021);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12443);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 12444);
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
							super::push(out, 5022);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5023);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12445);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 12446);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 12447);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 12448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 5024);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 12449);
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
							super::push(out, 5025);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 5026);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 5027);
							match s {
								[] => true,
								['в', s @ ..] => super::t2::n15905(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t3::n249(s, out),
		['з', s @ ..] => super::t3::n250(s, out),
		['и', s @ ..] => super::t3::n251(s, out),
		['л', s @ ..] => super::t3::n252(s, out),
		['м', s @ ..] => super::t3::n253(s, out),
		['н', s @ ..] => super::t3::n254(s, out),
		['о', s @ ..] => super::t3::n255(s, out),
		['р', s @ ..] => super::t3::n256(s, out),
		['у', s @ ..] => super::t3::n257(s, out),
		['ы', s @ ..] => super::t3::n258(s, out),
		['ь', s @ ..] => super::t3::n259(s, out),
		['ю', s @ ..] => super::t3::n260(s, out),
		['я', s @ ..] => super::t3::n261(s, out),
		_ => false,
	}
}

/// Words ending -дать.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 258);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5122);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5123);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12556);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 12557);
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
							super::push(out, 5124);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12558);
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
							super::push(out, 5125);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 5126);
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
						['и', s @ ..] => {
							super::push(out, 5127);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 12559);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 21467);
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
							super::push(out, 5128);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 12560);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 12561);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 12562);
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
							super::push(out, 5129);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 5130);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 5131);
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
						['ы', s @ ..] => {
							super::push(out, 5132);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 12563);
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
					super::push(out, 1443);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t3::n1529(s, out),
				['п', s @ ..] => super::t3::n1530(s, out),
				['р', s @ ..] => super::t3::n1531(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 1448);
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
			super::push(out, 261);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 5149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t3::n266(s, out),
		['е', s @ ..] => super::t3::n267(s, out),
		['ж', s @ ..] => super::t3::n268(s, out),
		['з', s @ ..] => super::t3::n269(s, out),
		['и', s @ ..] => super::t3::n270(s, out),
		['н', s @ ..] => super::t3::n271(s, out),
		['о', s @ ..] => super::t3::n272(s, out),
		['с', s @ ..] => super::t3::n273(s, out),
		['т', s @ ..] => super::t3::n274(s, out),
		['у', s @ ..] => super::t3::n275(s, out),
		['ы', s @ ..] => super::t3::n276(s, out),
		['ю', s @ ..] => super::t3::n277(s, out),
		['я', s @ ..] => super::t3::n278(s, out),
		_ => false,
	}
}

/// Words ending -жать.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 275);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1499);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 1500);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 5251);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12675);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 12676);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 12677);
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
				['з', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 5252);
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
				['к', s @ ..] => {
					super::push(out, 1502);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 5253);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 12678);
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
							super::push(out, 5254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5255);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5256);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 12679);
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
							super::push(out, 5257);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 21533);
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 12680);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 5258);
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
				['н', s @ ..] => {
					super::push(out, 1504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 5259);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12681);
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
							super::push(out, 5260);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 12682);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 21534);
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
							super::push(out, 5261);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5262);
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
					super::push(out, 1505);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5263);
							s.is_empty()
						}
						['б', s @ ..] => super::t3::n6020(s, out),
						['д', s @ ..] => {
							super::push(out, 5265);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 12683);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 12684);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 21536);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => super::t3::n6022(s, out),
						['о', s @ ..] => {
							super::push(out, 5267);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 5268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t3::n6025(s, out),
						['ы', s @ ..] => super::t3::n6026(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t3::n1596(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n281(s, out),
		['в', s @ ..] => super::t3::n282(s, out),
		['д', s @ ..] => super::t3::n283(s, out),
		['е', s @ ..] => super::t3::n284(s, out),
		['ж', s @ ..] => super::t3::n285(s, out),
		['з', s @ ..] => super::t3::n286(s, out),
		['и', s @ ..] => super::t3::n287(s, out),
		['л', s @ ..] => super::t3::n288(s, out),
		['н', s @ ..] => super::t3::n289(s, out),
		['о', s @ ..] => super::t3::n290(s, out),
		['р', s @ ..] => super::t3::n291(s, out),
		['с', s @ ..] => super::t3::n292(s, out),
		['т', s @ ..] => super::t3::n293(s, out),
		['у', s @ ..] => super::t3::n294(s, out),
		['ы', s @ ..] => super::t3::n295(s, out),
		['ю', s @ ..] => super::t3::n296(s, out),
		['я', s @ ..] => super::t3::n297(s, out),
		_ => false,
	}
}

/// Words ending -зать.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 294);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5353);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5354);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 12757);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 12758);
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
							super::push(out, 5356);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'н', s @ ..] => {
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
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									match s {
										[] => true,
										['е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
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
								['с', 'а', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 12759);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 21573);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
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
						['ы', s @ ..] => {
							super::push(out, 5357);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 12760);
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
					super::push(out, 1561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5358);
							s.is_empty()
						}
						['а', s @ ..] => super::t3::n6160(s, out),
						['б', s @ ..] => super::t3::n6161(s, out),
						['е', s @ ..] => super::t3::n6162(s, out),
						['з', s @ ..] => super::t3::n6163(s, out),
						['о', s @ ..] => super::t3::n6164(s, out),
						['с', s @ ..] => super::t3::n6165(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t3::n1656(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n299(s, out),
		['е', s @ ..] => super::t3::n300(s, out),
		['и', s @ ..] => super::t3::n301(s, out),
		['л', s @ ..] => super::t3::n302(s, out),
		['н', s @ ..] => super::t3::n303(s, out),
		['р', s @ ..] => super::t3::n304(s, out),
		['у', s @ ..] => super::t3::n305(s, out),
		['ы', s @ ..] => super::t3::n306(s, out),
		['ь', s @ ..] => super::t3::n307(s, out),
		['ю', s @ ..] => super::t3::n308(s, out),
		['я', s @ ..] => super::t3::n309(s, out),
		_ => false,
	}
}

/// Words ending -кать.
pub(crate) fn n68(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 306);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1587);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1588);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 5434);
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
					super::push(out, 1589);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 5435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 12847);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 21610);
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
							super::push(out, 5436);
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
				['г', s @ ..] => {
					super::push(out, 1590);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5437);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5438);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5439);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 5440);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 12848);
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
							super::push(out, 5441);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 21611);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 21612);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 30094);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n65206(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t3::n1686(s, out),
				['л', s @ ..] => super::t3::n1687(s, out),
				['м', s @ ..] => {
					super::push(out, 1594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5453);
							s.is_empty()
						}
						['о', s @ ..] => super::t3::n6287(s, out),
						['ш', s @ ..] => {
							super::push(out, 5455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => super::t3::n6289(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t3::n1689(s, out),
				['р', s @ ..] => super::t3::n1690(s, out),
				['т', s @ ..] => super::t3::n1691(s, out),
				['х', s @ ..] => super::t3::n1692(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n311(s, out),
		['в', s @ ..] => super::t3::n312(s, out),
		['е', s @ ..] => super::t3::n313(s, out),
		['и', s @ ..] => super::t3::n314(s, out),
		['й', s @ ..] => super::t3::n315(s, out),
		['л', s @ ..] => super::t3::n316(s, out),
		['м', s @ ..] => super::t3::n317(s, out),
		['н', s @ ..] => super::t3::n318(s, out),
		['о', s @ ..] => super::t3::n319(s, out),
		['р', s @ ..] => super::t3::n320(s, out),
		['с', s @ ..] => super::t4::n321(s, out),
		['т', s @ ..] => super::t4::n322(s, out),
		['у', s @ ..] => super::t4::n323(s, out),
		['ц', s @ ..] => super::t4::n324(s, out),
		['ч', s @ ..] => super::t4::n325(s, out),
		['ш', s @ ..] => super::t4::n326(s, out),
		['ы', s @ ..] => super::t4::n327(s, out),
		['ь', s @ ..] => super::t4::n328(s, out),
		['э', s @ ..] => super::t4::n329(s, out),
		['ю', s @ ..] => super::t4::n330(s, out),
		['я', s @ ..] => super::t4::n331(s, out),
		_ => false,
	}
}

/// Words ending -елать.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 329);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 1745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5787);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5788);
					match s {
						[] => true,
						['з', s @ ..] => {
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
				['б', 'о', s @ ..] => {
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', s @ ..] => {
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
				['з', s @ ..] => {
					super::push(out, 5789);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
								_ => false,
							}
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
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
					super::push(out, 5790);
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
								['о', 'п', '\u{1}', s @ ..] => {
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
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', s @ ..] => {
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5791);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13225);
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
		['ж', s @ ..] => {
			super::push(out, 1746);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5792);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5793);
					match s {
						[] => true,
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5794);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 13226);
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

/// Words ending -илать.
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 1748);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['о', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йлать.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1749);
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

/// Words ending -клать.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1750);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5796);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -млать.
pub(crate) fn n338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1751);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5797);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олать.
pub(crate) fn n339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1752);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5798);
					match s {
						[] => true,
						['л', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21836);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21837);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слать.
pub(crate) fn n340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1753);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1754);
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
							super::push(out, 13227);
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1755);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13228);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5799);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 13229);
							match s {
								[] => true,
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 13230);
							match s {
								[] => true,
								['и', 'н', '\u{1}', s @ ..] => {
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 1756);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5800);
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

/// Words ending -тлать.
pub(crate) fn n341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1757);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5801);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13231);
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
		['с', s @ ..] => {
			super::push(out, 1758);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5802);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5803);
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
					super::push(out, 5804);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5805);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21838);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21839);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30187);
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
							super::push(out, 13233);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 21840);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30188);
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
									super::push(out, 21841);
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
							super::push(out, 13234);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13235);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21842);
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
					super::push(out, 5806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5807);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13236);
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

/// Words ending -ылать.
pub(crate) fn n342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5808);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5809);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5810);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 13237);
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
					super::push(out, 5811);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5812);
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
		['с', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5813);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5814);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21843);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 30189);
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
					super::push(out, 5815);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 13238);
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
							super::push(out, 13239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 21844);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30190);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 30191);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													super::push(out, 36040);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
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

/// Words ending -мать.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1761);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 1762);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5817);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5818);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 13240);
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
							super::push(out, 5819);
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
		['е', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1764);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1765);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5820);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13241);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 13242);
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
								['з', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13243);
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
								['у', s @ ..] => {
									super::push(out, 13244);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 21845);
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
						['т', s @ ..] => {
							super::push(out, 5821);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 13245);
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
			super::push(out, 341);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1766);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 1767);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 5822);
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
					super::push(out, 1768);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5823);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 13246);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t4::n32887(s, out),
										_ => false,
									}
								}
								['н', s @ ..] => super::t4::n17311(s, out),
								_ => false,
							}
						}
						['о', 'д', s @ ..] => super::t4::n6740(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => super::t4::n1871(s, out),
				['и', s @ ..] => super::t4::n1872(s, out),
				['н', s @ ..] => super::t4::n1873(s, out),
				['р', 'п', '\u{1}', s @ ..] => super::t4::n1874(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t4::n346(s, out),
		['о', s @ ..] => super::t4::n347(s, out),
		['у', s @ ..] => super::t4::n348(s, out),
		['ы', s @ ..] => super::t4::n349(s, out),
		_ => false,
	}
}

/// Words ending -нать.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 1780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5859);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 5860);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 13270);
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
				['м', s @ ..] => {
					super::push(out, 1781);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5861);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5862);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 5863);
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
		['г', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1783);
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
					super::push(out, 1784);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 5864);
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
					super::push(out, 1785);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5865);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 13271);
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
							super::push(out, 5866);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1786);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 5867);
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
					super::push(out, 1787);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 5868);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 5869);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13272);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13273);
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
							super::push(out, 5870);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
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
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 5871);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 5872);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 13274);
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
							super::push(out, 5873);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 5874);
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
					super::push(out, 1788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 5875);
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
		['е', s @ ..] => {
			super::push(out, 348);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 5876);
							match s {
								[] => true,
								['п', s @ ..] => super::t4::n17390(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5877);
							match s {
								[] => true,
								['н', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 5878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t4::n353(s, out),
		['з', s @ ..] => super::t4::n354(s, out),
		['и', s @ ..] => super::t4::n355(s, out),
		['й', s @ ..] => super::t4::n356(s, out),
		['о', s @ ..] => super::t4::n357(s, out),
		['р', s @ ..] => super::t4::n358(s, out),
		['т', s @ ..] => super::t4::n359(s, out),
		['у', s @ ..] => super::t4::n360(s, out),
		_ => false,
	}
}

/// Words ending -пать.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1814);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 1815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5909);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 5910);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 13301);
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
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
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
							super::push(out, 5911);
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
						['с', s @ ..] => {
							super::push(out, 5912);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
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
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 5913);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13302);
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
					super::push(out, 1816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5914);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 1817);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5915);
							match s {
								[] => true,
								['ц', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 21885);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 21886);
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
										['о', s @ ..] => {
											super::push(out, 21887);
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
												['р', 'п', '\u{1}', s @ ..] => super::t4::n50487(s, out),
												_ => false,
											}
										}
										['с', s @ ..] => super::t4::n33000(s, out),
										['ы', s @ ..] => super::t4::n33001(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => super::t4::n6884(s, out),
						['к', s @ ..] => super::t4::n6885(s, out),
						_ => false,
					}
				}
				['ф', s @ ..] => super::t4::n1923(s, out),
				['х', s @ ..] => super::t4::n1924(s, out),
				['ц', s @ ..] => super::t4::n1925(s, out),
				['ч', s @ ..] => super::t4::n1926(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n362(s, out),
		['и', s @ ..] => super::t4::n363(s, out),
		['й', s @ ..] => super::t4::n364(s, out),
		['о', s @ ..] => super::t4::n365(s, out),
		['р', s @ ..] => super::t4::n366(s, out),
		['с', s @ ..] => super::t4::n367(s, out),
		['у', s @ ..] => super::t4::n368(s, out),
		['ы', s @ ..] => super::t4::n369(s, out),
		['ю', s @ ..] => super::t4::n370(s, out),
		['я', s @ ..] => super::t4::n371(s, out),
		_ => false,
	}
}

/// Words ending -рать.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1855);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1856);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 6029);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6030);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6031);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 13429);
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
					super::push(out, 1858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6032);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6033);
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
							super::push(out, 6034);
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 6035);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13430);
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
		['б', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1859);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 1860);
					match s {
						[] => true,
						['з', s @ ..] => {
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
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
					super::push(out, 1861);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 6036);
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
					super::push(out, 1862);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 1863);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
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
				['о', s @ ..] => {
					super::push(out, 1864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 6037);
							match s {
								[] => true,
								['о', s @ ..] => super::t4::n17689(s, out),
								_ => false,
							}
						}
						['в', s @ ..] => super::t4::n7055(s, out),
						['д', s @ ..] => super::t4::n7056(s, out),
						['з', s @ ..] => super::t4::n7057(s, out),
						['п', s @ ..] => super::t4::n7058(s, out),
						['р', s @ ..] => super::t4::n7059(s, out),
						['с', s @ ..] => super::t4::n7060(s, out),
						['т', s @ ..] => super::t4::n7061(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t4::n1970(s, out),
				['ы', s @ ..] => super::t4::n1971(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t4::n374(s, out),
		['г', s @ ..] => super::t4::n375(s, out),
		['д', s @ ..] => super::t4::n376(s, out),
		['ж', s @ ..] => super::t4::n377(s, out),
		['и', s @ ..] => super::t4::n378(s, out),
		['й', s @ ..] => super::t4::n379(s, out),
		['о', s @ ..] => super::t4::n380(s, out),
		['п', s @ ..] => super::t4::n381(s, out),
		['с', s @ ..] => super::t4::n382(s, out),
		_ => false,
	}
}

