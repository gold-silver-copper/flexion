use flexion::Path;

/// Words ending -тер.
pub(crate) fn n3345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3334);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14895);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37179);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65239);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 37180);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65240);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 65241);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 37181);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 37182);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 65242);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91581);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111595);
											s.is_empty()
										}
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
					super::push(out, 37183);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65243);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91582);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111596);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 65244);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91583);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111597);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124587);
													match s {
														[] => true,
														['м', s @ ..] => super::t37::n133798(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37184);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65245);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 65246);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91584);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 111598);
											s.is_empty()
										}
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
					super::push(out, 37185);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65247);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 91585);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111599);
											s.is_empty()
										}
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
					super::push(out, 37186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37187);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65249);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 65250);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91586);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111600);
											s.is_empty()
										}
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
					super::push(out, 37188);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65251);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91587);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111601);
											s.is_empty()
										}
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
					super::push(out, 37189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14896);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37191);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65253);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37192);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 65254);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 91589);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111602);
											s.is_empty()
										}
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
					super::push(out, 37193);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65255);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 91590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 65256);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37194);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65257);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 65258);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91592);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111603);
											s.is_empty()
										}
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
					super::push(out, 37195);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65259);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 37196);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65260);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t37::n14975(s, out),
		['й', s @ ..] => super::t37::n14976(s, out),
		['к', s @ ..] => super::t37::n14977(s, out),
		['л', s @ ..] => {
			super::push(out, 14900);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37211);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 65286);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 91618);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 111618);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t37::n37562(s, out),
				['о', s @ ..] => super::t37::n37563(s, out),
				['э', s @ ..] => super::t37::n37564(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t37::n14979(s, out),
		['о', s @ ..] => super::t37::n14980(s, out),
		['п', s @ ..] => super::t37::n14981(s, out),
		['р', s @ ..] => super::t37::n14982(s, out),
		['с', s @ ..] => super::t38::n14983(s, out),
		['т', s @ ..] => super::t38::n14984(s, out),
		['у', s @ ..] => super::t38::n14985(s, out),
		['ф', s @ ..] => super::t38::n14986(s, out),
		['х', s @ ..] => super::t38::n14987(s, out),
		['ы', s @ ..] => super::t38::n14988(s, out),
		['ь', s @ ..] => super::t38::n14989(s, out),
		['э', s @ ..] => super::t38::n14990(s, out),
		['ю', s @ ..] => super::t38::n14991(s, out),
		['я', s @ ..] => super::t38::n14992(s, out),
		_ => false,
	}
}

/// Words ending -уер.
pub(crate) fn n3346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3335);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14915);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37276);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65424);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91741);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111706);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124653);
													match s {
														[] => true,
														['б', s @ ..] => super::t38::n133841(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 14917);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 37278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фер.
pub(crate) fn n3347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3336);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14919);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37279);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37280);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 65425);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 65426);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 91742);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111707);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 124654);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 37282);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 37283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14920);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 37284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 14921);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37285);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65430);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91743);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111708);
											s.is_empty()
										}
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
					super::push(out, 37286);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 65431);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 65432);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91744);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111709);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124655);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37287);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65433);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91745);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111710);
											s.is_empty()
										}
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
					super::push(out, 37288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 37289);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 65435);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91746);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111711);
											s.is_empty()
										}
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
					super::push(out, 37290);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65436);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 91747);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111712);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14922);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37291);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65437);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91748);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111713);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 124656);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37292);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65438);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 91749);
									s.is_empty()
								}
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
			super::push(out, 14923);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37293);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 37294);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65440);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 65441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14924);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37295);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 37296);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 37297);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65442);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91750);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111714);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124657);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37298);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65443);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91751);
									s.is_empty()
								}
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
			super::push(out, 14925);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37299);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65444);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37300);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65445);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91753);
									s.is_empty()
								}
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
			super::push(out, 14926);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37301);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t38::n37653(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t38::n15005(s, out),
		['у', s @ ..] => super::t38::n15006(s, out),
		['ф', s @ ..] => super::t38::n15007(s, out),
		['ь', s @ ..] => {
			super::push(out, 14930);
			match s {
				[] => true,
				['л', s @ ..] => super::t38::n37663(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хер.
pub(crate) fn n3348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3337);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14931);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 14932);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37313);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 65460);
							match s {
								[] => true,
								['р', 'е', 'х', 'а', 'ш', '\u{1}', s @ ..] => {
									super::push(out, 91763);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 65461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14933);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37314);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65462);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91764);
									s.is_empty()
								}
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
			super::push(out, 14934);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37315);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 37316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14935);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14936);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 37318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14937);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37319);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цер.
pub(crate) fn n3349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3338);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 14938);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 37320);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65466);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 91765);
									s.is_empty()
								}
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
			super::push(out, 14939);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37321);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65467);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91766);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111721);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124661);
													match s {
														[] => true,
														['з', s @ ..] => super::t38::n133845(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 14940);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37322);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65468);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 65469);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 91767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 37323);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65470);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91768);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111722);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 124662);
													match s {
														[] => true,
														['с', s @ ..] => super::t38::n133846(s, out),
														_ => false,
													}
												}
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
							super::push(out, 65471);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91769);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111723);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14941);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37324);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65472);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 65473);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91770);
									s.is_empty()
								}
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
			super::push(out, 14942);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37325);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 65474);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 91771);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111724);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14943);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37326);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65475);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 91772);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111725);
											s.is_empty()
										}
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
					super::push(out, 37327);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 65476);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91773);
									s.is_empty()
								}
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
			super::push(out, 14944);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37328);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чер.
