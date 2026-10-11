use flexion::Path;

/// Words ending -лянка.
pub(crate) fn n21833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21713);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 47550);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76154);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100414);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117759);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128696);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47551);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 47552);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76156);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 76157);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 76158);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47553);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76159);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47554);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 76160);
					s.is_empty()
				}
				['д', 'е', s @ ..] => {
					super::push(out, 76161);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100418);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 100419);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117760);
									s.is_empty()
								}
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
			super::push(out, 47555);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76162);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 76163);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 47556);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100421);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117761);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 128697);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 135145);
													match s {
														[] => true,
														['в', s @ ..] => super::t49::n140271(s, out),
														_ => false,
													}
												}
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
							super::push(out, 100422);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117762);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 76164);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100423);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 100424);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117763);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128698);
											s.is_empty()
										}
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
					super::push(out, 76165);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100425);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117764);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128699);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47557);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76166);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 76167);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 76168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76169);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100427);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117765);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128700);
											s.is_empty()
										}
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
					super::push(out, 76170);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47558);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76171);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 76172);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100430);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117766);
									s.is_empty()
								}
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
			super::push(out, 47559);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76173);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'в', '\u{1}', s @ ..] => {
					super::push(out, 76174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 47560);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76175);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мянка.
pub(crate) fn n21834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21714);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47561);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 47562);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76176);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100433);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 100434);
							match s {
								[] => true,
								['с', '-', s @ ..] => {
									super::push(out, 117767);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128701);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 135146);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n140272(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 76177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47563);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100435);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 100436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47564);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47565);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 47566);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76179);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 47567);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нянка.
pub(crate) fn n21835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21715);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47568);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76180);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100438);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117768);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128702);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135147);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47569);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76181);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100439);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117769);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128703);
											s.is_empty()
										}
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
					super::push(out, 76182);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100440);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117770);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 76183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 47570);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76184);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100441);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117771);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128704);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47571);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76185);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47572);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76186);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100443);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117772);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128705);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135148);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 76187);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100444);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117773);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128706);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 135149);
													s.is_empty()
												}
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
							super::push(out, 100445);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117774);
									s.is_empty()
								}
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
			super::push(out, 47573);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76188);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 100446);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117775);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128707);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47574);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76189);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117776);
									s.is_empty()
								}
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
			super::push(out, 47575);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76190);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100448);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117777);
									s.is_empty()
								}
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
			super::push(out, 47576);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76191);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100449);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117778);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128708);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 100450);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оянка.
pub(crate) fn n21836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21716);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 47577);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76192);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 100451);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 117780);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128709);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135150);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140273(s, out),
														_ => false,
													}
												}
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
							super::push(out, 100452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пянка.
pub(crate) fn n21837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21717);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 47578);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76193);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рянка.
pub(crate) fn n21838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21718);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47579);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 76194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 47580);
			match s {
				[] => true,
				['е', 'р', 'е', 'с', '\u{1}', s @ ..] => {
					super::push(out, 76195);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 76196);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'с', s @ ..] => {
			super::push(out, 47581);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'ж', s @ ..] => {
			super::push(out, 47582);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47583);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76199);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 76200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 47584);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76201);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 76202);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 76203);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100456);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117781);
									s.is_empty()
								}
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
			super::push(out, 47585);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 47586);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 76205);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сянка.
pub(crate) fn n21839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21719);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47587);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76206);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 47588);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					super::push(out, 76207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'р', 'п', s @ ..] => {
			super::push(out, 47589);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47590);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76209);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тянка.
pub(crate) fn n21840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21720);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47591);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47592);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76211);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 76212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47593);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 47594);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 47595);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47596);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100460);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 100461);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117782);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 100462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 76215);
					s.is_empty()
				}
				['о', 'к', 'о', s @ ..] => {
					super::push(out, 76216);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 76217);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47597);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 47598);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76219);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 47599);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76220);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фянка.
pub(crate) fn n21841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21721);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 47600);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76221);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьянка.
pub(crate) fn n21842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21722);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 47601);
			s.is_empty()
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76222);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117783);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 117784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 76223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 47602);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 47603);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'е', 'л', 'а', 'в', '\u{1}', s @ ..] => {
			super::push(out, 47604);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 47605);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -авка-ярмарка.
pub(crate) fn n140274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138839);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140920);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142133);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кокалиберка.
pub(crate) fn n136559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135153);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядерка.
pub(crate) fn n48209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47726);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 76354);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 76355);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 100554);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жерка.
pub(crate) fn n21920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21800);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47727);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76356);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100555);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100556);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 100557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 76357);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47728);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 76358);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100559);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117829);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128726);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135154);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47729);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76359);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 100560);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117830);
									s.is_empty()
								}
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
			super::push(out, 47730);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76360);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 47731);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76361);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100562);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 117831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зерка.
pub(crate) fn n21921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21801);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47732);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 76363);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 100564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 76364);
					match s {
						[] => true,
						['н', 'а', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 100565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47733);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76365);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 76366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47734);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47735);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76368);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100566);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117832);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128727);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -керка.
pub(crate) fn n21922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21802);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47736);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100567);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 76370);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100568);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117834);
									s.is_empty()
								}
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
			super::push(out, 47737);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76371);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100569);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117835);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128728);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47738);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 47739);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лерка.
pub(crate) fn n21923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21803);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47740);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47741);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76373);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100570);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117836);
									s.is_empty()
								}
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
			super::push(out, 47742);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76374);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мерка.
pub(crate) fn n21924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21804);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76375);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100572);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117837);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128729);
											s.is_empty()
										}
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
					super::push(out, 76376);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 76377);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100573);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117838);
									s.is_empty()
								}
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
			super::push(out, 47743);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 47744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76379);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'д', s @ ..] => {
			super::push(out, 47745);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76380);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 47746);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 47747);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76382);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нерка.
pub(crate) fn n21925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21805);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47748);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76383);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 76384);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 76385);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 76386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47749);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76387);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 47750);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76388);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100581);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117839);
									match s {
										[] => true,
										['о', 'к', '\u{1}', s @ ..] => {
											super::push(out, 128730);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47751);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76389);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100582);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 100583);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117840);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 117841);
									match s {
										[] => true,
										['л', 'о', 'в', 'е', 'р', '-', s @ ..] => {
											super::push(out, 128731);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135155);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n140276(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 76390);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100584);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перка.
