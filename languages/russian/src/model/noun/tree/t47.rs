use flexion::Path;

/// Words ending -етник.
pub(crate) fn n30937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30715);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58407);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86217);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107928);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 122454);
									s.is_empty()
								}
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
			super::push(out, 58408);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86218);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 107929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 86219);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107930);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 107931);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122456);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131306);
											s.is_empty()
										}
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
					super::push(out, 86220);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107932);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122457);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131307);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136482);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140959(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 86221);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 107933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 86222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107934);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 107935);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 107936);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 107937);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 122460);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 122461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 107938);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122462);
									s.is_empty()
								}
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
			super::push(out, 58409);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 86223);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 107939);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 122463);
									s.is_empty()
								}
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
			super::push(out, 58410);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86224);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107940);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 122464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 86225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58411);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86226);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 86227);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107942);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122465);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131308);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 107943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 86228);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 107944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86229);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 86230);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 86231);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 107947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58412);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 86232);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 86233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58413);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86234);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 86235);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86236);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 107950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 58414);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86237);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58415);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86238);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 107952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 86239);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107953);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 86240);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 107954);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 107955);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122467);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131309);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58416);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 86241);
					match s {
						[] => true,
						['а', s @ ..] => super::t50::n109105(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t50::n59028(s, out),
		['ц', s @ ..] => super::t50::n59029(s, out),
		['ч', s @ ..] => super::t50::n59030(s, out),
		['ш', s @ ..] => super::t50::n59031(s, out),
		_ => false,
	}
}

/// Words ending -итник.
pub(crate) fn n30938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30716);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58421);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 58422);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 86248);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107962);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122470);
									s.is_empty()
								}
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
			super::push(out, 58423);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86249);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 86250);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107963);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122471);
									s.is_empty()
								}
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
			super::push(out, 58424);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 86251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86252);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 86253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58425);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 86254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58426);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86255);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 107965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 86256);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', '\u{1}', s @ ..] => {
			super::push(out, 58427);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 58428);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86257);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 58429);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86258);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 107968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктник.
pub(crate) fn n30939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30717);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58430);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 86259);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107969);
							match s {
								[] => true,
								['н', 'о', 'к', '-', s @ ..] => {
									super::push(out, 122472);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 131311);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136483);
													match s {
														[] => true,
														['щ', s @ ..] => super::t50::n140960(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58431);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86260);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 107970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтник.
pub(crate) fn n30940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30718);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58432);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 86261);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'ц', s @ ..] => {
			super::push(out, 58433);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 86262);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 107971);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122473);
									s.is_empty()
								}
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
			super::push(out, 58434);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 86263);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 107972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отник.
pub(crate) fn n30941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30719);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58435);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86264);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 86265);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 107974);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 122474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86266);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 107975);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122475);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131312);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 136484);
													match s {
														[] => true,
														['ы', s @ ..] => super::t50::n140961(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58436);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86267);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 107976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 86268);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 107977);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122476);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131313);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58437);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 86269);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 58438);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86270);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 107979);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122477);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 122478);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 122479);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131314);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136485);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 86271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58439);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 86272);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107980);
							match s {
								[] => true,
								['е', 'м', '\u{1}', s @ ..] => {
									super::push(out, 122480);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 86273);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 107981);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122481);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131315);
											s.is_empty()
										}
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
					super::push(out, 86274);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 107982);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 107983);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 86275);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 107984);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122483);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131316);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136486);
													match s {
														[] => true,
														['ч', s @ ..] => super::t50::n140962(s, out),
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
									super::push(out, 122484);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131317);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136487);
													match s {
														[] => true,
														['м', s @ ..] => super::t50::n140963(s, out),
														_ => false,
													}
												}
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
							super::push(out, 107985);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122485);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 131318);
											s.is_empty()
										}
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
					super::push(out, 86276);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 107986);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122486);
									s.is_empty()
								}
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
			super::push(out, 58440);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86277);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 107987);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122487);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131319);
											s.is_empty()
										}
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
					super::push(out, 86278);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 107988);
							match s {
								[] => true,
								['е', s @ ..] => super::t50::n123765(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t50::n87200(s, out),
				['х', s @ ..] => super::t50::n87201(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t50::n59053(s, out),
		['р', s @ ..] => super::t50::n59054(s, out),
		['с', s @ ..] => super::t50::n59055(s, out),
		['т', s @ ..] => super::t50::n59056(s, out),
		['х', s @ ..] => super::t50::n59057(s, out),
		_ => false,
	}
}

/// Words ending -ртник.
pub(crate) fn n30942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30720);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58446);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 86289);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 108000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 58447);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 86290);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108001);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122502);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131328);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136493);
													match s {
														[] => true,
														['и', s @ ..] => super::t50::n140967(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ь', s @ ..] => {
											super::push(out, 131329);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 136494);
													match s {
														[] => true,
														['и', s @ ..] => super::t50::n140968(s, out),
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
									super::push(out, 122503);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131330);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136495);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 86291);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 108002);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122504);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131331);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58448);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 86292);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 86293);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108003);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 122505);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 86294);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108004);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122506);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131332);
											match s {
												[] => true,
												['р', 'т', 'о', s @ ..] => {
													super::push(out, 136496);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 86295);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 108005);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стник.
pub(crate) fn n30943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30721);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58449);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 86296);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 108006);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122508);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131333);
											s.is_empty()
										}
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
					super::push(out, 86297);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 86298);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 108007);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122509);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131334);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 136497);
													match s {
														[] => true,
														['т', s @ ..] => super::t50::n140969(s, out),
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
									super::push(out, 122510);
									s.is_empty()
								}
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
			super::push(out, 58450);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 86299);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108008);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 108009);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 108010);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 108011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 86300);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108012);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122512);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 131335);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 136498);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140970(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 86301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 58451);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 86303);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108014);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 86304);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108015);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122514);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131336);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136499);
													s.is_empty()
												}
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
							super::push(out, 108016);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 122515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 108017);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 108018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 86305);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108019);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122517);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131337);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136500);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58452);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 86306);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108020);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 122518);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131338);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => super::t50::n109170(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 86307);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108022);
							match s {
								[] => true,
								['п', s @ ..] => super::t50::n123800(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t50::n87229(s, out),
				['п', s @ ..] => super::t50::n87230(s, out),
				['р', s @ ..] => super::t50::n87231(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t50::n59065(s, out),
		['у', s @ ..] => super::t50::n59066(s, out),
		_ => false,
	}
}

/// Words ending -утник.
pub(crate) fn n30944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30722);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58455);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 86314);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108029);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122529);
									s.is_empty()
								}
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
			super::push(out, 58456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86315);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 86316);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108030);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 122530);
									s.is_empty()
								}
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
			super::push(out, 58457);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86317);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 86318);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 108031);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122531);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 122532);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131347);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136502);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 86319);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 58458);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86320);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 86321);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 86322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108034);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 108035);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122533);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131348);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136503);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140971(s, out),
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
									super::push(out, 122534);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131349);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136504);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140972(s, out),
														_ => false,
													}
												}
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
							super::push(out, 108036);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 108037);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 122536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 108038);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122537);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 122538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 108039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58459);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 86323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 58460);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86324);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фтник.