pub(crate) fn n3350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3339);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14945);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37329);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14946);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37330);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65479);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91774);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111726);
											s.is_empty()
										}
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
					super::push(out, 37331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65480);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91775);
									s.is_empty()
								}
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
			super::push(out, 14947);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37332);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 37333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 37334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14948);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37335);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65483);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37336);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65484);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91777);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111727);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 65485);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 91778);
									s.is_empty()
								}
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
			super::push(out, 14949);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37337);
					match s {
						[] => true,
						['д', 'р', 'е', 'б', '\u{1}', s @ ..] => {
							super::push(out, 65486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 37338);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37339);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 14950);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37340);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65489);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 65490);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 91779);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', 'с', 'и', 'д', 'о', s @ ..] => {
							super::push(out, 65491);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91780);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111728);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124663);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37341);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65492);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91781);
									s.is_empty()
								}
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
			super::push(out, 14951);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37342);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37343);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65494);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 65495);
							match s {
								[] => true,
								['б', 'й', 'е', 'л', '\u{1}', s @ ..] => {
									super::push(out, 91782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37344);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65496);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шер.
pub(crate) fn n3351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3340);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14952);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37345);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65497);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91784);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111729);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14953);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 37346);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65498);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91785);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 111730);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14954);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37347);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37348);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65500);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 65501);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91786);
									s.is_empty()
								}
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
			super::push(out, 14955);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37349);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65502);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91787);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111731);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14956);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37350);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 37351);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 37352);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65504);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 91788);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111732);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 111733);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124664);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n133847(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 14957);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37354);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 14958);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 37355);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65506);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91789);
									s.is_empty()
								}
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
			super::push(out, 14959);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37356);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65507);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14960);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 14961);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37358);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 14962);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37359);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 37360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37361);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65511);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91790);
									s.is_empty()
								}
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
			super::push(out, 14963);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37362);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щер.
pub(crate) fn n3352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3341);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14964);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 37363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 14965);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 14966);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьер.
pub(crate) fn n3353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3342);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 14967);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37366);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65513);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91791);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111734);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14968);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37367);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65514);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91792);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111735);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124665);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37368);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65515);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 65516);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91794);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111736);
											s.is_empty()
										}
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
					super::push(out, 37369);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 65517);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37370);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65518);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 65519);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91796);
									s.is_empty()
								}
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
			super::push(out, 14969);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65520);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91797);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111737);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 124666);
													s.is_empty()
												}
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
							super::push(out, 65521);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91798);
									match s {
										[] => true,
										['-', 'н', 'е', 'ж', '\u{1}', s @ ..] => {
											super::push(out, 111738);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14970);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37371);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 37372);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37373);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65523);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91799);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111739);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124667);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['к', s @ ..] => {
							super::push(out, 65524);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91800);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111740);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 65525);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 65526);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91801);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 111741);
											s.is_empty()
										}
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
					super::push(out, 37374);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65527);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 91802);
									s.is_empty()
								}
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
			super::push(out, 14971);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 14972);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37375);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65528);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 65529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 37376);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65530);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91803);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111742);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 124668);
													s.is_empty()
												}
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
								['-', s @ ..] => {
									super::push(out, 91804);
									s.is_empty()
								}
								['й', s @ ..] => {
									super::push(out, 91805);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 91806);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111743);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124669);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', 'к', s @ ..] => super::t38::n92782(s, out),
								['ь', s @ ..] => {
									super::push(out, 91807);
									match s {
										[] => true,
										['л', s @ ..] => super::t38::n112929(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t38::n37730(s, out),
				_ => false,
			}
		}
		['с', 'а', s @ ..] => super::t38::n15051(s, out),
		['т', s @ ..] => super::t38::n15052(s, out),
		['ф', s @ ..] => super::t38::n15053(s, out),
		_ => false,
	}
}

/// Words ending -юер.
pub(crate) fn n3354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3343);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 14976);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37382);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65536);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 65537);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91811);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111748);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124671);
													match s {
														[] => true,
														['и', s @ ..] => super::t38::n133848(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндецемвир.
pub(crate) fn n125959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124672);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 132487);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 137108);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 139882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дир.
pub(crate) fn n3360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3349);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 15000);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37405);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65553);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91818);
									s.is_empty()
								}
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
			super::push(out, 15001);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65554);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91819);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 65555);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37406);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 37407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 15002);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15003);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37409);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15004);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37410);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65558);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91821);
									s.is_empty()
								}
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
					super::push(out, 37411);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65559);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91822);
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
						['г', s @ ..] => {
							super::push(out, 65560);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 65561);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 37412);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65562);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 91824);
									s.is_empty()
								}
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
			super::push(out, 15005);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15006);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37413);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65563);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жир.
pub(crate) fn n3361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15007);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15008);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 37414);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65564);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91826);
									s.is_empty()
								}
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
			super::push(out, 15009);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37415);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 65565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15010);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37416);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65566);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91827);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111751);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15011);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15012);
			match s {
				[] => true,
				['а', 'р', 'т', s @ ..] => {
					super::push(out, 37418);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15013);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 37420);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65568);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91828);
									s.is_empty()
								}
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
			super::push(out, 15014);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37421);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65569);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91829);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111752);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зир.
pub(crate) fn n3362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3351);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15015);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15016);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37423);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 37424);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65570);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 65571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15017);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37425);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 37426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15018);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 15019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кир.
pub(crate) fn n3363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3352);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15020);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15021);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 15022);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 15023);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15024);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15025);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15026);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37431);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65574);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лир.
pub(crate) fn n3364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3353);
	match s {
		[] => true,
		['е', 'в', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37432);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 37433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37434);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 37435);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65576);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91831);
									s.is_empty()
								}
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
			super::push(out, 15027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мир.
pub(crate) fn n3365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3354);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15028);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 15029);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37436);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65577);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37437);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65578);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91833);
									s.is_empty()
								}
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
			super::push(out, 15030);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37438);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 37439);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 37440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15031);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15032);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37442);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65580);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91834);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111753);
											s.is_empty()
										}
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
					super::push(out, 37443);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 37444);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65582);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91835);
									s.is_empty()
								}
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
			super::push(out, 15033);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37445);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65583);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91836);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111754);
											s.is_empty()
										}
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
					super::push(out, 37446);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65584);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 37447);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37448);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 37449);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65586);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91838);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111755);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15034);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37450);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65587);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 65588);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37451);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65589);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91841);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111756);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 65590);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91842);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111757);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 65591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37452);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65592);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 65593);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 91844);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111758);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15035);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37453);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 65594);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91845);
									s.is_empty()
								}
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
			super::push(out, 15036);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15037);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37455);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 65595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15038);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37456);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65596);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 15039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нир.