pub(crate) fn n21926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21806);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 47752);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76391);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100585);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117843);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128732);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47753);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47754);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76393);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100586);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117844);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128733);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -серка.
pub(crate) fn n21927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21807);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 47755);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76394);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 47756);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76395);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'и', 'ж', 'е', 'р', '\u{1}', s @ ..] => {
			super::push(out, 47757);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 47758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -терка.
pub(crate) fn n21928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21808);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47759);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76396);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'а', s @ ..] => {
					super::push(out, 76397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 47760);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47761);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76399);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100592);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117845);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128734);
											s.is_empty()
										}
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
					super::push(out, 76400);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100593);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 117846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 76401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 47762);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76402);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 76403);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 100596);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117847);
									s.is_empty()
								}
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
			super::push(out, 47763);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76404);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100597);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117848);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128735);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47764);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 76405);
					match s {
						[] => true,
						['у', 'б', s @ ..] => {
							super::push(out, 100598);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 76406);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 76407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 47765);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76408);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47766);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76409);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100600);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 100601);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 76410);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 100602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 76411);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100603);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', 'а', 'ш', '\u{1}', s @ ..] => {
			super::push(out, 47767);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 47768);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76412);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100604);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117853);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128736);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47769);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ферка.
pub(crate) fn n21929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21809);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 47770);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 76414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -церка.
pub(crate) fn n21930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21810);
	s.is_empty()
}

/// Words ending -черка.
pub(crate) fn n21931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21811);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47771);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76415);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шерка.
pub(crate) fn n21932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21812);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 47772);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76416);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щерка.
pub(crate) fn n21933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21813);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 47773);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 47774);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76418);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьерка.
pub(crate) fn n21934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21814);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 47775);
			match s {
				[] => true,
				['и', 'п', s @ ..] => {
					super::push(out, 76419);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100607);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117854);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128737);
											s.is_empty()
										}
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
					super::push(out, 76420);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -львадорка.
pub(crate) fn n119133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117871);
	s.is_empty()
}

/// Words ending -идорка.
pub(crate) fn n48294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47810);
	s.is_empty()
}

/// Words ending -жорка.
pub(crate) fn n21951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21831);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47811);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', 'о', '\u{1}', s @ ..] => {
			super::push(out, 47812);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47813);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 47814);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76454);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100634);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 117872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зорка.
pub(crate) fn n21952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21832);
	s.is_empty()
}

/// Words ending -иорка.
pub(crate) fn n21953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21833);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 47815);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 76455);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йорка.
pub(crate) fn n21954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21834);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 47816);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 76456);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 100636);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117873);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 128745);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135156);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140277(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -корка.
pub(crate) fn n21955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21835);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 47817);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76457);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47818);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47819);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 47820);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 76458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47821);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 47822);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 47823);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76460);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100640);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лорка.
pub(crate) fn n21956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21836);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47824);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76461);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100641);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117875);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128746);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47825);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -морка.
pub(crate) fn n21957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21837);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47826);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47827);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76463);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -норка.
pub(crate) fn n21958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21838);
	match s {
		[] => true,
		['и', 'м', '\u{1}', s @ ..] => {
			super::push(out, 47828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -порка.
pub(crate) fn n21959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21839);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47829);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 76464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 47830);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76465);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100643);
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
				['к', s @ ..] => {
					super::push(out, 76466);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 76467);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 76468);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100645);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117876);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128747);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 47831);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47832);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 47833);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сорка.
pub(crate) fn n21960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21840);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 47834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76472);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47835);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47836);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торка.
pub(crate) fn n21961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21841);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47837);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76475);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100649);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117877);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128748);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135157);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140278(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 76476);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100650);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117878);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128749);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135158);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 117879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 100651);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117880);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128750);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 100652);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117881);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128751);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135159);
													match s {
														[] => true,
														['р', s @ ..] => super::t49::n140279(s, out),
														_ => false,
													}
												}
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
							super::push(out, 100653);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 117882);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128752);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135160);
													s.is_empty()
												}
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
							super::push(out, 100654);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 117883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 76477);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100655);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117884);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128753);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135161);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n140280(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 76478);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 100656);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 117885);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 128754);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 135162);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47838);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100657);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 100658);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 76480);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47839);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76481);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 76482);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 47840);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76483);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 47841);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 47842);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 47843);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76486);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100662);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 76487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 47844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -форка.
pub(crate) fn n21962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21842);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 47845);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 47846);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хорка.
pub(crate) fn n21963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21843);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 47847);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 47848);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цорка.
pub(crate) fn n21964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21844);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 47849);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76490);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шорка.
pub(crate) fn n21965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21845);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47850);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 47851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -самосброска.
pub(crate) fn n136570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135164);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 138846);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ососка.
pub(crate) fn n48459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47973);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 76570);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100720);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 117914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 76571);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 76572);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 100721);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 117915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тоска.
pub(crate) fn n22035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21915);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 47974);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 47975);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76573);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 100722);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 117916);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128764);
											s.is_empty()
										}
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
					super::push(out, 76574);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100723);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 117917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кокшетауска.
pub(crate) fn n136571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135165);
	s.is_empty()
}

/// Words ending -темиртауска.
pub(crate) fn n136572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135166);
	s.is_empty()
}

/// Words ending -мка-коляска.
pub(crate) fn n136573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135167);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 138847);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 140923);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сло-коляска.
pub(crate) fn n136574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135168);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 138848);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140924);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word лоботряска.
pub(crate) fn n136575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135169);
	s.is_empty()
}

/// Words ending -очка-касатка.
pub(crate) fn n140284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138849);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140925);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бородавчатка.
pub(crate) fn n140285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138850);
	s.is_empty()
}

/// Words ending -тень-печатка.
pub(crate) fn n140286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138851);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 140926);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142136);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word шестилетка.
pub(crate) fn n136582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135176);
	s.is_empty()
}

/// Words ending -уболетка.
pub(crate) fn n101867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100784);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 117956);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 128780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -голетка.
pub(crate) fn n77486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76677);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 100785);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 117957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лолетка.
pub(crate) fn n77488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76679);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 100786);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 117958);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нолетка.
pub(crate) fn n77489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76680);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 100787);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 117959);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ролетка.
pub(crate) fn n77490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76681);
	s.is_empty()
}

/// Words ending -толетка.
pub(crate) fn n77491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76682);
	s.is_empty()
}

/// Words ending -плетка.
pub(crate) fn n48570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48079);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 76683);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 100788);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 117960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 76684);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 100789);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 117961);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 128781);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135177);
											s.is_empty()
										}
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
					super::push(out, 100790);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 117962);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 128782);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 135178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлетка.
pub(crate) fn n48571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48080);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 76685);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 100791);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 117963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлетка.
pub(crate) fn n48572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48081);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 76686);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100792);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 117964);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 76687);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 100793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улетка.
pub(crate) fn n48573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48082);
	s.is_empty()
}

/// Words ending -хлетка.
pub(crate) fn n48574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48083);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 76688);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 100794);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 117965);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 117966);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 128783);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 135179);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 138852);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n142372(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -метка.
pub(crate) fn n22080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21958);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 48084);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76689);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нетка.
pub(crate) fn n22081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21959);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48085);
			s.is_empty()
		}
		['и', 'д', s @ ..] => {
			super::push(out, 48086);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76690);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 100796);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117967);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76691);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 76692);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100797);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117968);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128784);
											s.is_empty()
										}
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
					super::push(out, 76693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 48087);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76694);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -петка.