pub(crate) fn n30945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30723);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытник.
pub(crate) fn n30946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30724);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58462);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 58463);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 58464);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108040);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 108041);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122539);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 122540);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131350);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136505);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 122541);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131351);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136506);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n140973(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58465);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86327);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58466);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьтник.
pub(crate) fn n30947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30725);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 58467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ютник.
pub(crate) fn n30948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30726);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58468);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86328);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58469);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86329);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятник.
pub(crate) fn n30949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30727);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58470);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 86330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 58471);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 58472);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86332);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108045);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122542);
									match s {
										[] => true,
										['т', 'е', 'т', '-', s @ ..] => {
											super::push(out, 131352);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136507);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140974(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 86333);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108046);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 86334);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122544);
									s.is_empty()
								}
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
			super::push(out, 58473);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86335);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108048);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122545);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 131353);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58474);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 86336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58475);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86337);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108049);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122546);
									match s {
										[] => true,
										['р', 'е', 'п', '-', s @ ..] => {
											super::push(out, 131354);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136508);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140975(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 86338);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 108050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 86339);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108051);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122547);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131355);
											s.is_empty()
										}
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
					super::push(out, 86340);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 86341);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 108052);
							match s {
								[] => true,
								['ц', '\u{1}', s @ ..] => {
									super::push(out, 122548);
									s.is_empty()
								}
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
			super::push(out, 58476);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86342);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108053);
							match s {
								[] => true,
								['-', s @ ..] => super::t50::n123826(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86343);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108054);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 122550);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131360);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t50::n59089(s, out),
		['п', s @ ..] => super::t50::n59090(s, out),
		['р', s @ ..] => super::t50::n59091(s, out),
		['с', s @ ..] => super::t50::n59092(s, out),
		['т', s @ ..] => {
			super::push(out, 58481);
			match s {
				[] => true,
				['у', s @ ..] => super::t50::n87275(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 58482);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рь-сантехник.
pub(crate) fn n137925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136513);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139540);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141318);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142340);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 142921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -свечник.
pub(crate) fn n87292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86370);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 108076);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 108077);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 122563);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 131365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 108078);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 122564);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 131366);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 136514);
									s.is_empty()
								}
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
			super::push(out, 108079);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 122565);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 131367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едечник.
pub(crate) fn n87293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86371);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 108080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдечник.
pub(crate) fn n87294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86372);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 108081);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 122566);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еечник.
pub(crate) fn n59111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58499);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 86373);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108082);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122567);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131368);
									s.is_empty()
								}
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
			super::push(out, 86374);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108083);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122568);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131369);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 131370);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 136515);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 139541);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86375);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108084);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 122569);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131371);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 136516);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жечник.
pub(crate) fn n59112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58500);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86376);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86377);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 86378);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108087);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 108088);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122570);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 122571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лечник.
pub(crate) fn n59113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58501);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86379);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 108089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 86380);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108090);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 122573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 86381);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108091);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 122574);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 122575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108092);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мечник.
pub(crate) fn n59114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58502);
	s.is_empty()
}

/// Words ending -нечник.
pub(crate) fn n59115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58503);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 86382);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108093);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122577);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131372);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136517);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139542);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86383);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108094);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122578);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 131373);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 122579);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оечник.
pub(crate) fn n59116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58504);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 86384);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 108095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 86385);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 108096);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -печник.
pub(crate) fn n59117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58505);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 86386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -речник.
pub(crate) fn n59118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58506);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 86387);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 86388);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86389);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108098);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122580);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131375);
									s.is_empty()
								}
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
			super::push(out, 86390);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108099);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122581);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131376);
									s.is_empty()
								}
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
			super::push(out, 86391);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 108100);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 86392);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 108101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сечник.
pub(crate) fn n59119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58507);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86393);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108102);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -течник.
pub(crate) fn n59120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58508);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 86394);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108103);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122584);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 131377);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 108104);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чечник.
pub(crate) fn n59121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58509);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86395);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 108105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 86396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108106);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 108107);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 108108);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122587);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131378);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 136518);
											s.is_empty()
										}
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
					super::push(out, 108109);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122588);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 131379);
									s.is_empty()
								}
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
			super::push(out, 86397);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шечник.
pub(crate) fn n59122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58510);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86398);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108111);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 122589);
							match s {
								[] => true,
								['р', 'о', 'н', 'р', s @ ..] => {
									super::push(out, 131380);
									match s {
										[] => true,
										['е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 136519);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 86399);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 86400);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 86401);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108114);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 86402);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108115);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122591);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									super::push(out, 131381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 108116);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 108117);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щечник.
pub(crate) fn n59123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58511);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86403);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 108118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 86404);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108119);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичник.
pub(crate) fn n30959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30737);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58512);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 86405);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108120);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122593);
									s.is_empty()
								}
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
			super::push(out, 58513);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 86406);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108121);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122594);
									s.is_empty()
								}
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
			super::push(out, 58514);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86407);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108122);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 122595);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131382);
											s.is_empty()
										}
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
					super::push(out, 86408);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108123);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 108124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 86409);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108125);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 122596);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86410);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 108126);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 122597);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131383);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136520);
													s.is_empty()
												}
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
							super::push(out, 108127);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122598);
									s.is_empty()
								}
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
			super::push(out, 58515);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 86411);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108128);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 122599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86412);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 108129);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 122600);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 131384);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 108130);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122601);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 131385);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136521);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n140983(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58516);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86413);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 108131);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122602);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131386);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136522);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 86414);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 108132);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 122603);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131387);
											s.is_empty()
										}
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
					super::push(out, 86415);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 108133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 86416);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 86417);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108135);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 86418);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 86419);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 86420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58517);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 86421);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108139);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t50::n59130(s, out),
		['п', s @ ..] => {
			super::push(out, 58519);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 86432);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 86433);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58520);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 86434);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108155);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 122620);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 86435);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 86436);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 86437);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58521);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 86438);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 86439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 58522);
			match s {
				[] => true,
				['р', s @ ..] => super::t50::n87363(s, out),
				_ => false,
			}
		}
		['щ', s @ ..] => super::t50::n59135(s, out),
		['я', s @ ..] => {
			super::push(out, 58524);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лчник.
pub(crate) fn n30960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30738);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 58525);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 86442);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очник.
pub(crate) fn n30961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30739);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58526);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 86443);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 58527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86444);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 108163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86445);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122622);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 122623);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131395);
											s.is_empty()
										}
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
					super::push(out, 86446);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108164);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 108165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 86447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 58528);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86448);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 108166);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 108167);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122626);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131396);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136526);
													s.is_empty()
												}
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
							super::push(out, 108168);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 108169);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122627);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131397);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 108170);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122628);
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
						['л', s @ ..] => {
							super::push(out, 108171);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122629);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131398);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 136527);
													s.is_empty()
												}
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
							super::push(out, 108172);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 108173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 86449);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108174);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122630);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131399);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 108175);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122631);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131400);
											s.is_empty()
										}
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
					super::push(out, 86450);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108176);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 122632);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131401);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 108177);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122633);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131402);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136528);
													s.is_empty()
												}
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
							super::push(out, 108178);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122634);
									s.is_empty()
								}
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
			super::push(out, 58529);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86451);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 58530);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86452);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 86453);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108179);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 108180);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122635);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131403);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 136529);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140985(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 86454);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 108181);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122636);
									s.is_empty()
								}
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
			super::push(out, 58531);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86455);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108182);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t50::n59144(s, out),
		['м', s @ ..] => super::t50::n59145(s, out),
		['н', s @ ..] => super::t51::n59146(s, out),
		['п', s @ ..] => super::t51::n59147(s, out),
		['р', s @ ..] => super::t51::n59148(s, out),
		['с', s @ ..] => super::t51::n59149(s, out),
		['т', s @ ..] => super::t51::n59150(s, out),
		['ч', s @ ..] => {
			super::push(out, 58539);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 86504);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108265);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t51::n59152(s, out),
		_ => false,
	}
}

/// Words ending -учник.
pub(crate) fn n30962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30740);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58541);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 86506);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 58542);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58543);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 86508);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 86509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58544);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86510);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 86511);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108270);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122702);
									s.is_empty()
								}
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
			super::push(out, 58545);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86512);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 86513);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 108271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 86514);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 86515);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108273);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86516);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58546);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 86517);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 108275);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 86518);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108276);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122705);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 131440);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136554);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 136555);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141000(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ычник.
pub(crate) fn n30963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30741);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 58547);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 86519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58548);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 86520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьчник.
pub(crate) fn n30964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30742);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 58549);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86521);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108277);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122706);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131441);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючник.
pub(crate) fn n30965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30743);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 58550);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86522);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 108278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58551);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 86523);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 86524);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108279);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122707);
									s.is_empty()
								}
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
			super::push(out, 58552);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 86525);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 86526);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячник.