pub(crate) fn n3366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3355);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15040);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 15041);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37457);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65597);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 65598);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91847);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 111759);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 124673);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n133850(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 124674);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n133851(s, out),
														_ => false,
													}
												}
												['т', s @ ..] => {
													super::push(out, 124675);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n133852(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15042);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 15043);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37458);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65599);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91848);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111760);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15044);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 37459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пир.
pub(crate) fn n3368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3357);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15045);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15046);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37460);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 37461);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65600);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91849);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111761);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124676);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37463);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 37464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15047);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65601);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 65602);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91850);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 111762);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124677);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n133853(s, out),
														_ => false,
													}
												}
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
							super::push(out, 65603);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91851);
									s.is_empty()
								}
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
			super::push(out, 15048);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 15049);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сир.
pub(crate) fn n3370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3359);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15051);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15052);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37468);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37469);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65604);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91852);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111763);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15053);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37470);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65605);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 91853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 37471);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 65606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15054);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37472);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65607);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 65608);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15055);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37473);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65609);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91855);
									s.is_empty()
								}
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
			super::push(out, 15056);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37474);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65610);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 91856);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111764);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124678);
													match s {
														[] => true,
														['л', s @ ..] => super::t38::n133854(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37475);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37476);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 15057);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37477);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тир.
pub(crate) fn n3371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3360);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15058);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37478);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 37479);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 37480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'к', s @ ..] => {
			super::push(out, 15059);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 37481);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65613);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15060);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37482);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 37483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15061);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15062);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37486);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 65614);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91857);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111765);
											s.is_empty()
										}
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
					super::push(out, 37487);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15063);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37488);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жуир.
pub(crate) fn n15147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15064);
	s.is_empty()
}

/// Words ending -фир.
pub(crate) fn n3373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15065);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15066);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15067);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 37491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65618);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15068);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 37492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15069);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37493);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'о', 'п', '\u{1}', s @ ..] => {
			super::push(out, 15070);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 15071);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 15072);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хир.
pub(crate) fn n3374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3363);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15073);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 37496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15074);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37497);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цир.
pub(crate) fn n3375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3364);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15075);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15076);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37499);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 15077);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37500);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 65621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чир.
pub(crate) fn n3376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3365);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15078);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 15079);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37501);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 65622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шир.
pub(crate) fn n3377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3366);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15080);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15081);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37503);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65623);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 91858);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111766);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124679);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15082);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37504);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65624);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 91859);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 111767);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15083);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37505);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 65625);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 65626);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37506);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65627);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91862);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 111768);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15084);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37507);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65628);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91863);
									s.is_empty()
								}
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
			super::push(out, 15085);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37508);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65629);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 91864);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 111769);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 124680);
													match s {
														[] => true,
														['н', s @ ..] => super::t38::n133855(s, out),
														_ => false,
													}
												}
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
							super::push(out, 65630);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 91865);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 111770);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 124681);
													match s {
														[] => true,
														['ь', s @ ..] => super::t38::n133856(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37509);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65631);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 91866);
									s.is_empty()
								}
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
			super::push(out, 15086);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37510);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65632);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91867);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 111771);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 124682);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37511);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15087);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37512);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65634);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91868);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111772);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 124683);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n133857(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37513);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65635);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 91869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 37514);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65636);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91870);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111773);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15088);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37515);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зинвор.
pub(crate) fn n66326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65650);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 91879);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овор.
pub(crate) fn n15202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15119);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 37538);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 65651);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 65652);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 91880);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111776);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 111777);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 124684);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 132496);
													s.is_empty()
												}
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
							super::push(out, 91881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 65653);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91882);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 111778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 65654);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91883);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 111779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 65655);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 91884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 65656);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 65657);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 65658);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 65659);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 91887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111780);
									s.is_empty()
								}
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
			super::push(out, 37539);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 65660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 37540);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 65661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 91889);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 111781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твор.
pub(crate) fn n15203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15120);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37541);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 65662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 37542);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 65663);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 91890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37543);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 65664);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 91891);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 111782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 65665);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 91892);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 111783);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 124685);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 132497);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 65666);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 91893);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 111784);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 124686);
											s.is_empty()
										}
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
					super::push(out, 65667);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 91894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 65668);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 91895);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 111785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 91896);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 111786);
									s.is_empty()
								}
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
			super::push(out, 37544);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 65669);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 91897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явор.
pub(crate) fn n15204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15121);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 37545);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гор.
pub(crate) fn n3393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3382);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15122);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15123);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37546);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 37547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 37549);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65672);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91898);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 91899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37550);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65673);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91900);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111787);
											s.is_empty()
										}
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
					super::push(out, 37551);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65674);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91901);
									s.is_empty()
								}
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
			super::push(out, 15124);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37552);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 65675);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 91902);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111788);
											s.is_empty()
										}
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
					super::push(out, 37553);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 65676);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 91903);
									s.is_empty()
								}
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
			super::push(out, 15125);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37554);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 65677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15126);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37555);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65678);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 91904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 37556);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65679);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37557);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 65680);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91906);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111789);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15127);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 37558);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65681);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91907);
									s.is_empty()
								}
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
			super::push(out, 15128);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дор.