pub(crate) fn n22082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21960);
	s.is_empty()
}

/// Words ending -ретка.
pub(crate) fn n22083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21961);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48088);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 48089);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 76695);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100799);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117969);
									s.is_empty()
								}
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
			super::push(out, 48090);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76696);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 100800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48091);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48092);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 76697);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100801);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 117970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 76698);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 76699);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100802);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 117971);
									s.is_empty()
								}
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
			super::push(out, 48093);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 48094);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76700);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сетка.
pub(crate) fn n22084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21962);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 48095);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76701);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 100803);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 117972);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128785);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135180);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n140288(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48096);
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76702);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 76703);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 100804);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 117973);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128786);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135181);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тетка.
pub(crate) fn n22085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21963);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48097);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 48098);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 76704);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фетка.
pub(crate) fn n22086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21964);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 48099);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 48100);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 76705);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117974);
									s.is_empty()
								}
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
			super::push(out, 48101);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 48102);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76706);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 117975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -четка.
pub(crate) fn n22087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21965);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48103);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76707);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', '\u{1}', s @ ..] => {
					super::push(out, 76708);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шетка.
pub(crate) fn n22088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21966);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щетка.
pub(crate) fn n22089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21967);
	s.is_empty()
}

/// Words ending -воспитка.
pub(crate) fn n101906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100823);
	s.is_empty()
}

/// Words ending -ритка.
pub(crate) fn n22103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21976);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100824);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 100825);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 117982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 76737);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100826);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 117983);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128788);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48135);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76738);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48136);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76739);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48137);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76740);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100829);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 117984);
									s.is_empty()
								}
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
			super::push(out, 48138);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ситка.
pub(crate) fn n22104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21977);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 48139);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48140);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -титка.
pub(crate) fn n22105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21978);
	s.is_empty()
}

/// Words ending -уитка.
pub(crate) fn n22106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21979);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 48141);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76743);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 48142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фитка.
pub(crate) fn n22107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21980);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 48143);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76744);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 100831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цитка.
pub(crate) fn n22108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21981);
	s.is_empty()
}

/// Words ending -читка.
pub(crate) fn n22109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21982);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 48144);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щитка.
pub(crate) fn n22110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21983);
	s.is_empty()
}

/// Words ending -довыработка.
pub(crate) fn n136588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135182);
	s.is_empty()
}

/// The word живоглотка.
pub(crate) fn n136589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135183);
	s.is_empty()
}

/// The word ротоглотка.
pub(crate) fn n136590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135184);
	s.is_empty()
}

/// The word грамотка.
pub(crate) fn n119259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117997);
	s.is_empty()
}

/// Words ending -бмотка.
pub(crate) fn n48658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48162);
	s.is_empty()
}

/// Words ending -рмотка.
pub(crate) fn n48659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48163);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 76761);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 100851);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 117998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128796);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нотка.
pub(crate) fn n22123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21996);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48164);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76762);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 76763);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100852);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 117999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -потка.
pub(crate) fn n22124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21997);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48165);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48166);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 76764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48167);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76765);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ротка.
pub(crate) fn n22125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21998);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 48168);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48169);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 48170);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48171);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76769);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 76770);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100855);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118001);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128797);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135185);
													s.is_empty()
												}
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
							super::push(out, 100856);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 100857);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 100858);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 76771);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100859);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 76772);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100860);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сотка.
pub(crate) fn n22126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21999);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48172);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76773);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100861);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 118006);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128798);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 135186);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140289(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48173);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 48174);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 48175);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76774);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 100862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118007);
									s.is_empty()
								}
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
			super::push(out, 48176);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 100863);
							s.is_empty()
						}
						['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 100864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 48177);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тотка.
pub(crate) fn n22127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22000);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 48178);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76776);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100865);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118008);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128799);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 135187);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хотка.
pub(crate) fn n22128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22001);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48179);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 76777);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 76778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48180);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48181);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76779);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 100866);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118009);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128800);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135188);
													match s {
														[] => true,
														['н', s @ ..] => super::t49::n140290(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щотка.
pub(crate) fn n22129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22002);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48182);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76780);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							super::push(out, 100867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьотка.
pub(crate) fn n22130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22003);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 48183);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -листовертка.
pub(crate) fn n136595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135189);
	s.is_empty()
}

/// Words ending -кистка.
pub(crate) fn n48699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48202);
	s.is_empty()
}

/// Words ending -листка.
pub(crate) fn n48700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48203);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 76799);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 76800);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 100883);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 118020);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 128805);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135190);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мистка.
pub(crate) fn n48701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48204);
	s.is_empty()
}

/// Words ending -нистка.
pub(crate) fn n48702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48205);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 76801);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 100884);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118021);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128806);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135191);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 76802);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 100885);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118022);
							match s {
								[] => true,
								['м', '-', s @ ..] => {
									super::push(out, 128807);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 135192);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 138856);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n142374(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 76803);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 100886);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118023);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 128808);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 135193);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138857);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n142375(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 100887);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 118024);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 128809);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 135194);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 138858);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n142376(s, out),
														_ => false,
													}
												}
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
							super::push(out, 118025);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 118026);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 128810);
									match s {
										[] => true,
										['а', '-', s @ ..] => {
											super::push(out, 135195);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 138859);
													match s {
														[] => true,
														['ц', s @ ..] => super::t49::n142377(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 100888);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гоистка.
pub(crate) fn n48703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48206);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 76804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пистка.
pub(crate) fn n48704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48207);
	s.is_empty()
}

/// Words ending -ристка.
pub(crate) fn n48705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48208);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 76805);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 100889);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118027);
							match s {
								[] => true,
								['м', '-', s @ ..] => {
									super::push(out, 128811);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 135196);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 138860);
													match s {
														[] => true,
														['в', s @ ..] => super::t49::n142378(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -систка.
pub(crate) fn n48706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48209);
	s.is_empty()
}

/// Words ending -тистка.
pub(crate) fn n48707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48210);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 76806);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 100890);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118028);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 128812);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 135197);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 138861);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фистка.
pub(crate) fn n48708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48211);
	s.is_empty()
}

/// Words ending -цистка.
pub(crate) fn n48709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48212);
	s.is_empty()
}

/// Words ending -чистка.
pub(crate) fn n48710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48213);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 76807);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 76808);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 76809);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 76810);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 76811);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 100891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шистка.
pub(crate) fn n48711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48214);
	s.is_empty()
}