pub(crate) fn n30966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30744);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58553);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 86527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58554);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86528);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 108281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86529);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 86530);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 108283);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122708);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 122709);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 131442);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 131443);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136556);
													match s {
														[] => true,
														['ц', s @ ..] => super::t51::n141001(s, out),
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
									super::push(out, 122710);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 131444);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136557);
													match s {
														[] => true,
														['-', s @ ..] => super::t51::n141002(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', 'о', 'с', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 122711);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мешник.
pub(crate) fn n59185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58571);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86554);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86555);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108299);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122718);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 131447);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136558);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139563);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86556);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 108300);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86557);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пешник.
pub(crate) fn n59186(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 86558);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 108302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 86559);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 108303);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86560);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 108304);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122721);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131448);
									s.is_empty()
								}
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
			super::push(out, 86561);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108305);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 108306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -грешник.
pub(crate) fn n87487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86562);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 108307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ерешник.
pub(crate) fn n87488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86563);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 108308);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трешник.
pub(crate) fn n87490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86565);
	s.is_empty()
}

/// Words ending -тешник.
pub(crate) fn n59188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58573);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86566);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 86567);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уешник.
pub(crate) fn n59189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58574);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чешник.
pub(crate) fn n59190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58575);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 86569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ишник.
pub(crate) fn n30971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30749);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58576);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 86570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 58577);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86571);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108309);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122722);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131449);
											s.is_empty()
										}
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
					super::push(out, 86572);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 108310);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122723);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 131450);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 58578);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 58579);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 86574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58580);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86575);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 108311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58581);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86576);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108312);
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
				['й', s @ ..] => {
					super::push(out, 86577);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108313);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 122724);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 131451);
											s.is_empty()
										}
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
					super::push(out, 86578);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 108314);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122725);
									s.is_empty()
								}
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
			super::push(out, 58582);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86579);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122726);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 86580);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58583);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86581);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58584);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86582);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58585);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 86583);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 58586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 58587);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 86585);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108320);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 122727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ошник.
pub(crate) fn n30972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30750);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 58588);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 86586);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 108321);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 122728);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 131452);
											s.is_empty()
										}
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
					super::push(out, 86587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 58589);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86588);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 86589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 58590);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86590);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58591);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86591);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108323);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122729);
									s.is_empty()
								}
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
			super::push(out, 58592);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86592);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108324);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122730);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 131453);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 108325);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122731);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131454);
											s.is_empty()
										}
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
					super::push(out, 86593);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58593);
			match s {
				[] => true,
				['в', 'а', 'р', 'е', 'с', 'в', '\u{1}', s @ ..] => {
					super::push(out, 86594);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 86595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 58594);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 58595);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86597);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86598);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 108328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58596);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 58597);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86599);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 86600);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108329);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 86601);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 86602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58598);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86603);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 86604);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108331);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86605);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 86606);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 108333);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122735);
									s.is_empty()
								}
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
			super::push(out, 58599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -авершник.
pub(crate) fn n30973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30751);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 58600);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ушник.
pub(crate) fn n30974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30752);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 58601);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 58602);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 86607);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 86608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 58603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86609);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 86610);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 108335);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122736);
									s.is_empty()
								}
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
			super::push(out, 58604);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 58605);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86611);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 86612);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 86613);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 86614);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108336);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86615);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 86616);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 86617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 58606);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 58607);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86618);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86619);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86620);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 86621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 86622);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108341);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 108342);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122739);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131455);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136559);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 58609);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86623);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108343);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122740);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 131456);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 108344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58610);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86625);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108345);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 86626);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 58611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86627);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 86628);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86629);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86630);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 86631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 58613);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86632);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 86633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 58614);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 86634);
					match s {
						[] => true,
						['е', 'ж', '\u{1}', s @ ..] => {
							super::push(out, 108348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 58615);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 58616);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 58617);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86635);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108349);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122743);
									s.is_empty()
								}
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
			super::push(out, 58618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ышник.
pub(crate) fn n30975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30753);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 58619);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 86636);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58620);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 86637);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 58621);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 86638);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108352);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122744);
									s.is_empty()
								}
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
			super::push(out, 58622);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86639);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 108353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 86640);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108354);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122745);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131457);
											s.is_empty()
										}
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
					super::push(out, 86641);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 108355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 86642);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108356);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 122746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эшник.
pub(crate) fn n30976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30754);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 58623);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 58624);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юшник.
pub(crate) fn n30977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30755);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 58625);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58626);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 58627);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 86645);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яшник.
pub(crate) fn n30978(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 58628);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86646);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 58629);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 86647);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льник.
pub(crate) fn n30983(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30760);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58633);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 86650);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 86651);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108360);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 122747);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131458);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 108361);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 122748);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'л', s @ ..] => {
							super::push(out, 108362);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122749);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 131459);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136560);
													s.is_empty()
												}
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
							super::push(out, 108363);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 86652);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 108364);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 122751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 86653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108365);
							s.is_empty()
						}
						['н', 'а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122752);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 122753);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131460);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136561);
													s.is_empty()
												}
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
							super::push(out, 108366);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', 'р', s @ ..] => {
					super::push(out, 86654);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108367);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 122755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 86655);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 86656);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 86657);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108370);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 108371);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 108372);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122756);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 131461);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136562);
													match s {
														[] => true,
														['к', s @ ..] => super::t51::n141004(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 86658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108373);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 108374);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 122757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 108375);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122758);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131462);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136563);
													s.is_empty()
												}
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
							super::push(out, 108376);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 108377);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122759);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 131463);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136564);
													match s {
														[] => true,
														['ь', s @ ..] => super::t51::n141005(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t51::n87584(s, out),
				['с', s @ ..] => {
					super::push(out, 86660);
					s.is_empty()
				}
				['т', s @ ..] => super::t51::n87586(s, out),
				['х', s @ ..] => super::t51::n87587(s, out),
				['ч', s @ ..] => super::t51::n87588(s, out),
				['ш', s @ ..] => super::t51::n87589(s, out),
				['щ', s @ ..] => super::t51::n87590(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t51::n59250(s, out),
		['и', s @ ..] => super::t51::n59251(s, out),
		['л', s @ ..] => super::t51::n59252(s, out),
		['о', s @ ..] => super::t51::n59253(s, out),
		['у', s @ ..] => super::t51::n59254(s, out),
		['ы', s @ ..] => super::t51::n59255(s, out),
		['ю', s @ ..] => super::t51::n59256(s, out),
		['я', s @ ..] => super::t51::n59257(s, out),
		_ => false,
	}
}

/// Words ending -нер-электрик.
pub(crate) fn n137996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136584);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139577);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 141336);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 142351);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 142928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гнетоэлектрик.
pub(crate) fn n137997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136585);
	s.is_empty()
}

/// Words ending -ик-семицветик.
pub(crate) fn n138000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136588);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139580);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141339);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 142353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкоэнергетик.
pub(crate) fn n138001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136589);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рнетик.
pub(crate) fn n59486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58866);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108540);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122876);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 108541);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 122877);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 131523);
									s.is_empty()
								}
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
			super::push(out, 86868);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108542);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ретик.
pub(crate) fn n31175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30940);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 58868);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 86870);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 86871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 58869);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 86872);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 58870);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 86873);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 108545);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122878);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131524);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136590);
													match s {
														[] => true,
														['з', s @ ..] => super::t51::n141022(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 86874);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 108546);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 122879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тетик.
pub(crate) fn n31176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30941);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 58871);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 86875);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 58872);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 86876);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108548);
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
				['е', 'н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108549);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 108550);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 122880);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 122881);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 86877);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ст-аналитик.
pub(crate) fn n138003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136591);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139583);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 141341);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 142355);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ист-практик.
pub(crate) fn n138004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136592);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139584);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141342);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -теллигентик.
pub(crate) fn n138005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136593);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139585);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пергаментик.
pub(crate) fn n138006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136594);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 139586);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нейролептик.
pub(crate) fn n138007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136595);
	s.is_empty()
}