pub(crate) fn n3394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3383);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15129);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37560);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65683);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 91908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 65684);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91909);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111790);
											s.is_empty()
										}
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
					super::push(out, 37561);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65685);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 65686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 37562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37563);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65688);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37564);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65689);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91911);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 65690);
							match s {
								[] => true,
								['а', 'л', '\u{1}', s @ ..] => {
									super::push(out, 91912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 65691);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91913);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111791);
											s.is_empty()
										}
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
					super::push(out, 37565);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65692);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37566);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65693);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 65694);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91916);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111792);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124687);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 111793);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124688);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n133860(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37567);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 65695);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91917);
									s.is_empty()
								}
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
			super::push(out, 15130);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37568);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91918);
									s.is_empty()
								}
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
			super::push(out, 15131);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 37569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 15132);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37570);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 37571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15133);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37572);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65698);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91919);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111794);
											s.is_empty()
										}
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
					super::push(out, 37573);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37574);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65700);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91920);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 37575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65701);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 65702);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91921);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 111795);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 124689);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37576);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65703);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 37577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65704);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 65705);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t38::n92899(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t38::n66382(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t38::n37934(s, out),
				['у', s @ ..] => super::t38::n37935(s, out),
				['х', s @ ..] => super::t38::n37936(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t38::n15217(s, out),
		['л', s @ ..] => super::t38::n15218(s, out),
		['н', s @ ..] => super::t38::n15219(s, out),
		['о', s @ ..] => super::t38::n15220(s, out),
		['р', s @ ..] => super::t38::n15221(s, out),
		['с', s @ ..] => super::t38::n15222(s, out),
		['т', s @ ..] => super::t38::n15223(s, out),
		_ => false,
	}
}

/// Words ending -еор.
pub(crate) fn n3395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3384);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 15141);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37595);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жор.
pub(crate) fn n3396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3385);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15142);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37596);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 37597);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65729);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 65730);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91940);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111805);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124693);
													s.is_empty()
												}
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
							super::push(out, 65731);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 91941);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111806);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 124694);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n133864(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37598);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15143);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 37599);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65733);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91942);
									s.is_empty()
								}
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
			super::push(out, 15144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15145);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37601);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 65734);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91943);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 111807);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зор.
pub(crate) fn n3397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3386);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 15146);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37602);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 65735);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 91944);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111808);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 124695);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n133865(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15147);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 15148);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15149);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 15150);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37605);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65736);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91945);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111809);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'к', s @ ..] => {
									super::push(out, 91946);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111810);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 124696);
													match s {
														[] => true,
														['р', s @ ..] => super::t38::n133866(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37606);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 65737);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15151);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37608);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15152);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37609);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65739);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91948);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111811);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124697);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 91949);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111812);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 111813);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124698);
													match s {
														[] => true,
														['е', s @ ..] => super::t38::n133867(s, out),
														_ => false,
													}
												}
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
							super::push(out, 65740);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91950);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111814);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 65741);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91951);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 91952);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 91953);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111815);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124699);
													s.is_empty()
												}
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
							super::push(out, 65742);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37610);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91955);
									s.is_empty()
								}
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
			super::push(out, 15153);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37611);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65744);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 91956);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111816);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37612);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65745);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15155);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 37614);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65747);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91957);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 65748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t38::n15239(s, out),
		['у', s @ ..] => super::t38::n15240(s, out),
		_ => false,
	}
}

/// Words ending -иор.
pub(crate) fn n3398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3387);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15158);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 15159);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 37620);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 65750);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91960);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111818);
											s.is_empty()
										}
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
					super::push(out, 37621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15160);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 15161);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 37623);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65751);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91961);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111819);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йор.
pub(crate) fn n3399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3388);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15162);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37624);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 65752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кор.
pub(crate) fn n3400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3389);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15163);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37625);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 37626);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 37627);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 65753);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91962);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111820);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15164);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37628);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37629);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15165);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37630);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37631);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 37632);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65756);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91963);
									s.is_empty()
								}
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
			super::push(out, 15166);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37633);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 37634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 15167);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 15168);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 15169);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 15170);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 37636);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15171);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 15172);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37637);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65760);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37638);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65761);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37639);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65762);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91966);
									s.is_empty()
								}
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
			super::push(out, 15173);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37640);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 37641);
					match s {
						[] => true,
						['л', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91967);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111821);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 91968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 65763);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 91969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37642);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 37643);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15174);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37644);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37645);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37646);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 65768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 37647);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 37648);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65769);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 91970);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 111822);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 65770);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 91971);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111823);
											s.is_empty()
										}
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
					super::push(out, 37649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 15175);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 15176);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37650);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15177);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15178);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 15179);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37652);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65773);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 37653);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 65774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 15180);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37654);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65775);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лор.
pub(crate) fn n3401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3390);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15181);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15182);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 15183);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15184);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 15185);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37658);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65777);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 65778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15186);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 37659);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65779);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91975);
									s.is_empty()
								}
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
			super::push(out, 15187);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37660);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 15188);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 15189);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мор.
pub(crate) fn n3402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3391);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15190);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37663);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 37664);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65781);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91976);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111824);
											s.is_empty()
										}
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
					super::push(out, 37665);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65782);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37666);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65783);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91978);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 111825);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124700);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15191);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37667);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15192);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37668);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37669);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 15193);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15194);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37672);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65787);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91979);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 111826);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15195);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37673);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65788);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 37674);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15196);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37675);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65790);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 91981);
									s.is_empty()
								}
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
			super::push(out, 15197);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37676);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65791);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 91982);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 65792);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91983);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111827);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 124701);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37677);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 65793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'и', 'с', '\u{1}', s @ ..] => {
					super::push(out, 37678);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 37679);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65794);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 91984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 37680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65795);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 65796);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91985);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 111828);
											s.is_empty()
										}
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
					super::push(out, 37681);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 37682);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 65797);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 91986);
									s.is_empty()
								}
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
			super::push(out, 15198);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37683);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15199);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37684);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15200);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 37685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 15201);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 15202);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37686);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65800);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 91987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нор.