/// Words ending -остка.
pub(crate) fn n22137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22010);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 48215);
			match s {
				[] => true,
				['х', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100892);
							s.is_empty()
						}
						['и', 'т', 'я', s @ ..] => {
							super::push(out, 100893);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118030);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128813);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 135198);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140297(s, out),
														_ => false,
													}
												}
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
							super::push(out, 100894);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 100895);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118031);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128814);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135199);
													s.is_empty()
												}
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
							super::push(out, 100896);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118032);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128815);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135200);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140298(s, out),
														_ => false,
													}
												}
												['ы', s @ ..] => {
													super::push(out, 135201);
													match s {
														[] => true,
														['т', 'е', 'ч', '\u{1}', s @ ..] => super::t49::n140299(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48216);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 48217);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76813);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100897);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118033);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 76814);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100898);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118034);
									s.is_empty()
								}
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
			super::push(out, 48218);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76815);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рстка.
pub(crate) fn n22138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22011);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48219);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76816);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 100899);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118035);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128816);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135202);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 76817);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100900);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118036);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128817);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 100901);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118037);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128818);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135203);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140300(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48220);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 76818);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -устка.
pub(crate) fn n22139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22012);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 48221);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76819);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 76820);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 100902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 48222);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76821);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ястка.
pub(crate) fn n22140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22013);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 48223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 76822);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -платье-пачка.
pub(crate) fn n140301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138866);
	s.is_empty()
}

/// Words ending -симпатеечка.
pub(crate) fn n136612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135206);
	s.is_empty()
}

/// The word бедняжечка.
pub(crate) fn n136613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135207);
	s.is_empty()
}

/// Words ending -ролечка.
pub(crate) fn n77747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76932);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 100955);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рлечка.
pub(crate) fn n48867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48367);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 76933);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 100956);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улечка.
pub(crate) fn n48868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48368);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 76934);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 100957);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 118058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 76935);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 100958);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 118059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 100959);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118060);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 128826);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 135208);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 76936);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 100960);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 118061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 100961);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 76937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 100962);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 100963);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 118063);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128827);
									s.is_empty()
								}
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
			super::push(out, 76938);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100964);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 76939);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 100965);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118065);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128828);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135209);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 138867);
													s.is_empty()
												}
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
							super::push(out, 118066);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 128829);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135210);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 138868);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юлечка.
pub(crate) fn n48869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48369);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 76940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нечка.
pub(crate) fn n22233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22103);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48370);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76941);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 76942);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100966);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 76943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48371);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76944);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100967);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118068);
									s.is_empty()
								}
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
			super::push(out, 48372);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 76945);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100968);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118069);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 128830);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48373);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76946);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 48374);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оечка.
pub(crate) fn n22234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22104);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 48375);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 76948);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 100970);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118070);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128831);
											s.is_empty()
										}
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
					super::push(out, 76949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48376);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -печка.
pub(crate) fn n22235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22105);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 48377);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76950);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 100971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48378);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 76951);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48379);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76952);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48380);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76953);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 48381);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76954);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -речка.
pub(crate) fn n22236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22106);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 48382);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76955);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 48383);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48384);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48385);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 76957);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 100977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сечка.
pub(crate) fn n22237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22107);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48386);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 76958);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 76959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 48387);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 48388);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48389);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 48390);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76960);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 76961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48391);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 48392);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 76962);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 100979);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118071);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128832);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48393);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 48394);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 76963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48395);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 76964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 48396);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -течка.
pub(crate) fn n22238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22108);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 48397);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48398);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48399);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 76967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100982);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48400);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 76968);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100983);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118072);
									s.is_empty()
								}
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
			super::push(out, 48401);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 76969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48402);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чечка.
pub(crate) fn n22240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22110);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48403);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76970);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100984);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118073);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128833);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48404);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48405);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 76971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 76972);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 100986);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 118074);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128834);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шечка.
pub(crate) fn n22241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22111);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48406);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76973);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 100987);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 76974);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 100988);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118076);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128835);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 135211);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140304(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 76975);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100989);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118077);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128836);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 100990);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 100991);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118078);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128837);
											s.is_empty()
										}
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
					super::push(out, 76976);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 100992);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 76977);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 76978);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 100993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 76979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48407);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76980);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 100994);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118080);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128838);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48408);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 76981);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 100995);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118081);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128839);
											s.is_empty()
										}
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
					super::push(out, 76982);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100996);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118082);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128840);
											s.is_empty()
										}
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
					super::push(out, 76983);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 76984);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 100997);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118083);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128841);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135212);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 76985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48409);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 76986);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 100998);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 118084);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128842);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135213);
													match s {
														[] => true,
														['м', s @ ..] => super::t49::n140305(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 76987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 100999);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 101000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 76988);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 76989);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48410);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 48411);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 76990);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 76991);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 101003);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118085);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128843);
											s.is_empty()
										}
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
					super::push(out, 76992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101004);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 101005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 76993);
					s.is_empty()
				}
				['л', 'х', s @ ..] => {
					super::push(out, 76994);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101006);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 118086);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128844);
											s.is_empty()
										}
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
					super::push(out, 76995);
					s.is_empty()
				}
				['п', s @ ..] => super::t49::n77811(s, out),
				['р', s @ ..] => super::t49::n77812(s, out),
				['т', s @ ..] => super::t49::n77813(s, out),
				['ч', s @ ..] => super::t49::n77814(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t49::n48912(s, out),
		['ю', s @ ..] => super::t49::n48913(s, out),
		['я', s @ ..] => super::t49::n48914(s, out),
		_ => false,
	}
}

/// Words ending -щечка.
pub(crate) fn n22242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22112);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 48415);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -меньшевичка.
pub(crate) fn n136625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135219);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 138871);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -наркологичка.
pub(crate) fn n136626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135220);
	s.is_empty()
}

/// Words ending -изичка.
pub(crate) fn n48931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48430);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 77021);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -личка.
pub(crate) fn n22250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22120);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48431);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 48432);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48433);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77024);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101026);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 77025);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101027);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118105);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128858);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135221);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48434);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 48435);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 77026);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48436);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77027);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48437);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 77028);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 118106);
									s.is_empty()
								}
								['к', 'л', 'а', '-', s @ ..] => {
									super::push(out, 118107);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128859);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135222);
													match s {
														[] => true,
														['й', s @ ..] => super::t49::n140307(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77029);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77030);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101031);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 77031);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48438);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 48439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 48440);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77033);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мичка.
pub(crate) fn n22251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22121);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48441);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77034);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101033);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118109);
									s.is_empty()
								}
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
			super::push(out, 48442);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 77035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 48443);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77036);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48444);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77037);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'а', s @ ..] => {
					super::push(out, 77038);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101036);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118110);
									s.is_empty()
								}
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
			super::push(out, 48445);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77039);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48446);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 77040);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 48447);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 77041);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 101039);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118111);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128860);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ничка.