/// Words ending -психолептик.
pub(crate) fn n138008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136596);
	s.is_empty()
}

/// Words ending -самонакладчик.
pub(crate) fn n138010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136598);
	s.is_empty()
}

/// Words ending -сопрокладчик.
pub(crate) fn n138011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136599);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139587);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 141343);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 142357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -топрокладчик.
pub(crate) fn n138012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136600);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139588);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141344);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 142358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -белеукладчик.
pub(crate) fn n138013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136601);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139589);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -убоукладчик.
pub(crate) fn n138014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136602);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139590);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141345);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 142359);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 142930);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 143236);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 143387);
													match s {
														[] => true,
														['с', s @ ..] => super::t51::n144908(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ведчик.
pub(crate) fn n59634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59010);
	match s {
		[] => true,
		['з', 'а', 'р', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 108627);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122929);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131544);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 136603);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139591);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n142792(s, out),
														_ => false,
													}
												}
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
							super::push(out, 122930);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131545);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136604);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 139592);
													s.is_empty()
												}
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
							super::push(out, 122931);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131546);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 136605);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139593);
													match s {
														[] => true,
														['м', s @ ..] => super::t51::n142793(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 108628);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 122932);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 131547);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136606);
											s.is_empty()
										}
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
					super::push(out, 108629);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одчик.
pub(crate) fn n31266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31023);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 59012);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86967);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 108630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 86968);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 108631);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122933);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 108632);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 122934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 86969);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108633);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 108634);
							match s {
								[] => true,
								['п', 'о', 'д', s @ ..] => {
									super::push(out, 122935);
									match s {
										[] => true,
										['о', 'в', '-', s @ ..] => {
											super::push(out, 131548);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 136607);
													match s {
														[] => true,
														['р', s @ ..] => super::t51::n141034(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 86970);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122936);
									s.is_empty()
								}
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
			super::push(out, 59013);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					super::push(out, 86971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 59014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядчик.
pub(crate) fn n31268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31025);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 59015);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 86972);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 86973);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 122937);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 131549);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136608);
													match s {
														[] => true,
														['в', s @ ..] => super::t51::n141035(s, out),
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
									super::push(out, 122938);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 131550);
											match s {
												[] => true,
												['с', '-', s @ ..] => {
													super::push(out, 136609);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n141036(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ция-заказчик.
pub(crate) fn n138022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139597);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 141351);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ор-рассказчик.
pub(crate) fn n138023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136611);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139598);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 141352);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -р-погрузчик.
pub(crate) fn n138024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136612);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139599);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141353);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142367);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 142933);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 143239);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 143388);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгрузчик.
pub(crate) fn n109806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108648);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 122947);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 131558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язчик.
pub(crate) fn n31278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31034);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 59028);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word пингвинчик.
pub(crate) fn n138025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136613);
	s.is_empty()
}

/// Words ending -осподинчик.
pub(crate) fn n132924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131566);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 136614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зинчик.
pub(crate) fn n59697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59071);
	match s {
		[] => true,
		['н', 'е', 'б', '\u{1}', s @ ..] => {
			super::push(out, 87018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дминчик.
pub(crate) fn n59698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59072);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87019);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108680);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ончик.
pub(crate) fn n31297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31051);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 59073);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 87020);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108681);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 122966);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131567);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 59074);
			s.is_empty()
		}
		['д', 'и', 'п', s @ ..] => {
			super::push(out, 59075);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 87021);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108682);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122967);
									s.is_empty()
								}
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
			super::push(out, 59076);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 87022);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108683);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122968);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131568);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136615);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59077);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 87023);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108684);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'п', s @ ..] => {
			super::push(out, 59078);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 87024);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108685);
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
				['а', 'р', s @ ..] => {
					super::push(out, 87025);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108686);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 87026);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108687);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 122971);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131569);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 59079);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 59080);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87027);
					s.is_empty()
				}
				['м', 'о', 'п', 'м', s @ ..] => {
					super::push(out, 87028);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108688);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 122972);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131570);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 59081);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 59082);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 59083);
			s.is_empty()
		}
		['ь', 'н', s @ ..] => {
			super::push(out, 59084);
			match s {
				[] => true,
				['и', 'м', s @ ..] => {
					super::push(out, 87029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унчик.
pub(crate) fn n31298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31052);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 59085);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87030);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 108690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 59086);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87031);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 59087);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 87032);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 108692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 87033);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108693);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122973);
									s.is_empty()
								}
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
			super::push(out, 59088);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 87034);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 87035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 87036);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108694);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 87037);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108695);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 122975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 87038);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108696);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 122976);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 87039);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108697);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 122977);
									s.is_empty()
								}
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
			super::push(out, 59089);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87040);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 59090);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 87041);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рянчик.
pub(crate) fn n31299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31053);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 59091);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 87042);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 108700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 122978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -репортерчик.
pub(crate) fn n138028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136616);
	s.is_empty()
}

/// Words ending -умуляторчик.
pub(crate) fn n138029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136617);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 139600);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141354);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-перехватчик.
pub(crate) fn n138030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136618);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139601);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141355);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142368);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 142934);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 143240);
											match s {
												[] => true,
												['с', '\u{1}', s @ ..] => {
													super::push(out, 143389);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ь-перехватчик.
pub(crate) fn n138031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136619);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139602);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141356);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142369);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 142935);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 143241);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143390);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 139603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141357);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 142370);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 142936);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 143242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ин-датчик.
pub(crate) fn n124267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122986);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 131577);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 136620);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139604);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141358);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 142371);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едатчик.
pub(crate) fn n87995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87058);
	match s {
		[] => true,
		['р', 'е', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 122987);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 122988);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 131578);
							s.is_empty()
						}
						['м', 'е', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 131579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здатчик.
pub(crate) fn n87996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87059);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 122989);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 122990);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 131580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 108714);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 122991);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 131581);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 136621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -модатчик.
pub(crate) fn n109875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108715);
	s.is_empty()
}

/// Words ending -родатчик.
pub(crate) fn n109876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108716);
	s.is_empty()
}

/// Words ending -сдатчик.
pub(crate) fn n87998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87061);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 108717);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 122992);
					match s {
						[] => true,
						['е', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 131582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 122993);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отдатчик.
pub(crate) fn n87999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87062);
	s.is_empty()
}

/// Words ending -катчик.
pub(crate) fn n59743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59115);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 87063);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108718);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122994);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131583);
									s.is_empty()
								}
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
			super::push(out, 87064);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108719);
					match s {
						[] => true,
						['п', 'о', 'с', 'е', 'л', 'о', 'к', '\u{1}', s @ ..] => {
							super::push(out, 122995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 87065);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108720);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матчик.
pub(crate) fn n59744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59116);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 87066);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -патчик.
pub(crate) fn n59745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59117);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 87067);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ратчик.
pub(crate) fn n59746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59118);
	s.is_empty()
}

/// Words ending -татчик.
pub(crate) fn n59747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59119);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 87068);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етчик.
pub(crate) fn n31315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31063);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 59120);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87069);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 108721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 87070);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 108722);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 122997);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 131584);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136622);
													match s {
														[] => true,
														['р', 'п', '\u{1}', s @ ..] => super::t51::n141045(s, out),
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
									super::push(out, 122998);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 122999);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 131585);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136623);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59121);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 59122);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87071);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 108723);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 123000);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131586);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136624);
													match s {
														[] => true,
														['ч', s @ ..] => super::t51::n141046(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 87072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 59123);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 87073);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 87074);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 87075);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 108724);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 123001);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131587);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136625);
													match s {
														[] => true,
														['г', s @ ..] => super::t51::n141047(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59124);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 59125);
			match s {
				[] => true,
				['е', 'б', 'о', 'н', 'е', 'л', 'е', 'з', '\u{1}', s @ ..] => {
					super::push(out, 87076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 59126);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 59127);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 59128);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87077);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 108725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 87078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108726);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 108727);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 108728);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 123002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 108729);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 123003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 87079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итчик.