pub(crate) fn n3403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3392);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15203);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37687);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65801);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 91988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 15204);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15205);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37690);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 65802);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 91989);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111829);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 124702);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n133868(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65803);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 65804);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 91990);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 111830);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124703);
													s.is_empty()
												}
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
							super::push(out, 65805);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 91991);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111831);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124704);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15206);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37692);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 65806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37693);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65807);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 91993);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 111832);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124705);
													match s {
														[] => true,
														['м', s @ ..] => super::t38::n133869(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37694);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 37695);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65809);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 91994);
									s.is_empty()
								}
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
			super::push(out, 15207);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37696);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 65810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15208);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37697);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15209);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 37699);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 65813);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 91995);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 111833);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124706);
													match s {
														[] => true,
														['с', s @ ..] => super::t38::n133870(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пор.
pub(crate) fn n3404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3393);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15210);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 37701);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 65815);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 91996);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111834);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 124707);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37702);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 37703);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37704);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65816);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 91997);
									s.is_empty()
								}
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
			super::push(out, 15211);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37705);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15212);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37706);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 65818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15213);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37707);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37708);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15214);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37709);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15215);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37710);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37711);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65822);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 91998);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111835);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 124708);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37712);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 37713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65823);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 65824);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 91999);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 111836);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 124709);
													s.is_empty()
												}
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
							super::push(out, 65825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15216);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65826);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92000);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111837);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 65827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15217);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15218);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37716);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 37717);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 65828);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92001);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111838);
											s.is_empty()
										}
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
					super::push(out, 37718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37719);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 37720);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 65830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15219);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 15220);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37721);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рор.
pub(crate) fn n3405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3394);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15221);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15222);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15223);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37723);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 65831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37724);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 65832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92002);
									s.is_empty()
								}
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
			super::push(out, 15224);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37725);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65833);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92003);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111839);
											s.is_empty()
										}
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
					super::push(out, 37726);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сор.
pub(crate) fn n3406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3395);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15225);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 37727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 15226);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37728);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 65835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15227);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37729);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 65836);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15228);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37730);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92004);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 92005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37731);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65837);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92006);
									s.is_empty()
								}
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
			super::push(out, 15229);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37732);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 37733);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65839);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92007);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111840);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 124710);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 92008);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 111841);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15230);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37734);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 65840);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92009);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111842);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 124711);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n133871(s, out),
														['о', s @ ..] => super::t38::n133872(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 111843);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124712);
													match s {
														[] => true,
														['п', s @ ..] => super::t38::n133873(s, out),
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
									super::push(out, 92010);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 111844);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 65841);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92011);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111845);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 92012);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 111846);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 65842);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92013);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111847);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 124713);
													s.is_empty()
												}
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
							super::push(out, 65843);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92014);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 111848);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t38::n15314(s, out),
		_ => false,
	}
}

/// Words ending -тор.
pub(crate) fn n3407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3396);
	match s {
		[] => true,
		['а', s @ ..] => super::t38::n15315(s, out),
		['в', s @ ..] => {
			super::push(out, 15233);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37754);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65919);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37755);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65920);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92159);
									s.is_empty()
								}
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
			super::push(out, 15234);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37756);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 65921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15235);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37757);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65922);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 65923);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92161);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112011);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124889);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37758);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 65924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 37759);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 65925);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92162);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 65926);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92163);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112012);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124890);
													s.is_empty()
												}
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
							super::push(out, 65927);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92164);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112013);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124891);
													match s {
														[] => true,
														['э', s @ ..] => super::t38::n134007(s, out),
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
									super::push(out, 92165);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112014);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 124892);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n134008(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 124893);
													match s {
														[] => true,
														['с', s @ ..] => super::t38::n134009(s, out),
														_ => false,
													}
												}
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
							super::push(out, 65928);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 65929);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 37760);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65930);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 92168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 65931);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 37761);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 65932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 37762);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 65933);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92170);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112015);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 65934);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92171);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112016);
											s.is_empty()
										}
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
					super::push(out, 37763);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 37764);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 65935);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92172);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112017);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 124894);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37765);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 65936);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 65937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t38::n15319(s, out),
		['к', s @ ..] => super::t38::n15320(s, out),
		['л', s @ ..] => super::t38::n15321(s, out),
		['н', s @ ..] => super::t38::n15322(s, out),
		['о', s @ ..] => super::t38::n15323(s, out),
		['п', s @ ..] => super::t38::n15324(s, out),
		['р', s @ ..] => super::t38::n15325(s, out),
		['с', s @ ..] => super::t38::n15326(s, out),
		['т', s @ ..] => super::t38::n15327(s, out),
		['у', s @ ..] => super::t38::n15328(s, out),
		['ф', s @ ..] => super::t38::n15329(s, out),
		['ю', s @ ..] => super::t38::n15330(s, out),
		['я', s @ ..] => super::t38::n15331(s, out),
		_ => false,
	}
}

/// Words ending -уор.
pub(crate) fn n3408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3397);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 15249);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37801);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66005);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92261);
									s.is_empty()
								}
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
			super::push(out, 15250);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37802);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фор.
pub(crate) fn n3409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3398);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15251);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37803);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 66006);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92262);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37804);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66007);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 37805);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66008);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 37806);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66009);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92265);
									s.is_empty()
								}
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
			super::push(out, 15252);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15253);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 37808);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66011);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92266);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112091);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124946);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37809);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66012);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92267);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112092);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15254);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15255);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37811);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66013);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92268);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 112093);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 124947);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37812);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 37813);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 37814);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 66014);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92269);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112094);
											s.is_empty()
										}
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
						['а', s @ ..] => {
							super::push(out, 66015);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 66016);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92270);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112095);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 124948);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134056(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15256);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37815);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15257);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37816);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66018);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 66019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15258);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37817);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66020);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92271);
									s.is_empty()
								}
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
			super::push(out, 15259);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37818);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хор.
pub(crate) fn n3410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3399);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15260);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 15261);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15262);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37820);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 37821);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66022);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цор.
pub(crate) fn n3411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3400);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15263);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37822);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15264);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37823);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шор.
pub(crate) fn n3412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3401);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15265);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37824);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66025);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92273);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112096);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15266);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 15267);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37826);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 37827);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьор.
pub(crate) fn n3413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3402);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 15268);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37828);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37829);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мини-ссср.
pub(crate) fn n126246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124949);
	s.is_empty()
}