pub(crate) fn n22252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22122);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48448);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77042);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101040);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77043);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101041);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118113);
									s.is_empty()
								}
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
			super::push(out, 48449);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77044);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101042);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118114);
									s.is_empty()
								}
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
			super::push(out, 48450);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77045);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101043);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118115);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 128861);
											s.is_empty()
										}
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
					super::push(out, 77046);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 101044);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118116);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 128862);
											s.is_empty()
										}
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
					super::push(out, 77047);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101045);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118117);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77048);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101046);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 77049);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48451);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77050);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101048);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77051);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					super::push(out, 77052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 48452);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48453);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 77054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48454);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48455);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77056);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77057);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101052);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118120);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', 'е', 'т', '\u{1}', s @ ..] => {
			super::push(out, 48456);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 48457);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77058);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101053);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118121);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128864);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101054);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118122);
									s.is_empty()
								}
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
			super::push(out, 48458);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77059);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оичка.
pub(crate) fn n22253(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 48459);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 48460);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77061);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101055);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пичка.
pub(crate) fn n22254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22123);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 48461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ричка.
pub(crate) fn n22255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22124);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 48462);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77062);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 101056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118124);
									s.is_empty()
								}
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
			super::push(out, 48463);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77063);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101057);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118125);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128865);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48464);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 77064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48465);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 77065);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101058);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 77066);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 101059);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'т', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77067);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 77068);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48466);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77069);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101061);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 118128);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128866);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135223);
													s.is_empty()
												}
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
							super::push(out, 101062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 77070);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77071);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77072);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101065);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118130);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128867);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 128868);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 118131);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128869);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 101066);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сичка.
pub(crate) fn n22256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22125);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 48467);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 77073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48468);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77074);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101067);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118133);
									s.is_empty()
								}
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
			super::push(out, 48469);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48470);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77076);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тичка.
pub(crate) fn n22257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22126);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48471);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48472);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77078);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'л', 'а', 'н', s @ ..] => {
			super::push(out, 48473);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48474);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 48475);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 48476);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77080);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101070);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118134);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128870);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48477);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77081);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 101071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 77082);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 77083);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101073);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118135);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128871);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фичка.
pub(crate) fn n22258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22127);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48478);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77084);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48479);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77085);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101075);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77086);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101076);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118137);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128872);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 135224);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word психичка.
pub(crate) fn n22259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22128);
	s.is_empty()
}

/// Words ending -чичка.
pub(crate) fn n22260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22129);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 48480);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шичка.
pub(crate) fn n22261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22130);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 48481);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77088);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щичка.
pub(crate) fn n22262(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48482);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 48483);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 77090);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ама-бабочка.
pub(crate) fn n136631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135225);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 138873);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 140935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нна-бабочка.
pub(crate) fn n136632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135226);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 138874);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 140936);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 142142);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142819);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тук-бабочка.
pub(crate) fn n136633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135227);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138875);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140937);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142143);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 142820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -н-белобочка.
pub(crate) fn n136634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135228);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 138876);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 140938);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 142144);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 142821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оробочка.
pub(crate) fn n102169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101084);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 118142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тобочка.
pub(crate) fn n77917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77101);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 101085);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 118143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 128878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбочка.
pub(crate) fn n48995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48493);
	s.is_empty()
}

/// Words ending -убочка.
pub(crate) fn n48996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48494);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 77102);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101086);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 118144);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 118145);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 128880);
									s.is_empty()
								}
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
			super::push(out, 77103);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 101087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбочка.
pub(crate) fn n48997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48495);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 77104);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 101088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 77105);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юбочка.
pub(crate) fn n48998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48496);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 77106);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101090);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вочка.
pub(crate) fn n22267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22135);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48497);
			match s {
				[] => true,
				['л', 'с', s @ ..] => {
					super::push(out, 77107);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101091);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118146);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 128881);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135229);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77109);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77110);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101093);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 118147);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128882);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135230);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48498);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77111);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 101094);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 118148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 101095);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118149);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128883);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 101096);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101097);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 101098);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77112);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48499);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 48500);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 48501);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48502);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77115);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 77116);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101101);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118153);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128884);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135231);
													s.is_empty()
												}
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
							super::push(out, 101102);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118154);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128885);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 101103);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77117);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 77118);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 101104);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 118156);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128886);
											s.is_empty()
										}
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
					super::push(out, 77119);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101105);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118157);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 101106);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118158);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 128887);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135232);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140312(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77120);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101107);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 101108);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77121);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101109);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118160);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128888);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135233);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48503);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77123);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101110);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118161);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128889);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48504);
			match s {
				[] => true,
				['з', s @ ..] => super::t49::n77940(s, out),
				['и', s @ ..] => {
					super::push(out, 77125);
					match s {
						[] => true,
						['п', s @ ..] => super::t49::n102197(s, out),
						_ => false,
					}
				}
				['л', 'а', 'м', '\u{1}', s @ ..] => super::t49::n77942(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дочка.
pub(crate) fn n22268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22136);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48505);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48506);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77127);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77128);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101114);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118163);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128890);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48507);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48508);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 48509);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 48510);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77129);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101115);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118164);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128891);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135234);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140313(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77130);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зочка.
pub(crate) fn n22269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22137);
	match s {
		[] => true,
		['а', 'л', s @ ..] => {
			super::push(out, 48511);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77131);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101116);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118165);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128892);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135235);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48512);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 48513);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кочка.
pub(crate) fn n22270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22138);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 48514);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77132);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101117);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 77133);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лочка.
pub(crate) fn n22271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22139);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48515);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77134);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 77135);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 77136);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'ц', s @ ..] => {
					super::push(out, 77137);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101120);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118167);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 128893);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135236);
													match s {
														[] => true,
														['р', s @ ..] => super::t49::n140314(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77138);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 77139);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 77140);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101121);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118168);
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
						['и', s @ ..] => {
							super::push(out, 101122);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 101123);
							match s {
								[] => true,
								['е', 'м', '\u{1}', s @ ..] => {
									super::push(out, 118169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 77141);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101124);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118170);
									match s {
										[] => true,
										['ы', 'в', '-', s @ ..] => {
											super::push(out, 128894);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135237);
													match s {
														[] => true,
														['к', s @ ..] => super::t49::n140315(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48516);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77142);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48517);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 77143);
					s.is_empty()
				}
				['д', 'и', s @ ..] => {
					super::push(out, 77144);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101126);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77145);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101127);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118172);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128895);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135238);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77146);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101128);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 101129);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 101130);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118173);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128896);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135239);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', 'п', s @ ..] => {
					super::push(out, 77148);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 77149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48518);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 77150);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 77151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'а', 'с', 'к', 'е', 'с', '\u{1}', s @ ..] => {
					super::push(out, 77152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48519);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 48520);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77153);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 77154);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101133);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t49::n49023(s, out),
		['у', s @ ..] => {
			super::push(out, 48522);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 77161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 48523);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77162);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101146);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118189);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128907);
											s.is_empty()
										}
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
					super::push(out, 77163);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77164);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мочка.