pub(crate) fn n31316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31064);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 59129);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 59130);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87080);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 108730);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123004);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 131588);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136626);
													match s {
														[] => true,
														['н', s @ ..] => super::t51::n141048(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87081);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отчик.
pub(crate) fn n31317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31065);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 59132);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					super::push(out, 87082);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 108731);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123005);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 131589);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136627);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141049(s, out),
														_ => false,
													}
												}
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
							super::push(out, 108732);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 123006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 108733);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123007);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 136628);
													match s {
														[] => true,
														['р', s @ ..] => super::t51::n141050(s, out),
														['я', s @ ..] => super::t51::n141051(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 136629);
													s.is_empty()
												}
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
							super::push(out, 108734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 59133);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87083);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 108735);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 108736);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 123008);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 131590);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136630);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утчик.
pub(crate) fn n31318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31066);
	s.is_empty()
}

/// Words ending -ытчик.
pub(crate) fn n31319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31067);
	match s {
		[] => true,
		['б', 'о', 'д', 'о', 'з', s @ ..] => {
			super::push(out, 59135);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					super::push(out, 87084);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108737);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123009);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131591);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сятчик.
pub(crate) fn n31320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31068);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 59136);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 87085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108738);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ька-мальчик.
pub(crate) fn n138043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136631);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139612);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 141363);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142377);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 142940);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пай-мальчик.
pub(crate) fn n138044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136632);
	s.is_empty()
}

/// Words ending -рок-мальчик.
pub(crate) fn n138045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136633);
	s.is_empty()
}

/// Words ending -ригинальчик.
pub(crate) fn n138046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136634);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -к-с-пальчик.
pub(crate) fn n138047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136635);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139614);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 141364);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 142378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -крокодильчик.
pub(crate) fn n138048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136636);
	s.is_empty()
}

/// Words ending -омольчик.
pub(crate) fn n109908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108748);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 123017);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 131599);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 136637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-заправщик.
pub(crate) fn n138050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136638);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139615);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141365);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142379);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 142941);
									match s {
										[] => true,
										['а', 'с', '\u{1}', s @ ..] => {
											super::push(out, 143245);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -возаправщик.
pub(crate) fn n138051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136639);
	s.is_empty()
}

/// Words ending -лозаправщик.
pub(crate) fn n138052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136640);
	s.is_empty()
}

/// Words ending -тозаправщик.
pub(crate) fn n138053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136641);
	s.is_empty()
}

/// Words ending -доставщик.
pub(crate) fn n124311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123030);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 131607);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 136642);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139616);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 141366);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142380);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142942);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поставщик.
pub(crate) fn n124312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123031);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 131608);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 136643);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139617);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141367);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142381);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 142943);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 136644);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 139618);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 141368);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142382);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142944);
													match s {
														[] => true,
														['м', s @ ..] => super::t51::n144699(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 131609);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 136645);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 139619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -евщик.
pub(crate) fn n31348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31094);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 59169);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 87117);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 108764);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123032);
									s.is_empty()
								}
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
			super::push(out, 59170);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 87118);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 87119);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 108765);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 108766);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123034);
									s.is_empty()
								}
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
			super::push(out, 59171);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 87120);
					match s {
						[] => true,
						['д', 'о', 'н', s @ ..] => {
							super::push(out, 108767);
							match s {
								[] => true,
								['с', 'а', 'р', 'к', '-', s @ ..] => {
									super::push(out, 123035);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131610);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 136646);
													match s {
														[] => true,
														['л', s @ ..] => super::t51::n141060(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ивщик.
pub(crate) fn n31349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31095);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 59173);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87121);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 108768);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 123036);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131611);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 136647);
													match s {
														[] => true,
														['т', s @ ..] => super::t51::n141061(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59174);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 59175);
			match s {
				[] => true,
				['а', 'и', s @ ..] => {
					super::push(out, 87122);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 108769);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 123037);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 131612);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136648);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t51::n141062(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овщик.
pub(crate) fn n31350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31096);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 59176);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 87123);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108770);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 123038);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 87124);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108771);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 123039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 108772);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 123040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 87125);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108773);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 123041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 87126);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108774);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 108775);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 123042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 87127);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 108776);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 123043);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131613);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136649);
													match s {
														[] => true,
														['р', s @ ..] => super::t51::n141063(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 87128);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 108777);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 123044);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131614);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 59177);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87129);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108778);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 123045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 87130);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108779);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 123046);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 131615);
											s.is_empty()
										}
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
					super::push(out, 87131);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108780);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123047);
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
							super::push(out, 108781);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 123048);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131616);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 108782);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 108783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 59178);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 59179);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87133);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 108784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 87134);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108785);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 123049);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 87135);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108786);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 123050);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 87136);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 59180);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87137);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 108788);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 123051);
									match s {
										[] => true,
										['-', s @ ..] => super::t51::n132976(s, out),
										['о', s @ ..] => super::t51::n132977(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t51::n109949(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t51::n59809(s, out),
		['м', s @ ..] => {
			super::push(out, 59182);
			s.is_empty()
		}
		['н', s @ ..] => super::t51::n59811(s, out),
		['п', s @ ..] => super::t51::n59812(s, out),
		['р', s @ ..] => super::t51::n59813(s, out),
		['с', s @ ..] => super::t51::n59814(s, out),
		['т', s @ ..] => super::t51::n59815(s, out),
		['ф', s @ ..] => super::t51::n59816(s, out),
		['х', s @ ..] => super::t51::n59817(s, out),
		['ц', s @ ..] => super::t51::n59818(s, out),
		_ => false,
	}
}

/// Words ending -увщик.
pub(crate) fn n31351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31097);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 59191);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 108843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывщик.
pub(crate) fn n31352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31098);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 59192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -явщик.
pub(crate) fn n31353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31099);
	s.is_empty()
}

/// Words ending -тик-атомщик.
pub(crate) fn n138080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136667);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139640);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 141384);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word подкормщик.
pub(crate) fn n138081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136668);
	s.is_empty()
}

/// Words ending -линолеумщик.
pub(crate) fn n138082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136669);
	s.is_empty()
}

/// Words ending -к-вакуумщик.
pub(crate) fn n138083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136670);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139641);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 141385);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 142398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отливанщик.
pub(crate) fn n138084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136671);
	s.is_empty()
}

/// Words ending -подкаменщик.
pub(crate) fn n138085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136672);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 139642);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141386);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142399);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 142953);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 143252);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еменщик.
pub(crate) fn n88136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87193);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 108866);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131659);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 131660);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 136673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зменщик.
pub(crate) fn n88137(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 108867);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 123117);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 131661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 108868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уменщик.
pub(crate) fn n88140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87196);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 108870);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьменщик.
pub(crate) fn n88141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87197);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 108871);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ренщик.
pub(crate) fn n59846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59218);
	s.is_empty()
}

/// Words ending -тенщик.
pub(crate) fn n59847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59219);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 87198);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108872);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 123118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ценщик.
pub(crate) fn n59848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59220);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 87199);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инщик.
pub(crate) fn n31369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31115);
	s.is_empty()
}

/// Words ending -онщик.
pub(crate) fn n31370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31116);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 59221);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 87200);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 87201);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108873);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123119);
									s.is_empty()
								}
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
			super::push(out, 59222);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 59223);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 87202);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 108874);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123120);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131662);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136674);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59224);
			match s {
				[] => true,
				['р', 'а', 'г', '\u{1}', s @ ..] => {
					super::push(out, 87203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 59225);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87204);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 87205);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 108875);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 123121);
									match s {
										[] => true,
										['л', 'э', '-', s @ ..] => {
											super::push(out, 131663);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136675);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n141083(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унщик.
pub(crate) fn n31371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31117);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 59226);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 87206);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108876);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 123122);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131664);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янщик.
pub(crate) fn n31372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31118);
	s.is_empty()
}