/// Words ending -б-кинотеатр.
pub(crate) fn n126247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124950);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 132688);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 137247);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 139970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ч-педиатр.
pub(crate) fn n126248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124951);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132689);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 137248);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 139971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етр.
pub(crate) fn n3422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3411);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37842);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 37843);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66037);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92280);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 112102);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t38::n15369(s, out),
		['п', s @ ..] => {
			super::push(out, 15287);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37854);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 37855);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15288);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 15289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стр.
pub(crate) fn n3428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3417);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15298);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37861);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 37862);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66082);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 66083);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92347);
									match s {
										[] => true,
										['и', 'п', '\u{1}', s @ ..] => {
											super::push(out, 112162);
											s.is_empty()
										}
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
					super::push(out, 37863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15299);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15300);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 37865);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66084);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92348);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 66085);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92349);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112163);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 124991);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37866);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 37867);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37868);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 37869);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66087);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92350);
									s.is_empty()
								}
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
			super::push(out, 15301);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37870);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 37871);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66089);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 66090);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 37872);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37873);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66092);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92353);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112164);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 124992);
													s.is_empty()
												}
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
							super::push(out, 66093);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 92354);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112165);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 66094);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92355);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112166);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 66095);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92356);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 112167);
											s.is_empty()
										}
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
					super::push(out, 37874);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66096);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92357);
									match s {
										[] => true,
										['-', 'е', s @ ..] => {
											super::push(out, 112168);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 124993);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134079(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 37875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15302);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37876);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66098);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92358);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 112169);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15303);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37877);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 37878);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66100);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 92359);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112170);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124994);
													match s {
														[] => true,
														['д', s @ ..] => super::t38::n134080(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t38::n93356(s, out),
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
			super::push(out, 15304);
			match s {
				[] => true,
				['р', s @ ..] => super::t38::n38240(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t38::n15388(s, out),
		['э', s @ ..] => super::t38::n15389(s, out),
		['ю', s @ ..] => super::t38::n15390(s, out),
		['я', s @ ..] => super::t38::n15391(s, out),
		_ => false,
	}
}

/// Words ending -опинамбур.
pub(crate) fn n126294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124996);
	s.is_empty()
}

/// Words ending -рубадур.
pub(crate) fn n93364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92368);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 112175);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -падур.
pub(crate) fn n38267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37906);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 66117);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -радур.
pub(crate) fn n38268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37907);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 66118);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 92369);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 112176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идур.
pub(crate) fn n15418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15335);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 37908);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 66119);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндур.
pub(crate) fn n15419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15336);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 37909);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 66120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 37910);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 66121);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 92370);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 112177);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 124997);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 132713);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n138679(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одур.
pub(crate) fn n15420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15337);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 37911);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 66122);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 92371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удур.
pub(crate) fn n15421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15338);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 37912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 66123);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жур.
pub(crate) fn n3437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3426);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15339);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37913);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', 'г', 'и', '\u{1}', s @ ..] => {
			super::push(out, 15340);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 15341);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 37914);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66125);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92372);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зур.
pub(crate) fn n3438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3427);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15342);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кур.
pub(crate) fn n3439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3428);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15344);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15345);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 37916);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 37917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15346);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37918);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15347);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37919);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 37920);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66128);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92373);
									s.is_empty()
								}
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
			super::push(out, 15348);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37921);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15349);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37922);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 37923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15350);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 37924);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 66130);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 37925);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15351);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37926);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 15352);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37927);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лур.
pub(crate) fn n3440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3429);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15353);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 37928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15354);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37929);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66134);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 92375);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 112179);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124998);
													match s {
														[] => true,
														['у', s @ ..] => super::t38::n134083(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мур.
pub(crate) fn n3441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3430);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15355);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15356);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37930);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 37931);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 37932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15357);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37933);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 37934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 15358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15359);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 37936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 15360);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37937);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нур.
pub(crate) fn n3442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3431);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15361);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 15362);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 37938);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15363);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15364);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37940);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 66137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 37941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15365);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15366);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37943);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 37944);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66138);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92376);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112180);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124999);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 37945);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66139);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оур.
pub(crate) fn n3443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3432);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 15367);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пур.
pub(crate) fn n3444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3433);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15368);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 37947);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15369);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 37948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15370);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 37949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15371);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37950);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 66141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15372);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 37951);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15373);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сур.
pub(crate) fn n3445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3434);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15374);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 15375);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37952);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 66143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15376);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37953);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15377);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37954);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 66145);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92378);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112181);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 66146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 15378);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37955);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тур.
pub(crate) fn n3446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3435);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 15379);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15380);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37956);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 37957);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 37958);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66149);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 37959);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 66150);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 37960);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66151);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 37961);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15381);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15382);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37963);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15383);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15384);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37965);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66154);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 66155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 37966);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66156);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 66157);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92382);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112182);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125000);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134084(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15385);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37967);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66158);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92383);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 112183);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 125001);
													match s {
														[] => true,
														['у', s @ ..] => super::t38::n134085(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фур.
pub(crate) fn n3447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3436);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15386);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 37968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15387);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 15388);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 37970);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хур.
pub(crate) fn n3448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3437);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15389);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 37971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15390);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 37972);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чур.
pub(crate) fn n3449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3438);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 15391);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 37973);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 66161);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шур.
pub(crate) fn n3450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3439);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 15392);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 37974);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15393);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 15394);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 37975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щур.
pub(crate) fn n3451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3440);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15395);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15396);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37976);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15397);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 37977);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15398);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37978);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 15399);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37979);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яур.
pub(crate) fn n3452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3441);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 15400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рандмауэр.
pub(crate) fn n126300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125002);
	s.is_empty()
}

/// The word от-кутюр.
pub(crate) fn n126301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125003);
	s.is_empty()
}

/// Words ending -ье-футляр.
pub(crate) fn n126302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125004);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 132717);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 137263);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -есняр.
pub(crate) fn n38385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38024);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 66197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 92399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ояр.
pub(crate) fn n3487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3476);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15445);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38025);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15446);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 15447);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 38026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 15448);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 38027);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьяр.
pub(crate) fn n3488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3477);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 15449);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38028);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92400);
									s.is_empty()
								}
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
			super::push(out, 15450);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38029);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66201);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92401);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 112192);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15451);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38030);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66202);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92402);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112193);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нерал-бас.