pub(crate) fn n22272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22140);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48524);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77165);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 77166);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77167);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 77168);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 77169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 48525);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77170);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101148);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118190);
									s.is_empty()
								}
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
			super::push(out, 48526);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77171);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 77172);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77173);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101150);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118191);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128908);
											s.is_empty()
										}
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
					super::push(out, 77174);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 77175);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101151);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118192);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128909);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135245);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77176);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101152);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 77177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48528);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77178);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48529);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101154);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 101155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118194);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48530);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48531);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48532);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 77182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 48533);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77183);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101157);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118195);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128910);
											s.is_empty()
										}
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
					super::push(out, 77184);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 48534);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 48535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ночка.
pub(crate) fn n22273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22141);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48536);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77186);
					s.is_empty()
				}
				['г', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101159);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118196);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'ц', '\u{1}', s @ ..] => {
							super::push(out, 101160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77187);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 101161);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118197);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128911);
											s.is_empty()
										}
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
					super::push(out, 77188);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101162);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 101163);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118199);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128912);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 101164);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118200);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128913);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101165);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118201);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 101166);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 77189);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77190);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101168);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118203);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128914);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 101169);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77191);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77192);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77193);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101172);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118205);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 118206);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128915);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135246);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77194);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101174);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 101175);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 77195);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101176);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118209);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128916);
											s.is_empty()
										}
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
					super::push(out, 77196);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101177);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118210);
									s.is_empty()
								}
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
			super::push(out, 48537);
			match s {
				[] => true,
				['б', 'а', s @ ..] => {
					super::push(out, 77197);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118211);
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
							super::push(out, 101179);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 101180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77198);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101181);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118212);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128917);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135247);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n140318(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77199);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101182);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118213);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 128918);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 135248);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t49::n49040(s, out),
		['н', 'а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77209);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 77210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t49::n102276(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t49::n49042(s, out),
		['у', s @ ..] => super::t49::n49043(s, out),
		['ы', s @ ..] => super::t49::n49044(s, out),
		['я', s @ ..] => super::t49::n49045(s, out),
		_ => false,
	}
}

/// Words ending -почка.
pub(crate) fn n22274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22142);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48543);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77227);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 77228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48544);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77229);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 77230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101216);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 101217);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118235);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128935);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['а', s @ ..] => {
							super::push(out, 101218);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118236);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 101219);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118237);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128936);
											s.is_empty()
										}
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
					super::push(out, 77231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48545);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 48546);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48547);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77232);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48548);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 77234);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77235);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 48549);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 77236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 48550);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77237);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101223);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118238);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128937);
											s.is_empty()
										}
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
					super::push(out, 77238);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рочка.
pub(crate) fn n22275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22143);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48551);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77239);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 77240);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77241);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101225);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118239);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 128938);
											s.is_empty()
										}
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
					super::push(out, 77242);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 77243);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77244);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101226);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118240);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 101227);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118241);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128939);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135257);
													s.is_empty()
												}
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
							super::push(out, 101228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 77245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 48552);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77246);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101229);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118242);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128940);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135258);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48553);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77247);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 101230);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 118243);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 128941);
											s.is_empty()
										}
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
					super::push(out, 77248);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118244);
									s.is_empty()
								}
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
			super::push(out, 48554);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77249);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 77250);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77251);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77252);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 77253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101233);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 101234);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118245);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 101235);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118246);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128942);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48555);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48556);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77255);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 77256);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 77257);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101236);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77258);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 77259);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118248);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128943);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 118249);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101237);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118250);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77260);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 77261);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101238);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 101239);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118251);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 128944);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135259);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140323(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77262);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101240);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118252);
									s.is_empty()
								}
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
			super::push(out, 48557);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77263);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101241);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77264);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77265);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t49::n49061(s, out),
		['у', s @ ..] => super::t49::n49062(s, out),
		['ы', s @ ..] => super::t49::n49063(s, out),
		['ю', s @ ..] => super::t49::n49064(s, out),
		['я', s @ ..] => super::t49::n49065(s, out),
		_ => false,
	}
}

/// Words ending -сочка.
pub(crate) fn n22276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22144);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48563);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 48564);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77278);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101255);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118259);
									s.is_empty()
								}
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
			super::push(out, 48565);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 48566);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77280);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 77281);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101257);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118260);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 128947);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135260);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48567);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77282);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 77283);
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					super::push(out, 77284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 48568);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77285);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -точка.
pub(crate) fn n22277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22145);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 48569);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77286);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101258);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118261);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128948);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 101259);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 118262);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128949);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48570);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77287);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 77288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 77289);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 101261);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77290);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101262);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118264);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128950);
											s.is_empty()
										}
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
					super::push(out, 77291);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 77292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77293);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118265);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 118266);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 128951);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135261);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77294);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101265);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 77295);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 77296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 48571);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 48572);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77298);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118268);
									s.is_empty()
								}
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
			super::push(out, 48573);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77299);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118269);
									s.is_empty()
								}
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
			super::push(out, 48574);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77300);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101270);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 77302);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101272);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118271);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128952);
											s.is_empty()
										}
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
					super::push(out, 77303);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 77304);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101273);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118272);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 128953);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135262);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77305);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 77306);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 101274);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118273);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128954);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135263);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77307);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101275);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118274);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128955);
											s.is_empty()
										}
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
					super::push(out, 77308);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77309);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101276);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118275);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128956);
											s.is_empty()
										}
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
					super::push(out, 77310);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101277);
							s.is_empty()
						}
						['м', s @ ..] => super::t49::n102365(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 77311);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 77312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t49::n49078(s, out),
		['н', s @ ..] => super::t49::n49079(s, out),
		['о', s @ ..] => super::t49::n49080(s, out),
		['р', s @ ..] => super::t49::n49081(s, out),
		['с', s @ ..] => super::t49::n49082(s, out),
		['т', s @ ..] => super::t49::n49083(s, out),
		['у', s @ ..] => super::t49::n49084(s, out),
		['ф', s @ ..] => super::t49::n49085(s, out),
		['х', s @ ..] => {
			super::push(out, 48582);
			s.is_empty()
		}
		['ы', s @ ..] => super::t49::n49087(s, out),
		['ю', s @ ..] => super::t49::n49088(s, out),
		['я', s @ ..] => super::t49::n49089(s, out),
		_ => false,
	}
}

/// Words ending -фочка.
pub(crate) fn n22278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22146);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 48586);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 77341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -як-самоучка.
pub(crate) fn n136677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135271);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 138893);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 140947);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -авторучка.
pub(crate) fn n119566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118304);
	s.is_empty()
}