/// Words ending -зик-ядерщик.
pub(crate) fn n138089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136676);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139644);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 141388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рг-лазерщик.
pub(crate) fn n138090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136677);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 139645);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141389);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орщик.
pub(crate) fn n31380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31126);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 59243);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87228);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 87229);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 108895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 87230);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 108896);
							match s {
								[] => true,
								['п', '-', s @ ..] => {
									super::push(out, 123133);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 131670);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136678);
													match s {
														[] => true,
														['е', s @ ..] => super::t51::n141086(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 87231);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 108897);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 123134);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131671);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136679);
													match s {
														[] => true,
														['с', s @ ..] => super::t51::n141087(s, out),
														_ => false,
													}
												}
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
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123135);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 123136);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 131672);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136680);
													match s {
														[] => true,
														['н', s @ ..] => super::t51::n141088(s, out),
														_ => false,
													}
												}
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
							super::push(out, 108898);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 123137);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131673);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 136681);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141089(s, out),
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
									super::push(out, 123138);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 123139);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 123140);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 131674);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136682);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 87232);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 87233);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 108899);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123141);
									s.is_empty()
								}
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
			super::push(out, 59244);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87234);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 108900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 87235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 59245);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 59246);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 59247);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 59248);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 59249);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87236);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108901);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 123142);
									s.is_empty()
								}
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
			super::push(out, 59250);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 87237);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 108902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 59251);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87238);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108903);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 123143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 87239);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 108904);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 123144);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 131675);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136683);
													s.is_empty()
												}
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
							super::push(out, 108905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 59252);
			match s {
				[] => true,
				['с', 'е', s @ ..] => {
					super::push(out, 87240);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 108906);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 123145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 87241);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 108907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t51::n59881(s, out),
		['ф', s @ ..] => {
			super::push(out, 59254);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отрщик.
pub(crate) fn n59883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59255);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 87248);
			match s {
				[] => true,
				['с', 'о', 'р', s @ ..] => {
					super::push(out, 108914);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 123149);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131677);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -турщик.
pub(crate) fn n59884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59256);
	s.is_empty()
}

/// Words ending -льщик.
pub(crate) fn n31387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31133);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 59260);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 87252);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 87253);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 108917);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 108918);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 108919);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 123152);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 131679);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136685);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141091(s, out),
														['с', s @ ..] => super::t51::n141092(s, out),
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
									super::push(out, 123153);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 131680);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136686);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t51::n141093(s, out),
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
									super::push(out, 123154);
									s.is_empty()
								}
								['ш', 'а', s @ ..] => {
									super::push(out, 123155);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131681);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136687);
													match s {
														[] => true,
														['о', s @ ..] => super::t51::n141094(s, out),
														_ => false,
													}
												}
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
							super::push(out, 108920);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 108921);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 108922);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 123156);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131682);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 136688);
													match s {
														[] => true,
														['к', s @ ..] => super::t51::n141095(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => super::t51::n124439(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 87254);
					s.is_empty()
				}
				['д', s @ ..] => super::t51::n88201(s, out),
				['ж', s @ ..] => {
					super::push(out, 87256);
					s.is_empty()
				}
				['з', s @ ..] => super::t51::n88203(s, out),
				['и', s @ ..] => {
					super::push(out, 87258);
					s.is_empty()
				}
				['к', s @ ..] => super::t51::n88205(s, out),
				['м', s @ ..] => super::t51::n88206(s, out),
				['н', s @ ..] => super::t51::n88207(s, out),
				['п', s @ ..] => super::t51::n88208(s, out),
				['р', s @ ..] => super::t51::n88209(s, out),
				['с', s @ ..] => super::t51::n88210(s, out),
				['т', s @ ..] => super::t51::n88211(s, out),
				['у', s @ ..] => super::t51::n88212(s, out),
				['х', s @ ..] => super::t51::n88213(s, out),
				['ш', s @ ..] => super::t51::n88214(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t51::n59889(s, out),
		['и', s @ ..] => super::t51::n59890(s, out),
		['л', s @ ..] => super::t51::n59891(s, out),
		['о', s @ ..] => super::t51::n59892(s, out),
		['у', s @ ..] => super::t51::n59893(s, out),
		['ы', s @ ..] => super::t51::n59894(s, out),
		['я', s @ ..] => super::t51::n59895(s, out),
		_ => false,
	}
}

/// The word пригородок.
pub(crate) fn n138123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136710);
	s.is_empty()
}

/// Words ending -село-поселок.
pub(crate) fn n141108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139668);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141405);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -латье-чулок.
pub(crate) fn n138127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136714);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 139669);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -полуребенок.
pub(crate) fn n138129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136716);
	s.is_empty()
}

/// Words ending -перепеленок.
pub(crate) fn n138130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136717);
	s.is_empty()
}

/// Words ending -ненок.
pub(crate) fn n31646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31391);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 59598);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 59599);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 87565);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 109137);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 123268);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131749);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пенок.
pub(crate) fn n31647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31392);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 59600);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87566);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 87567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 59601);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 87568);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 109138);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 123269);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 131750);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136718);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренок.
pub(crate) fn n31648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31393);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 59602);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 59603);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 59604);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87569);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 87570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 59605);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 87571);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 109139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 87572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 59606);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 87573);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 109141);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123270);
									s.is_empty()
								}
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
			super::push(out, 59607);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 59608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 87574);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сенок.
pub(crate) fn n31649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31394);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 59609);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 87575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 59610);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 59611);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 87576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 59612);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 59613);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тенок.
pub(crate) fn n31650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31395);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 59614);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 87577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 59615);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 59616);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 59617);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 87579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 59618);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 87580);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 109143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 59619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87581);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 109144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 87582);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 109145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 109146);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 109147);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 123271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 109148);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 123272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 87583);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 109149);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 123273);
									s.is_empty()
								}
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
			super::push(out, 59620);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 59621);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 59622);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 87585);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 109151);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 123274);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131751);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ценок.
pub(crate) fn n31652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31397);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 59623);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87586);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 109152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 87587);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 109153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченок.
pub(crate) fn n31653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31398);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 59624);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 87588);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 109154);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123275);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131752);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенок.
pub(crate) fn n31654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31399);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 59625);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 59626);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 87589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 109155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -москвичонок.
pub(crate) fn n138132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136719);
	s.is_empty()
}

/// Words ending -к-трескунок.
pub(crate) fn n138133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136720);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139670);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141406);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -к-свистунок.
pub(crate) fn n138134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136721);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139671);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141407);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word аудиорынок.
pub(crate) fn n138135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136722);
	s.is_empty()
}

/// Words ending -ототок.
pub(crate) fn n60528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59897);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 87800);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фоток.
pub(crate) fn n31850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31595);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 59898);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 87801);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 109262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хоток.
pub(crate) fn n31851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31596);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 59899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -владивосток.
pub(crate) fn n138139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136726);
	s.is_empty()
}

/// Words ending -росток.
pub(crate) fn n60547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59916);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 87820);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 87821);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 109275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 87822);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 109276);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 123331);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 109277);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 123332);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 131776);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136727);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139673);
													match s {
														[] => true,
														['ч', s @ ..] => super::t51::n142855(s, out),
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
									super::push(out, 131777);
									s.is_empty()
								}
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
			super::push(out, 87823);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 109278);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 123333);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131778);
									s.is_empty()
								}
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
			super::push(out, 87824);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 109279);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 123334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 87825);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 109280);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 123335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 87826);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 109281);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 123336);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 131779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 109282);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 123337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 87827);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 87828);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 109283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 87829);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 109284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вгусток.
pub(crate) fn n88778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87831);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 109286);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 123339);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кусток.
pub(crate) fn n60550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59919);
	s.is_empty()
}