pub(crate) fn n126303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125005);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 132718);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нджубас.
pub(crate) fn n93409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92413);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 112195);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 125006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нубас.
pub(crate) fn n38414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38053);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 66221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рубас.
pub(crate) fn n38415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38054);
	s.is_empty()
}

/// Words ending -вас.
pub(crate) fn n3492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3481);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15466);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15467);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15468);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38057);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 38058);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 66222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15469);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38059);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15470);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38060);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66224);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гас.
pub(crate) fn n3493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3482);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15471);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15472);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66225);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 66226);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92415);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112196);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125007);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15473);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 38064);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66227);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92416);
									s.is_empty()
								}
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
			super::push(out, 15474);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38065);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 38066);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66228);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92417);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112197);
											s.is_empty()
										}
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
					super::push(out, 38067);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15475);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66230);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 66231);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 66232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15476);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 38069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дас.
pub(crate) fn n3494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3483);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 15477);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38070);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15478);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38071);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15479);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еас.
pub(crate) fn n3495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3484);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15480);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38073);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15481);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38074);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66237);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92418);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112198);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жас.
pub(crate) fn n3496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3485);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 15482);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38075);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15483);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38076);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зас.
pub(crate) fn n3497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3486);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 15484);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иас.
pub(crate) fn n3498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3487);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 15485);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38078);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15486);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15487);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15488);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38081);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66241);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92419);
									s.is_empty()
								}
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
			super::push(out, 15489);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38082);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йас.
pub(crate) fn n3499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3488);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15490);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кас.
pub(crate) fn n3500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3489);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15491);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66244);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92420);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 92421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 38085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15492);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38086);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15493);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38087);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 38088);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66246);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 38089);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38090);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66247);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92423);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112199);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15494);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 38091);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 15495);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38092);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66249);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 92424);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112200);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125008);
													match s {
														[] => true,
														['б', s @ ..] => super::t38::n134088(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15496);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38093);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66250);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 66251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15497);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38094);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 66252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15498);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38095);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 15499);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38096);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 66254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лас.
pub(crate) fn n3501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3490);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15500);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38097);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 15501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 15502);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38100);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 66255);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92425);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112201);
											s.is_empty()
										}
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
					super::push(out, 38101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66256);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 92426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38102);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66257);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92427);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112202);
											s.is_empty()
										}
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
					super::push(out, 38103);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38104);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38105);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38106);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66261);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92428);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 66262);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92429);
									s.is_empty()
								}
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
			super::push(out, 15503);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38107);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66263);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92430);
									s.is_empty()
								}
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
			super::push(out, 15504);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38108);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 15505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38110);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 38111);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38112);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15506);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38113);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38114);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15507);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38115);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 66269);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92431);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112203);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 125009);
													s.is_empty()
												}
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
							super::push(out, 66270);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38116);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мас.
pub(crate) fn n3502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3491);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15508);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15509);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38117);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66272);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 38118);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 38119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15510);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15511);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38121);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15512);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15513);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38123);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66276);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38124);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66277);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66278);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 66279);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92436);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 112204);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 125010);
													s.is_empty()
												}
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
							super::push(out, 66280);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92437);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 112205);
											s.is_empty()
										}
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
					super::push(out, 38126);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92438);
									s.is_empty()
								}
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
			super::push(out, 15514);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38127);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66282);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92439);
									s.is_empty()
								}
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
			super::push(out, 15515);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 38128);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66283);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92440);
									s.is_empty()
								}
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
			super::push(out, 15516);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38129);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66284);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92441);
									s.is_empty()
								}
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
			super::push(out, 15517);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38130);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66285);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 92442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нас.
pub(crate) fn n3503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3492);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15518);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38132);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 38133);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 66288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38134);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66289);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 38135);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 15519);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15520);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38137);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66291);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92444);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112206);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 125011);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15521);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 38139);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 38140);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38141);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 66292);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92445);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112207);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 125012);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15522);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38142);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15523);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38143);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66294);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 38144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оас.
pub(crate) fn n3504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3493);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 15524);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 38145);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пас.
pub(crate) fn n3505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3494);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 15525);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 15526);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38147);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 38148);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15527);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38150);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38151);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 38152);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 66300);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92447);
									s.is_empty()
								}
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
			super::push(out, 15528);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38153);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66301);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92448);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112208);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125013);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n134089(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38154);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 15529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 15530);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38156);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38157);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15531);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38158);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 38159);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66306);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92449);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38160);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 38161);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38162);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66309);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 92450);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 112209);
											s.is_empty()
										}
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
					super::push(out, 38163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15532);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38164);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15533);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 15534);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38166);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 15535);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 38167);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 66312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рас.
pub(crate) fn n3506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3495);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15536);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38168);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66313);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 38170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 15537);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 15538);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38171);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38172);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 66316);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92453);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112210);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15539);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38173);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 38174);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66317);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92454);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112211);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15540);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38175);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66318);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92455);
									s.is_empty()
								}
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
			super::push(out, 15541);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 38176);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66319);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92456);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112212);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 125014);
													match s {
														[] => true,
														['о', s @ ..] => super::t38::n134090(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15542);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38177);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66320);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 92457);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 112213);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125015);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134091(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15543);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15544);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38179);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66321);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92458);
									s.is_empty()
								}
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
			super::push(out, 15545);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15546);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38181);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38182);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66323);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92459);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112214);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15547);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 15548);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38184);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 66324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сас.
pub(crate) fn n3507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3496);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15549);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тас.
pub(crate) fn n3508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3497);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15550);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38185);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66325);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 92460);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112215);
											s.is_empty()
										}
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
					super::push(out, 38186);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15551);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38187);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66327);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92461);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 112216);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125016);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134092(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38188);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66328);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92462);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112217);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 125017);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15552);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38189);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66329);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92463);
									s.is_empty()
								}
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
			super::push(out, 15553);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38190);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 38191);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92464);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 66331);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92465);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112218);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125018);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15554);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38192);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уас.