/// Words ending -хоручка.
pub(crate) fn n78187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77362);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 101327);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 118305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уручка.
pub(crate) fn n49116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48612);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 77363);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыручка.
pub(crate) fn n49117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48613);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 77364);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101329);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сучка.
pub(crate) fn n22292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22160);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48614);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 48615);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77365);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 48616);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77366);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тучка.
pub(crate) fn n22293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22161);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48617);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 48618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хучка.
pub(crate) fn n22294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22162);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чучка.
pub(crate) fn n22295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22163);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 48620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щучка.
pub(crate) fn n22296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22164);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48621);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыучка.
pub(crate) fn n22297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22165);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 48622);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мба-вонючка.
pub(crate) fn n136678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135272);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138894);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 140948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тье-рубашка.
pub(crate) fn n136679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135273);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138895);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140949);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 142150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мальчугашка.
pub(crate) fn n136680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135274);
	s.is_empty()
}

/// Words ending -икашка.
pub(crate) fn n49185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48681);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 77410);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 101354);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 118318);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 128975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 101355);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 118319);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 128976);
									s.is_empty()
								}
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
			super::push(out, 77411);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 101356);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 118320);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 128977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкашка.
pub(crate) fn n49186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48682);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77412);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 77413);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 101357);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окашка.
pub(crate) fn n49187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48683);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 77414);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 101358);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -букашка.
pub(crate) fn n78240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77415);
	s.is_empty()
}

/// Words ending -лашка.
pub(crate) fn n22327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22195);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48685);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 77417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48686);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77418);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101360);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118322);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 128978);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48687);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77419);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 77420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48688);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77421);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 101362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 48689);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77422);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -машка.
pub(crate) fn n22328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22196);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48690);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 48691);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77423);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101363);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 77424);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 77425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48692);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77426);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77427);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101365);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118324);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 128979);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48693);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48694);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нашка.
pub(crate) fn n22329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22197);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48695);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77430);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 48696);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77432);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118325);
									s.is_empty()
								}
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
			super::push(out, 48697);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77433);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101368);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118326);
									s.is_empty()
								}
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
			super::push(out, 48698);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77434);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101369);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118327);
									s.is_empty()
								}
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
			super::push(out, 48699);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77435);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 101370);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118328);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128980);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135275);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140331(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48700);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 77436);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 48701);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77437);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пашка.
pub(crate) fn n22330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22198);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48702);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 77438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48703);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77439);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101374);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118329);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 118330);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 128981);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135276);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140332(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48704);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48705);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48706);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48707);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77442);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101375);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118331);
									s.is_empty()
								}
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
			super::push(out, 48708);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77443);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101376);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рашка.
pub(crate) fn n22331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22199);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48709);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 48710);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77444);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101377);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77445);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101378);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77446);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77447);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 77448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 48711);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77449);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101379);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118334);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 128982);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48712);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77450);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101380);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77451);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101381);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118336);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 128983);
											s.is_empty()
										}
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
					super::push(out, 77452);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101382);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118337);
									s.is_empty()
								}
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
			super::push(out, 48713);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48714);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77454);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101383);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118338);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77455);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48715);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77456);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101385);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118339);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128984);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 135277);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48716);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 77457);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101386);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118340);
									s.is_empty()
								}
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
			super::push(out, 48717);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77458);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77459);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 77460);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сашка.
pub(crate) fn n22332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22200);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ташка.
pub(crate) fn n22333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22201);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48719);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48720);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77462);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48721);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77463);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101389);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118341);
									s.is_empty()
								}
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
			super::push(out, 48722);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 48723);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77464);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 101390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чашка.
pub(crate) fn n22334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22202);
	s.is_empty()
}

/// Words ending -шашка.
pub(crate) fn n22335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22203);
	s.is_empty()
}

/// Words ending -скороебишка.
pub(crate) fn n136686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135280);
	s.is_empty()
}

/// Words ending -охвальбишка.
pub(crate) fn n136687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135281);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 138898);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -четоводишка.
pub(crate) fn n136688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135282);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зишка.
pub(crate) fn n22356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22224);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 48764);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 77499);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101415);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118355);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 128992);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 135283);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48765);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77500);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 77501);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лишка.
pub(crate) fn n22358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22226);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48767);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77502);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101418);
							match s {
								[] => true,
								['н', 'е', 'г', '\u{1}', s @ ..] => {
									super::push(out, 118356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 77503);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101419);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 118357);
									s.is_empty()
								}
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
			super::push(out, 48768);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77504);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101420);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118358);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 128993);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135284);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48769);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77505);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101421);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 77506);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101422);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 118360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77507);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101423);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118361);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 128994);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135285);
													s.is_empty()
												}
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
							super::push(out, 101424);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 118362);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 128995);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48770);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 77508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 48771);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77509);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 101425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48772);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77510);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101426);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118363);
									s.is_empty()
								}
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
			super::push(out, 48773);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 77511);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нишка.
pub(crate) fn n22360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22228);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48774);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 48775);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 48776);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77513);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101429);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118364);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 128996);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48777);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77514);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101430);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 118365);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 128997);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135286);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48778);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77515);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101431);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 118366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77516);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 77517);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101432);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 118367);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 128998);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135287);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77518);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48780);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77519);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77520);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101435);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 118368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77521);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101436);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118369);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 101437);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118370);
									s.is_empty()
								}
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
			super::push(out, 48781);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 77522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пишка.
pub(crate) fn n22361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22229);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 48782);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48783);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 77524);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ришка.
pub(crate) fn n22362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22230);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48784);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77525);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 77526);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77527);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101438);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118371);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 128999);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135288);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n140335(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48785);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48786);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77528);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101439);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 118372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 77529);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101440);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 118373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77530);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101441);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 101442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 77531);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101443);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 118374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 77532);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101444);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 118375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 77533);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101445);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118376);
									s.is_empty()
								}
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
			super::push(out, 48787);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77534);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77535);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 101446);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118377);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129000);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 135289);
													match s {
														[] => true,
														['е', s @ ..] => super::t49::n140336(s, out),
														_ => false,
													}
												}
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
							super::push(out, 101447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48788);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 77536);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101448);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118378);
									s.is_empty()
								}
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
			super::push(out, 48789);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сишка.
pub(crate) fn n22363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22231);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48790);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77537);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101449);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118379);
									s.is_empty()
								}
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
			super::push(out, 48791);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 77538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48792);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77539);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48793);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 48794);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77540);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тишка.
pub(crate) fn n22364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22232);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48795);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77541);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101452);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118380);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 129001);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135290);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77542);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101453);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118381);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129002);
											s.is_empty()
										}
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
					super::push(out, 77543);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101454);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 101455);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118382);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 129003);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 129004);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48796);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 77544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 48797);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77545);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101456);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118383);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 129005);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 135291);
													s.is_empty()
												}
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
							super::push(out, 101457);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118384);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129006);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135292);
													match s {
														[] => true,
														['э', s @ ..] => super::t49::n140337(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77546);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101458);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 101459);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 118386);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 129007);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48798);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77547);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101460);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 77550);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48799);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77551);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101464);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 101465);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118389);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129008);
											s.is_empty()
										}
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
					super::push(out, 77552);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101466);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 118390);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 129009);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135293);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140338(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77553);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101467);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118391);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 129010);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 101468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48800);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77554);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 48801);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77555);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 48802);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77556);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уишка.