/// The word соня-полчок.
pub(crate) fn n141114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139674);
	s.is_empty()
}

/// Words ending -тист-пастушок.
pub(crate) fn n142856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141410);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 142412);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -токаннибализм.
pub(crate) fn n138175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136762);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гиперреализм.
pub(crate) fn n138176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136763);
	s.is_empty()
}

/// Words ending -оригинализм.
pub(crate) fn n138177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136764);
	s.is_empty()
}

/// Words ending -транационализм.
pub(crate) fn n138178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136765);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 139694);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141418);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 142415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -безнационализм.
pub(crate) fn n138179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136766);
	s.is_empty()
}

/// Words ending -ралиберализм.
pub(crate) fn n124790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123507);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 131859);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 136767);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139695);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 141419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тализм.
pub(crate) fn n61626(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 88528);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 109650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 88529);
			s.is_empty()
		}
		['н', 'е', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 109651);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 123508);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131860);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 136768);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 139696);
													match s {
														[] => true,
														['н', s @ ..] => super::t51::n142866(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'а', s @ ..] => {
					super::push(out, 109652);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 123509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уализм.
pub(crate) fn n61627(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 88530);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 88531);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 88532);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -близм.
pub(crate) fn n33120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32830);
	match s {
		[] => true,
		['а', 'т', 'н', 'р', 'е', 'т', '\u{1}', s @ ..] => {
			super::push(out, 60979);
			s.is_empty()
		}
		['й', s @ ..] => {
			match s {
				[] => true,
				['е', 'т', 'н', 'р', 'е', 'т', '\u{1}', s @ ..] => {
					super::push(out, 88533);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 88534);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -телизм.
pub(crate) fn n33121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32831);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 60980);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -илизм.
pub(crate) fn n33122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32832);
	s.is_empty()
}

/// Words ending -ллизм.
pub(crate) fn n33123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32833);
	s.is_empty()
}

/// Words ending -олизм.
pub(crate) fn n33124(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 60981);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 60982);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 88535);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -булизм.
pub(crate) fn n33125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32834);
	match s {
		[] => true,
		['м', 'а', 'н', s @ ..] => {
			super::push(out, 60983);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зм-ленинизм.
pub(crate) fn n138182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136769);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139697);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141421);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 142417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -прогибиционизм.
pub(crate) fn n138183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136770);
	s.is_empty()
}

/// Words ending -антропоцентризм.
pub(crate) fn n138184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136771);
	s.is_empty()
}

/// Words ending -супремасизм.
pub(crate) fn n138185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136772);
	s.is_empty()
}

/// Words ending -рахроматизм.
pub(crate) fn n138186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136773);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139698);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141422);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тахроматизм.
pub(crate) fn n138187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136774);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139699);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141423);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 142419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трихроматизм.
pub(crate) fn n138188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136775);
	s.is_empty()
}

/// Words ending -постсоветизм.
pub(crate) fn n138189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136776);
	s.is_empty()
}

/// Words ending -перпатриотизм.
pub(crate) fn n138190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136777);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 139700);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141424);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерхпатриотизм.
pub(crate) fn n138191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136778);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139701);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141425);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -востизм.
pub(crate) fn n61682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61031);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 88571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -утизм.
pub(crate) fn n33163(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 61032);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 61033);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 88572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютизм.
pub(crate) fn n33164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32865);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 61034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нгрессвуман.
pub(crate) fn n138204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136791);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139711);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рыба-лоцман.
pub(crate) fn n138205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136792);
	s.is_empty()
}

/// Words ending -клуб-ресторан.
pub(crate) fn n138208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136795);
	s.is_empty()
}

/// Words ending -агон-ресторан.
pub(crate) fn n138209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136796);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ролльхеттан.
pub(crate) fn n138212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136799);
	s.is_empty()
}

/// Words ending -иномбудсмен.
pub(crate) fn n138215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136802);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 139716);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -архитриклин.
pub(crate) fn n138221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136808);
	s.is_empty()
}

/// Words ending -коктебельчанин.
pub(crate) fn n141160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139720);
	s.is_empty()
}

/// Words ending -вязьмитинин.
pub(crate) fn n138227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136814);
	s.is_empty()
}

/// Words ending -ифлеемлянин.
pub(crate) fn n138228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136815);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нянин.
pub(crate) fn n34590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34259);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 62362);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 89411);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 62363);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 89412);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110128);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 123728);
									s.is_empty()
								}
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
			super::push(out, 62364);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 89413);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110129);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 89414);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 89415);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 62365);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 89416);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 110132);
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
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110133);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 123730);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131958);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 136816);
													s.is_empty()
												}
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
							super::push(out, 110134);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 123731);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 131959);
											s.is_empty()
										}
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
					super::push(out, 89417);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 89418);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110135);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 123732);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131960);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 110136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 62366);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 89419);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110137);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 123733);
									s.is_empty()
								}
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
			super::push(out, 62367);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 89420);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 110138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орянин.
pub(crate) fn n63025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62369);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 89421);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 89422);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 110139);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 123734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 89423);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 89424);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тянин.
pub(crate) fn n34592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34261);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 62370);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 89425);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110140);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 123735);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 131961);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136817);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t51::n141162(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 62371);
			match s {
				[] => true,
				['в', 'а', s @ ..] => {
					super::push(out, 89426);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110141);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 123736);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 131962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фянин.
pub(crate) fn n34593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34262);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 62372);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 89427);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110142);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 123737);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 131963);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 62373);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 89428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьянин.
pub(crate) fn n34594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34263);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 62374);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 89429);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 62375);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 89430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -севдоэфедрин.
pub(crate) fn n138232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136819);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 139723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -радиожаргон.
pub(crate) fn n138236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136823);
	s.is_empty()
}

/// Words ending -биль-фургон.
pub(crate) fn n138237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136824);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139726);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 141434);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word платибелодон.
pub(crate) fn n138239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136826);
	s.is_empty()
}

/// Words ending -рамма-шпион.
pub(crate) fn n138242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136829);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 139727);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141435);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утник-шпион.
pub(crate) fn n138243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136830);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 139728);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141436);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -молет-шпион.
pub(crate) fn n138244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136831);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 139729);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141437);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 142425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -геликоприон.
pub(crate) fn n138245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136832);
	s.is_empty()
}

/// Words ending -роман-фельетон.
pub(crate) fn n141174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139734);
	s.is_empty()
}

/// Words ending -пикопланктон.
pub(crate) fn n138254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136841);
	s.is_empty()
}

/// Words ending -нанопланктон.
pub(crate) fn n138255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136842);
	s.is_empty()
}

/// Words ending -фитопланктон.
pub(crate) fn n138256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136843);
	s.is_empty()
}

/// Words ending -антаподотон.
pub(crate) fn n138257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136844);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139735);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -альс-бостон.
pub(crate) fn n138258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136845);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139736);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -орогомилово.
pub(crate) fn n138273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136860);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 139746);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ико-тырново.
pub(crate) fn n138274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136861);
	s.is_empty()
}

/// The word волшебство.
pub(crate) fn n138275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136862);
	s.is_empty()
}

/// Words ending -умовство.
pub(crate) fn n111686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110517);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 123940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новство.
pub(crate) fn n91024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90064);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 110519);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 123941);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132057);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 136863);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139747);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 110520);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 123942);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повство.
pub(crate) fn n91025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90065);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 110521);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 123943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровство.
pub(crate) fn n91026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90066);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 110522);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 123944);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 110523);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 123945);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совство.
pub(crate) fn n91027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90067);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 110524);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 123946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 110525);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товство.
pub(crate) fn n91028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90068);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 110526);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 123947);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 132058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 110527);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 110528);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 123948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 110529);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 123949);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 132059);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 136864);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 123950);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132060);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 136865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 123951);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132061);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 136866);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 123952);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 132062);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 136867);
									s.is_empty()
								}
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
			super::push(out, 110530);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 123953);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховство.