pub(crate) fn n3509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3498);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 15555);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38193);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66332);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 92466);
									s.is_empty()
								}
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
			super::push(out, 15556);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38194);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фас.
pub(crate) fn n3510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3499);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15557);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15558);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38196);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66335);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хас.
pub(crate) fn n3511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3500);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15559);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 15560);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15561);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 15562);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -час.
pub(crate) fn n3512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3501);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15563);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 15564);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38200);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66336);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92468);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 112219);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 125019);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['н', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92469);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 112220);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125020);
													match s {
														[] => true,
														['м', s @ ..] => super::t38::n134093(s, out),
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
									super::push(out, 92470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 66337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38201);
					s.is_empty()
				}
				['т', 'т', 'а', 'в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66338);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 92471);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112221);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125021);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t38::n134094(s, out),
														_ => false,
													}
												}
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
							super::push(out, 66339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15565);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 15566);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15567);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38203);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
			super::push(out, 15568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аниотабес.
pub(crate) fn n126321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125023);
	s.is_empty()
}

/// Words ending -средневес.
pub(crate) fn n126322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125024);
	s.is_empty()
}

/// Words ending -гковес.
pub(crate) fn n67056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66362);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 92487);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 112227);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловес.
pub(crate) fn n38596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38233);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 66363);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 92488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новес.
pub(crate) fn n38597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38234);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 66364);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ровес.
pub(crate) fn n38598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38235);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 66365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -свес.
pub(crate) fn n15678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15593);
	s.is_empty()
}

/// Words ending -твес.
pub(crate) fn n15679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15594);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 38236);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гес.
pub(crate) fn n3527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3516);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 15595);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38237);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дес.
pub(crate) fn n3528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3517);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 15596);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 38238);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66367);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92489);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112228);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15597);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38239);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66368);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 92490);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112229);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 125025);
													match s {
														[] => true,
														['т', s @ ..] => super::t38::n134095(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15598);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38240);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 66369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 38241);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15599);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 15600);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еес.
pub(crate) fn n3529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3518);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 15601);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38244);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зес.
pub(crate) fn n3530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3519);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 15602);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 38245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иес.
pub(crate) fn n3531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3520);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 15603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38246);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 15604);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38247);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кес.
pub(crate) fn n3532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3521);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 15605);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15606);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38249);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 66373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 15607);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 15608);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38250);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66374);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92491);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112230);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 125026);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лес.
pub(crate) fn n3533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3522);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 15609);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 15610);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 38251);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 38252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15611);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 38254);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66377);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92492);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112231);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 125027);
													match s {
														[] => true,
														['с', s @ ..] => super::t38::n134096(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38255);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66378);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92493);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 112232);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 125028);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 38256);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66379);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92494);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112233);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15612);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38257);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66380);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92495);
									s.is_empty()
								}
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
			super::push(out, 15613);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38258);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 66381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 38259);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66382);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 38260);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 66383);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92497);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 38261);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 66384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15614);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38263);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66386);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 38264);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66387);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92499);
									s.is_empty()
								}
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
			super::push(out, 15615);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 15616);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38265);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 66388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 15617);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38266);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66389);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92500);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112234);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 125029);
													match s {
														[] => true,
														['к', s @ ..] => super::t38::n134097(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мес.
pub(crate) fn n3534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3523);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15618);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 15619);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 38268);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 15620);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 38269);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66391);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 66392);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92502);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112235);
											s.is_empty()
										}
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
					super::push(out, 38270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 15621);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38271);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 38272);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 66395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 38273);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15622);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 38274);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 66397);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 66398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 15623);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38275);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66399);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 66400);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 92503);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 66401);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92504);
									s.is_empty()
								}
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
			super::push(out, 15624);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 38276);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66402);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92505);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112237);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 66403);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 66404);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 92507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нес.
pub(crate) fn n3535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3524);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15625);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 38277);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66405);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92508);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112238);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15626);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38278);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66406);
							match s {
								[] => true,
								['о', 'р', s @ ..] => {
									super::push(out, 92509);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 112239);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15627);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38279);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 66407);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 92510);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 112240);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15628);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 38280);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 66408);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92511);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 112241);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 125030);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134098(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15629);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38281);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 66409);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92512);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 112242);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 15630);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 38282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 15631);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38283);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пес.
pub(crate) fn n3536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3525);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 15632);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38284);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 38285);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66411);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92513);
									s.is_empty()
								}
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
			super::push(out, 15633);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 38286);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66412);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 92514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 38287);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 66413);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 92515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 38288);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66414);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 92516);
									s.is_empty()
								}
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
			super::push(out, 15634);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 38289);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 66415);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 66416);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 92517);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 112243);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125031);
													match s {
														[] => true,
														['м', s @ ..] => super::t38::n134099(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рес.
pub(crate) fn n3537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3526);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 15635);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 38290);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 38291);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66417);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 92518);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 112244);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 125032);
													match s {
														[] => true,
														['а', s @ ..] => super::t38::n134100(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15636);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38292);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 66418);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 92519);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 92520);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 112246);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 66419);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 92521);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 112247);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 92522);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 112248);
											s.is_empty()
										}
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
					super::push(out, 38293);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 66420);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 92523);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 112249);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 125033);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 15637);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 38294);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 38295);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66422);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 92524);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 112250);
											s.is_empty()
										}
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
					super::push(out, 38296);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 66423);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 92525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 38297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 15638);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38298);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 66425);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 66426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 15639);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 38299);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 66427);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 92526);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 112251);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 125034);
													match s {
														[] => true,
														['б', s @ ..] => super::t38::n134101(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 38300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 66428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 38301);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 66429);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 92527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 66430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 15640);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 38302);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 66431);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 92528);
									s.is_empty()
								}
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
			super::push(out, 15641);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 38303);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сес.
pub(crate) fn n3538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3527);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 15642);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 38304);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 66432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