pub(crate) fn n22365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22233);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 48803);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77557);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чишка.
pub(crate) fn n22367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22235);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48805);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77558);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 101473);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118392);
									s.is_empty()
								}
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
			super::push(out, 48806);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48807);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77560);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101474);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118393);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129011);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 135294);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48808);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 48809);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 48810);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 48811);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77561);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101475);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118394);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129012);
											s.is_empty()
										}
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
					super::push(out, 77562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48812);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 48813);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 48814);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77563);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 48815);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77564);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101477);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118395);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129013);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шишка.
pub(crate) fn n22368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22236);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 48816);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77565);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48817);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 48818);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 77567);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101479);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьишка.
pub(crate) fn n22369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22237);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 48819);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77568);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101480);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118397);
									s.is_empty()
								}
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
			super::push(out, 48820);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77569);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101481);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 118398);
									s.is_empty()
								}
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
			super::push(out, 48821);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77570);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48822);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77571);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101483);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -акула-кошка.
pub(crate) fn n136701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135295);
	s.is_empty()
}

/// Words ending -листоблошка.
pub(crate) fn n136702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135296);
	s.is_empty()
}

/// Words ending -па-крошка.
pub(crate) fn n119667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118405);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 129017);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 135297);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 138904);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 140953);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 142153);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -к-крошка.
pub(crate) fn n102581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101494);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 118406);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 129018);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 135298);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 138905);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 140954);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орошка.
pub(crate) fn n49348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48843);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 77591);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 77592);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 77593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 101495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тошка.
pub(crate) fn n22381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22249);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48845);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 48846);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 77595);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 48847);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48848);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48849);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77598);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -расслабушка.
pub(crate) fn n136705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135299);
	s.is_empty()
}

/// Words ending -вушка.
pub(crate) fn n22383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22251);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48859);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77615);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101513);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77616);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77617);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77618);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101516);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 101517);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 101518);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 101519);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 118418);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129023);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 118419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77619);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101520);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118420);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129024);
											s.is_empty()
										}
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
					super::push(out, 77620);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101521);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 101522);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118421);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129025);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135300);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48860);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 48861);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77621);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101523);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118422);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129026);
											s.is_empty()
										}
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
					super::push(out, 77622);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101524);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77623);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101525);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 118424);
									s.is_empty()
								}
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
			super::push(out, 48862);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77624);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48863);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77625);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 77626);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101527);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118425);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129027);
											s.is_empty()
										}
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
					super::push(out, 77627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48864);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77628);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101529);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 118426);
									s.is_empty()
								}
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
			super::push(out, 48865);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77629);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101530);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 77630);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101531);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118428);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129028);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135301);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77631);
					match s {
						[] => true,
						['а', s @ ..] => super::t49::n102619(s, out),
						['в', s @ ..] => super::t49::n102620(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t49::n78457(s, out),
				['л', s @ ..] => super::t49::n78458(s, out),
				['п', s @ ..] => super::t49::n78459(s, out),
				['р', s @ ..] => super::t49::n78460(s, out),
				['с', s @ ..] => super::t49::n78461(s, out),
				['х', s @ ..] => super::t49::n78462(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ингушка.
pub(crate) fn n78466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77641);
	s.is_empty()
}

/// Words ending -ргушка.
pub(crate) fn n49374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48869);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 77642);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 101549);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118449);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 129041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгушка.
pub(crate) fn n49375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48870);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 77643);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 101550);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягушка.
pub(crate) fn n49376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48871);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 77644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -душка.
pub(crate) fn n22385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22253);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48872);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 48873);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77645);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 77646);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101551);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 101552);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118451);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129042);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135308);
													match s {
														[] => true,
														['-', s @ ..] => super::t49::n140345(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 129043);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101553);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77647);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 77648);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101554);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118453);
									s.is_empty()
								}
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
			super::push(out, 48874);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77649);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101555);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 77650);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101556);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77651);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101557);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118456);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129044);
											s.is_empty()
										}
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
					super::push(out, 77652);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101558);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118457);
									s.is_empty()
								}
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
			super::push(out, 48875);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77653);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 48876);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77654);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 77655);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 77656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48877);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77657);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 48878);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48879);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77660);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101561);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118458);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129045);
											s.is_empty()
										}
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
					super::push(out, 77661);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101562);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 77662);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 101563);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118460);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129046);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101564);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77663);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101565);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101566);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 101567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77664);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101568);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77665);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101569);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118465);
									s.is_empty()
								}
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
			super::push(out, 48880);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77666);
					match s {
						[] => true,
						['п', s @ ..] => super::t49::n102657(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48881);
			match s {
				[] => true,
				['о', s @ ..] => super::t49::n78492(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t49::n49387(s, out),
		['я', s @ ..] => super::t49::n49388(s, out),
		_ => false,
	}
}

/// Words ending -зушка.
pub(crate) fn n22386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22254);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48884);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 77671);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101573);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кушка.
pub(crate) fn n22387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22255);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48885);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77672);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101574);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 118470);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129048);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135310);
													match s {
														[] => true,
														['ш', s @ ..] => super::t49::n140347(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77673);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101575);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118471);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129049);
											s.is_empty()
										}
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
					super::push(out, 77674);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101576);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118472);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129050);
											s.is_empty()
										}
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
					super::push(out, 77675);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 77676);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 101577);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 118473);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129051);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135311);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140348(s, out),
														_ => false,
													}
												}
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
							super::push(out, 101578);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118474);
									s.is_empty()
								}
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
			super::push(out, 48886);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 77677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48887);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 77678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 48888);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77679);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48889);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77680);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48890);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77681);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101581);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118475);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129052);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48891);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 48892);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77683);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101582);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118476);
									s.is_empty()
								}
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
			super::push(out, 48893);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77684);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101583);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118477);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129053);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лушка.
pub(crate) fn n22388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22256);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48894);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 77685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 48895);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77686);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 48896);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77687);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 101585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48897);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77688);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 101586);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118478);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129054);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 118479);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129055);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135312);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77689);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48898);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 77691);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77692);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 48899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48900);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 77694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48901);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48902);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77697);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48903);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 48904);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77699);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 101593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мушка.
pub(crate) fn n22389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22257);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48905);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 48906);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 77700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48907);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77701);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 77702);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101596);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118480);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 129056);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48908);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77703);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101597);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 77704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77705);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 101599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48909);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48910);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77707);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48911);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 48912);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 48913);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77710);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101603);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77711);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