pub(crate) fn n91029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90069);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 110531);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 123954);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цовство.
pub(crate) fn n91030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90070);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 110532);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 123955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увство.
pub(crate) fn n63967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63306);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 90071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гство.
pub(crate) fn n35602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35261);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 63307);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 90072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 63308);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 90073);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110533);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 123956);
									s.is_empty()
								}
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
			super::push(out, 63309);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90074);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дство.
pub(crate) fn n35603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35262);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63310);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 63311);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 90075);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 110535);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 123957);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 132064);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136868);
													match s {
														[] => true,
														['с', s @ ..] => super::t51::n141188(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 90076);
					s.is_empty()
				}
				['р', 'с', s @ ..] => {
					super::push(out, 90077);
					match s {
						[] => true,
						['о', 'п', s @ ..] => {
							super::push(out, 110536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 63312);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 90078);
					match s {
						[] => true,
						['о', 'т', 'е', s @ ..] => {
							super::push(out, 110537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 90079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 110538);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 110539);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 110540);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 123958);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132065);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 110541);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 110542);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123959);
									s.is_empty()
								}
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
			super::push(out, 63313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ество.
pub(crate) fn n35604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35263);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 63314);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 90080);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110543);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 123960);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 123961);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132066);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 123962);
									s.is_empty()
								}
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
			super::push(out, 63315);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 90081);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 90082);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110544);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 123963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 90083);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 110545);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123964);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 123965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 110546);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 110547);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 123966);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 132067);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136869);
													match s {
														[] => true,
														['т', s @ ..] => super::t51::n141189(s, out),
														_ => false,
													}
												}
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
							super::push(out, 110548);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 110549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 90084);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110550);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 123968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 90085);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 110551);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 110552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 90086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 63316);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 90087);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110553);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 123969);
									s.is_empty()
								}
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
			super::push(out, 63317);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 90088);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 110554);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 123970);
									s.is_empty()
								}
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
			super::push(out, 63318);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 90089);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t51::n63980(s, out),
		['ш', s @ ..] => {
			super::push(out, 63320);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90105);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 90106);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 110586);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 124013);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132115);
											s.is_empty()
										}
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
					super::push(out, 90107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 63321);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 90108);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 90109);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 110588);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124014);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 132116);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136920);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 90110);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 110589);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124015);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 132117);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136921);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 90111);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 110590);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 110591);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 110592);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 90112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жство.
pub(crate) fn n35605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35264);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 63322);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 90113);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 90114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 110593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йство.
pub(crate) fn n35606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35265);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63323);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 90115);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 110594);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124019);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 132118);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 136922);
													match s {
														[] => true,
														['а', s @ ..] => super::t51::n141238(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 90116);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 110595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 90117);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110596);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 124020);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 132119);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 136923);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132120);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 132121);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136924);
													match s {
														[] => true,
														['д', s @ ..] => super::t51::n141239(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 63324);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 90118);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110597);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 124021);
									s.is_empty()
								}
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
			super::push(out, 63325);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 63326);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 90119);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 90120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 63327);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 90121);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110599);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 90122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 110600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 63328);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кство.
pub(crate) fn n35607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35266);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 63329);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мство.
pub(crate) fn n35608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35267);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63330);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 63331);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 63332);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 63333);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 90123);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110601);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 124022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 110602);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124023);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 132122);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136925);
													s.is_empty()
												}
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
							super::push(out, 110603);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 124024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 110604);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 110605);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 124026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 90124);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 90125);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 63334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нство.
pub(crate) fn n35609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35268);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63335);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 90126);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 90127);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110608);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 124027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 110609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 90128);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110610);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124028);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 132123);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 110611);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 90129);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 110612);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124030);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 132124);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136926);
													match s {
														[] => true,
														['м', s @ ..] => super::t51::n141240(s, out),
														_ => false,
													}
												}
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
							super::push(out, 110613);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 110614);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['а', 'т', 'е', s @ ..] => {
									super::push(out, 124031);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 132125);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136927);
													match s {
														[] => true,
														['в', 'о', s @ ..] => super::t51::n141241(s, out),
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
									super::push(out, 124032);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 132126);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136928);
													match s {
														[] => true,
														['т', s @ ..] => super::t51::n141242(s, out),
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
									super::push(out, 124033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 110615);
							s.is_empty()
						}
						['т', 'с', s @ ..] => {
							super::push(out, 110616);
							match s {
								[] => true,
								['и', 'р', 'х', 'о', s @ ..] => {
									super::push(out, 124034);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 124035);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 132127);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 136929);
													s.is_empty()
												}
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
							super::push(out, 110617);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 124036);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 132128);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136930);
													match s {
														[] => true,
														['д', s @ ..] => super::t51::n141243(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 90130);
					match s {
						[] => true,
						['и', 'л', s @ ..] => {
							super::push(out, 110618);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 124037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 90131);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110619);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 110620);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 110621);
							match s {
								[] => true,
								['у', 'т', s @ ..] => {
									super::push(out, 124038);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 132129);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136931);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t51::n111794(s, out),
						['ц', s @ ..] => super::t51::n111795(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 90132);
					match s {
						[] => true,
						['т', s @ ..] => super::t51::n111796(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t51::n91094(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t51::n63997(s, out),
		['и', s @ ..] => super::t51::n63998(s, out),
		['о', s @ ..] => super::t51::n63999(s, out),
		['у', s @ ..] => super::t51::n64000(s, out),
		['я', s @ ..] => super::t51::n64001(s, out),
		_ => false,
	}
}

/// Words ending -оство.
pub(crate) fn n35610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35269);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 63341);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90156);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пство.
pub(crate) fn n35611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35270);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63342);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 90157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 63343);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 63344);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 90158);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 90159);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 110663);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 124072);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 132156);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 136949);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 90160);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', 'к', 'с', s @ ..] => {
			super::push(out, 63345);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 90161);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 110665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124073);
									s.is_empty()
								}
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
			super::push(out, 63346);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 90162);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рство.
pub(crate) fn n35612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35271);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 63347);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 90163);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 90164);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110666);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 124074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 110667);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 124075);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 132157);
											s.is_empty()
										}
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
					super::push(out, 90165);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110668);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124076);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 132158);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136950);
													match s {
														[] => true,
														['н', s @ ..] => super::t51::n141249(s, out),
														_ => false,
													}
												}
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
							super::push(out, 110669);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 124077);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 132159);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136951);
													s.is_empty()
												}
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
							super::push(out, 110670);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 124078);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 132160);
											match s {
												[] => true,
												['г', '-', s @ ..] => {
													super::push(out, 136952);
													match s {
														[] => true,
														['д', s @ ..] => super::t51::n141250(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 90166);
					match s {
						[] => true,
						['е', 'п', s @ ..] => {
							super::push(out, 110671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 90167);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110672);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 124080);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 132161);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 110673);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 124081);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 132162);
											s.is_empty()
										}
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
					super::push(out, 90168);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 110674);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 124082);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 132163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 110675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 90169);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 110676);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 124083);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 132164);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 110677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 124084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 90170);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 110678);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 110679);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 124085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 90171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 63348);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 90172);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 110680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 63349);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 90173);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 110681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 90174);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 110682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t51::n64011(s, out),
		['и', s @ ..] => {
			super::push(out, 63351);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 90194);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 90195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t51::n64013(s, out),
		['у', s @ ..] => {
			super::push(out, 63353);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 90209);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 90210);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 110762);
							s.is_empty()
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
				['н', s @ ..] => {
					super::push(out, 90211);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 110763);
							match s {
								[] => true,
								['р', s @ ..] => super::t51::n125447(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t51::n91179(s, out),
				_ => false,
			}
		}
		['э', s @ ..] => super::t51::n64016(s, out),
		['я', s @ ..] => super::t51::n64017(s, out),
		_ => false,
	}
}

/// Words ending -сство.
pub(crate) fn n35613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35272);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 63356);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 90215);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

