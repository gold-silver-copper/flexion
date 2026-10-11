use flexion::Path;

/// Words ending -она.
pub(crate) fn n815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 814);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6130);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 6131);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 6132);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23279);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 50432);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79246);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102730);
											s.is_empty()
										}
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
					super::push(out, 23280);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 50433);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23281);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50434);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 79248);
									s.is_empty()
								}
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
			super::push(out, 6133);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23282);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50435);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79249);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 102731);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 119116);
													match s {
														[] => true,
														['о', s @ ..] => super::t32::n130707(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23283);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79250);
									s.is_empty()
								}
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
			super::push(out, 6134);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6135);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23285);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50437);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79251);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 102732);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 119117);
													match s {
														[] => true,
														['о', s @ ..] => super::t32::n130708(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23286);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50438);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79252);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 102733);
											s.is_empty()
										}
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
					super::push(out, 23287);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 23288);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 50439);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79253);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102734);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6136);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23289);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 23290);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50440);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79254);
									s.is_empty()
								}
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
			super::push(out, 6137);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23291);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23293);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 50443);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 50444);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6138);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23294);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 23295);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50445);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79256);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23296);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50446);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23297);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50447);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 79258);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102735);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 119118);
													match s {
														[] => true,
														['п', s @ ..] => super::t32::n130709(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23298);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50448);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79259);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102736);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6139);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 23299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50449);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6140);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23300);
					match s {
						[] => true,
						['з', s @ ..] => super::t32::n50965(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t32::n23440(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t32::n6157(s, out),
		['с', s @ ..] => super::t32::n6158(s, out),
		['т', s @ ..] => super::t32::n6159(s, out),
		['ф', s @ ..] => super::t32::n6160(s, out),
		['х', s @ ..] => super::t32::n6161(s, out),
		['ц', s @ ..] => super::t32::n6162(s, out),
		['ш', s @ ..] => super::t32::n6163(s, out),
		['ь', s @ ..] => super::t32::n6164(s, out),
		_ => false,
	}
}

/// Words ending -пна.
pub(crate) fn n816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 815);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6149);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рна.
pub(crate) fn n817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 816);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6150);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23314);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 23315);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 23316);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50463);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 79273);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102743);
											s.is_empty()
										}
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
					super::push(out, 23317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6151);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23318);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50465);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79274);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 50466);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79275);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102744);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 50467);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23319);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23320);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50469);
							match s {
								[] => true,
								['и', 'ц', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 102745);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 119123);
													match s {
														[] => true,
														['о', s @ ..] => super::t32::n130712(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 119124);
													match s {
														[] => true,
														['о', s @ ..] => super::t32::n130713(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 119125);
													match s {
														[] => true,
														['е', s @ ..] => super::t32::n130714(s, out),
														_ => false,
													}
												}
												['ь', s @ ..] => {
													super::push(out, 119126);
													match s {
														[] => true,
														['л', s @ ..] => super::t32::n130715(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 102746);
											s.is_empty()
										}
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
					super::push(out, 23321);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 50470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6152);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23322);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6153);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23323);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6154);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23325);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23326);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сна.
pub(crate) fn n818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 817);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6155);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23328);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50475);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79277);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102747);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6156);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23329);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23330);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 23331);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50477);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79278);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102748);
											s.is_empty()
										}
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
					super::push(out, 23332);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 50478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6157);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6158);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23334);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50480);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 6159);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23336);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50481);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 79280);
									s.is_empty()
								}
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
			super::push(out, 6160);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23337);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тна.
pub(crate) fn n819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 818);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 6161);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23338);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6162);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23339);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 6163);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уна.
pub(crate) fn n820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 819);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6164);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6165);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23340);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 23341);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50485);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 50486);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79281);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102749);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 119127);
													match s {
														[] => true,
														['и', s @ ..] => super::t32::n130716(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6166);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23342);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50487);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 79282);
									s.is_empty()
								}
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
			super::push(out, 6167);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23343);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23344);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50488);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6168);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23345);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50489);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79283);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102750);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119128);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6169);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23346);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6170);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23348);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23349);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 50491);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79284);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102751);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 50492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23350);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 50493);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79285);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102752);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 119129);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6171);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23351);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50494);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79286);
									s.is_empty()
								}
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
			super::push(out, 6172);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6173);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6174);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23353);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50495);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79287);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 102753);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119130);
													match s {
														[] => true,
														['к', s @ ..] => super::t32::n130717(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23354);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50496);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 79288);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 102754);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6175);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23355);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50497);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79289);
									s.is_empty()
								}
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
			super::push(out, 6176);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23356);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50498);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 79290);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23357);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 50499);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79291);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102755);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119131);
													match s {
														[] => true,
														['с', s @ ..] => super::t32::n130718(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6177);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 23358);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 6178);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23359);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 50500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фна.
pub(crate) fn n821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 820);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6179);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23360);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хна.
pub(crate) fn n822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6180);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23361);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50502);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6181);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23363);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6182);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6183);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 23364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чна.
pub(crate) fn n823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 822);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6184);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23365);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23366);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50507);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шна.
pub(crate) fn n824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 823);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6185);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23367);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 23368);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50508);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 79294);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102756);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119132);
													match s {
														[] => true,
														['б', s @ ..] => super::t32::n130719(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6186);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23369);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ына.
pub(crate) fn n825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 824);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6187);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23370);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 50509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6188);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23371);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 50510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6189);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6190);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23372);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50511);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 79295);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102757);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 119133);
													match s {
														[] => true,
														['п', s @ ..] => super::t32::n130720(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьна.
pub(crate) fn n826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 825);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6191);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23373);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 50512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юна.
pub(crate) fn n827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 826);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6192);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6193);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6194);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яна.
pub(crate) fn n828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 827);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6195);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6196);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23376);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50514);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 79296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 23377);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50515);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79297);
									s.is_empty()
								}
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
			super::push(out, 6197);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23378);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50516);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79298);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 102758);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6198);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23379);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23380);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 50517);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23381);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6199);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 23382);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6200);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23383);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50520);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 6201);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23384);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50521);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79299);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102759);
											s.is_empty()
										}
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
						['и', s @ ..] => {
							super::push(out, 50522);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79300);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102760);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 119134);
													s.is_empty()
												}
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
							super::push(out, 50523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50524);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 50525);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79302);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102761);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 79303);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102762);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 119135);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23385);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 23386);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50527);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -глазенапа.
pub(crate) fn n120398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119136);
	s.is_empty()
}

/// The word растрепа.
pub(crate) fn n120399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119137);
	s.is_empty()
}

/// The word гольтепа.
pub(crate) fn n120400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119138);
	s.is_empty()
}

/// Words ending -типа.
pub(crate) fn n6249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6233);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 23413);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23414);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щипа.
pub(crate) fn n6250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6234);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23415);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23416);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 50545);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лпа.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 840);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6235);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23417);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мпа.
pub(crate) fn n842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 841);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6236);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -опа.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 842);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 6238);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6239);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6240);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23419);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50546);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79317);
									match s {
										[] => true,
										['к', '-', s @ ..] => {
											super::push(out, 102770);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 119139);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6241);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23420);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 23421);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50547);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79318);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23422);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50548);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6242);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23424);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 50550);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79320);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102771);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6243);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6244);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6245);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23426);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 23427);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50552);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 50553);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 50554);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79321);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 102772);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119140);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 79322);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102773);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 50555);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 50556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6247);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23430);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ппа.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 843);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6248);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6249);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23432);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50559);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79323);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102774);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119141);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23433);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50560);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79324);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102775);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119142);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6250);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рпа.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 844);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6251);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6252);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23435);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 50561);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79325);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102776);
											s.is_empty()
										}
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
					super::push(out, 23436);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23437);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 50562);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79326);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 79327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 23438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6253);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 23439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -спа.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 845);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 6254);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 23440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6255);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -упа.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 846);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6256);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6257);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 6258);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 6259);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6260);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23441);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 50564);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 50565);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 50566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23442);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23443);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50568);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79329);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102777);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6261);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6262);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23445);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50570);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыпа.
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 847);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 6263);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23446);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьпа.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 848);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -япа.
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 849);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6265);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 23447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6266);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6267);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23448);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50571);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бара.
pub(crate) fn n6286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6270);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23450);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 50573);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 50574);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79333);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 50575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 23451);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50576);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 79335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23452);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 50577);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79336);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 50578);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23453);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50579);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79337);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102780);
									s.is_empty()
								}
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
			super::push(out, 23454);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 50580);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 50581);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23455);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50582);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79340);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102781);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 102782);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 119143);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 129369);
													match s {
														[] => true,
														['н', s @ ..] => super::t32::n136864(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -арвара.
pub(crate) fn n51098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50583);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 79341);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 102783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усвара.
pub(crate) fn n51099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50584);
	s.is_empty()
}

/// Words ending -гара.
pub(crate) fn n6288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6272);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23458);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 50585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 23459);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 50586);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 79342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23460);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 50587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23461);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23462);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23463);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 50590);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дара.
pub(crate) fn n6289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6273);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23464);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 23465);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50591);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жара.
pub(crate) fn n6290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23466);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23467);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 50592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23468);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50593);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зара.
pub(crate) fn n6291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6275);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иара.
pub(crate) fn n6292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6276);
	s.is_empty()
}

/// Words ending -кара.
pub(crate) fn n6293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6277);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 23470);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50594);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 79348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23471);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 50595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 23472);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50596);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 79349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 23473);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50597);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23474);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50598);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79351);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 79352);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 79353);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102786);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 119144);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23475);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 50599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 23476);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 50600);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79354);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 102787);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 119145);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23477);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50601);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79355);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 79356);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 50602);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 50603);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 79358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лара.
pub(crate) fn n6294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6278);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 23478);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мара.
pub(crate) fn n6295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6279);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23479);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50604);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 50605);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 79359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 50606);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79360);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 50607);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 50608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 23480);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23481);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50610);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23482);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 23483);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50612);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нара.
pub(crate) fn n6296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6280);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 23484);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 50613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 50614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23485);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50615);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102790);
									s.is_empty()
								}
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
			super::push(out, 23486);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 50616);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79365);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 102791);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 79366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оара.
pub(crate) fn n6297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6281);
	s.is_empty()
}

/// Words ending -пара.
pub(crate) fn n6298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6282);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 23487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рара.
pub(crate) fn n6299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6283);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23488);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23489);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 50618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сара.
pub(crate) fn n6300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6284);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23490);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 23491);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50619);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23492);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50620);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тара.
pub(crate) fn n6301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6285);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23493);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 50621);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 79369);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102792);
									s.is_empty()
								}
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
			super::push(out, 23494);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 50622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уара.
pub(crate) fn n6302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6286);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 23496);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 50623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фара.
pub(crate) fn n6303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6287);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 23497);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50624);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79370);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102793);
									s.is_empty()
								}
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
			super::push(out, 23498);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хара.
pub(crate) fn n6304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6288);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23499);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 23500);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50626);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23501);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цара.
pub(crate) fn n6305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6289);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23502);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чара.
pub(crate) fn n6306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6290);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23503);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23504);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50628);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79373);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102794);
									s.is_empty()
								}
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
			super::push(out, 23505);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23506);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 23507);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 50631);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 79374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23508);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23509);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 50633);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 50634);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 50635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 23510);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 50636);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шара.
pub(crate) fn n6307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6291);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 23511);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 23512);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 50637);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 50638);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 23513);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 50639);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79378);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 102795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бра.
pub(crate) fn n853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 852);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6292);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23514);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 23515);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 23516);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 23517);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50641);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79379);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 102796);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 119146);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6293);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23519);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 23520);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50643);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50644);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 50645);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79381);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102797);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 119147);
													match s {
														[] => true,
														['р', s @ ..] => super::t32::n130722(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6294);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 6295);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23522);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23523);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50646);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79382);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6296);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6297);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23526);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вра.
pub(crate) fn n854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 853);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6298);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6299);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23527);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 23528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гра.
pub(crate) fn n855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 854);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 6300);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23529);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 50650);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 79383);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102798);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6301);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23530);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50651);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23531);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50652);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79385);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23532);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50653);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 79386);
									s.is_empty()
								}
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
			super::push(out, 6302);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23533);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 50654);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79387);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102799);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6303);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 23534);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6304);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6305);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6306);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23538);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50656);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79388);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102800);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6307);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23539);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дра.
pub(crate) fn n856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 855);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6308);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23540);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50658);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 50659);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 79389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23543);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 23544);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6309);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23545);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50663);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79390);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102801);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119148);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23546);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23547);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50665);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 79391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 23548);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50666);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 50667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 23549);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 23550);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6310);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23551);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50669);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23552);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 50670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6311);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23553);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23554);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50671);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79394);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102802);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6312);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23555);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50672);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79395);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102803);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119149);
													match s {
														[] => true,
														['с', s @ ..] => super::t32::n130723(s, out),
														_ => false,
													}
												}
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
							super::push(out, 50673);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79396);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102804);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 119150);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', 'а', 'к', '\u{1}', s @ ..] => {
									super::push(out, 79397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 50674);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79398);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102805);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 119151);
													s.is_empty()
												}
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
							super::push(out, 50675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23556);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50676);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 79400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 50677);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79401);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102806);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 79402);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102807);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 50678);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79403);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 102808);
											s.is_empty()
										}
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
					super::push(out, 23557);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23558);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 50679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t32::n23700(s, out),
				['я', s @ ..] => {
					super::push(out, 23560);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 50682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t32::n6329(s, out),
		['ы', s @ ..] => super::t32::n6330(s, out),
		_ => false,
	}
}

/// Words ending -ера.
pub(crate) fn n857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 856);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6315);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6316);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23567);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6317);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 6318);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23568);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50688);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23569);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 50689);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79408);
									s.is_empty()
								}
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
			super::push(out, 6319);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23570);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50690);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23571);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50691);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 50692);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 79411);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102811);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119153);
													match s {
														[] => true,
														['п', s @ ..] => super::t32::n130724(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6320);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23572);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23573);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 23574);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50695);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79412);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102812);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6321);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23575);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 50696);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79413);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102813);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 119154);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6322);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23576);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23577);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 50697);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 50698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23578);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23579);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 50700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6323);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23580);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50701);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 79414);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102814);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 119155);
													match s {
														[] => true,
														['е', s @ ..] => super::t32::n130725(s, out),
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
									super::push(out, 79415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23581);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 50702);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79416);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 50703);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79417);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23582);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50704);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 50705);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t32::n51221(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t32::n23724(s, out),
				['у', s @ ..] => super::t32::n23725(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t32::n6340(s, out),
		['п', s @ ..] => super::t32::n6341(s, out),
		['р', s @ ..] => super::t32::n6342(s, out),
		['с', s @ ..] => super::t32::n6343(s, out),
		['т', s @ ..] => super::t32::n6344(s, out),
		['ф', s @ ..] => super::t32::n6345(s, out),
		['х', s @ ..] => {
			super::push(out, 6330);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 23601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t32::n6347(s, out),
		['ш', s @ ..] => super::t32::n6348(s, out),
		['щ', s @ ..] => super::t32::n6349(s, out),
		['ь', s @ ..] => super::t32::n6350(s, out),
		_ => false,
	}
}

/// Words ending -жра.
pub(crate) fn n858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 857);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6335);
			match s {
				[] => true,
				['и', 'х', s @ ..] => {
					super::push(out, 23610);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зра.
pub(crate) fn n859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 858);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6336);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 23611);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ира.
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 859);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6337);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6338);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23612);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 23613);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6339);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23615);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23616);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50735);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79443);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102829);
											s.is_empty()
										}
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
					super::push(out, 23617);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50736);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79444);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 79445);
									s.is_empty()
								}
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
			super::push(out, 6340);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23618);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6341);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23619);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23620);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 50738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23621);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50739);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23622);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6342);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23623);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50741);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79447);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102830);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6343);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23624);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23625);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 50742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6344);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23626);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23627);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23628);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 50744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6345);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 6346);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23629);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23630);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 23631);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23632);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 23633);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50745);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79448);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102831);
											s.is_empty()
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 79449);
									s.is_empty()
								}
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
			super::push(out, 6347);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23634);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50746);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79450);
									s.is_empty()
								}
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
			super::push(out, 6348);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23635);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6349);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23636);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50748);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6350);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23637);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23638);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50750);
							match s {
								[] => true,
								['в', 'к', '-', s @ ..] => {
									super::push(out, 79451);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102832);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119166);
													match s {
														[] => true,
														['т', s @ ..] => super::t32::n130733(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['й', s @ ..] => {
											super::push(out, 102833);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119167);
													match s {
														[] => true,
														['з', s @ ..] => super::t32::n130734(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 102834);
											match s {
												[] => true,
												['и', s @ ..] => super::t32::n120430(s, out),
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
							super::push(out, 50751);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => super::t32::n6367(s, out),
		['х', s @ ..] => super::t32::n6368(s, out),
		['ц', s @ ..] => super::t32::n6369(s, out),
		['ш', s @ ..] => super::t32::n6370(s, out),
		_ => false,
	}
}

/// Words ending -йра.
pub(crate) fn n861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 860);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6355);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23644);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23645);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 23646);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 50758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6356);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23647);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50759);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23648);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50760);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23649);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50761);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79457);
									s.is_empty()
								}
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
			super::push(out, 6357);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23650);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 6358);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23652);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кра.
pub(crate) fn n862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 861);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6359);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 23653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6360);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23654);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6361);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6362);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23655);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мра.
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 862);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6363);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6364);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ора.
pub(crate) fn n864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 863);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6365);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 6366);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23658);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 50765);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79458);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102835);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6367);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23659);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23660);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50766);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79459);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102836);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119169);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 79460);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 102837);
											s.is_empty()
										}
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
					super::push(out, 23661);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50767);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23662);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50768);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23663);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50769);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 79463);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 50770);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 79464);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23664);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6368);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23666);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50771);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23667);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23668);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50772);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 79466);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102838);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 50773);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79467);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102839);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 50774);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 23669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23670);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 23671);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 50776);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79469);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 102840);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119170);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23672);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50777);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 79470);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 102841);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119171);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23673);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23674);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6370);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23675);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23676);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50780);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23677);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 50781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50782);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79472);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 50783);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79473);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'а', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50784);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 50785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23678);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6371);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23679);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23680);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 50787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => super::t32::n23824(s, out),
				['и', s @ ..] => {
					super::push(out, 23682);
					s.is_empty()
				}
				['о', s @ ..] => super::t32::n23826(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t32::n6388(s, out),
		['к', s @ ..] => super::t32::n6389(s, out),
		['л', s @ ..] => super::t32::n6390(s, out),
		['м', s @ ..] => super::t32::n6391(s, out),
		['н', s @ ..] => super::t32::n6392(s, out),
		['п', s @ ..] => super::t32::n6393(s, out),
		['р', s @ ..] => super::t32::n6394(s, out),
		['с', s @ ..] => super::t32::n6395(s, out),
		['т', s @ ..] => super::t32::n6396(s, out),
		['ф', s @ ..] => super::t32::n6397(s, out),
		['х', s @ ..] => super::t32::n6398(s, out),
		['ч', s @ ..] => super::t32::n6399(s, out),
		['ш', s @ ..] => super::t32::n6400(s, out),
		['ь', s @ ..] => super::t32::n6401(s, out),
		_ => false,
	}
}

/// Words ending -пра.
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 864);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6386);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23726);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6387);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23728);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рра.
pub(crate) fn n866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 865);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6388);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23729);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50844);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79513);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 102871);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119189);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6389);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23731);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50846);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79514);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102872);
											s.is_empty()
										}
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
					super::push(out, 23732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тра.
pub(crate) fn n867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 866);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6391);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23733);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50848);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23734);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 50849);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23735);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50850);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23736);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50851);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79518);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102873);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119190);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6392);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23738);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 50852);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79519);
									s.is_empty()
								}
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
			super::push(out, 6393);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23739);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23740);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50853);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 79520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 50854);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79521);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102874);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 50855);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79522);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102875);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119191);
													match s {
														[] => true,
														['л', s @ ..] => super::t32::n130744(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23741);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 50856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6394);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23742);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 23743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50857);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 50858);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79523);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102876);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 119192);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n130745(s, out),
														_ => false,
													}
												}
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
							super::push(out, 50859);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 50860);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 50861);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79526);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102877);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119193);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23744);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 23745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6395);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23746);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50862);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 79527);
									s.is_empty()
								}
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
			super::push(out, 6396);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23747);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23748);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50864);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79528);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 23749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6397);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 6398);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6399);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 23751);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50865);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79529);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 102878);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 119194);
													match s {
														[] => true,
														['о', s @ ..] => super::t32::n130746(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t32::n6416(s, out),
		['у', s @ ..] => {
			super::push(out, 6401);
			match s {
				[] => true,
				['п', s @ ..] => super::t32::n23901(s, out),
				['с', s @ ..] => {
					super::push(out, 23759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t32::n6418(s, out),
		['ь', s @ ..] => super::t32::n6419(s, out),
		_ => false,
	}
}

/// Words ending -ура.
pub(crate) fn n868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 867);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6404);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23764);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23765);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50877);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 50878);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 79541);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 102884);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 119199);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23766);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6406);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23767);
					match s {
						[] => true,
						['ф', '-', s @ ..] => {
							super::push(out, 50880);
							match s {
								[] => true,
								['п', 'о', 'п', s @ ..] => {
									super::push(out, 79542);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102885);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23768);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23769);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6408);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23771);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23772);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50881);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79543);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102886);
											s.is_empty()
										}
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
					super::push(out, 23773);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50882);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6409);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23774);
					match s {
						[] => true,
						['е', 'ц', s @ ..] => {
							super::push(out, 50883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23775);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50884);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79544);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102887);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119200);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6410);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23776);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50885);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79545);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102888);
											s.is_empty()
										}
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
					super::push(out, 23777);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79546);
									s.is_empty()
								}
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
			super::push(out, 6411);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23778);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23779);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50887);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 50888);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23780);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50889);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79549);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23781);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 50890);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79550);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 102889);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119201);
													match s {
														[] => true,
														['р', s @ ..] => super::t32::n130750(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23783);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23784);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50891);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 50892);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 50893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6413);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23786);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 50894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79551);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23787);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50895);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79552);
									s.is_empty()
								}
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
			super::push(out, 6414);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23788);
					match s {
						[] => true,
						['е', 'т', '\u{1}', s @ ..] => {
							super::push(out, 50896);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 50897);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 79553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t32::n23932(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t32::n6431(s, out),
		['т', s @ ..] => super::t32::n6432(s, out),
		['ф', s @ ..] => super::t32::n6433(s, out),
		['ч', s @ ..] => super::t32::n6434(s, out),
		['ш', s @ ..] => super::t32::n6435(s, out),
		_ => false,
	}
}

/// Words ending -фра.
pub(crate) fn n869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 868);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6420);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6421);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 23805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6422);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хра.
pub(crate) fn n870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 869);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6423);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23807);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 23808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6424);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23809);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50917);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79569);
									s.is_empty()
								}
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
			super::push(out, 6425);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6426);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23811);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6427);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6428);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 23813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6429);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыра.
pub(crate) fn n871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 870);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6431);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23815);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50920);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79570);
									s.is_empty()
								}
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
			super::push(out, 6432);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23816);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50921);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79571);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 102899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 50922);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79572);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102900);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119208);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6433);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 23817);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 50923);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79573);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102901);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119209);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6434);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 23818);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 50924);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79574);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 102902);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эра.
pub(crate) fn n872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 871);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6435);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 6436);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23819);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50925);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79575);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102903);
											s.is_empty()
										}
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
					super::push(out, 23820);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6437);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23821);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50927);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79576);
									s.is_empty()
								}
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
			super::push(out, 6438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юра.
pub(crate) fn n873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 872);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6439);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 6440);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6441);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23823);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6442);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6443);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 6444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яра.
pub(crate) fn n874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 873);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6445);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23825);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 50928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6446);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23826);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23827);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50930);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 50931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6447);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23828);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50932);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79577);
									s.is_empty()
								}
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
			super::push(out, 6448);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23829);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6449);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23830);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 50934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23831);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50935);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 50936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 23832);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 23833);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 50937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6450);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23834);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 50938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23835);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50939);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79578);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 79579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 50940);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23836);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50941);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 50942);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 79581);
									s.is_empty()
								}
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
			super::push(out, 6451);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 23837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 6452);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23838);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23839);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 50943);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 79582);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102904);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 119210);
													match s {
														[] => true,
														['я', s @ ..] => super::t32::n130754(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word занавеса.
pub(crate) fn n120473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119211);
	s.is_empty()
}

/// The word пол-леса.
pub(crate) fn n120474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119212);
	s.is_empty()
}

/// Words ending -т-пьеса.
pub(crate) fn n80424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79591);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 102909);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 119213);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129403);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 135475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иса.
pub(crate) fn n877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 876);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6469);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23858);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23859);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6470);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23860);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50961);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79592);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 102910);
											s.is_empty()
										}
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
					super::push(out, 23861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6471);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6472);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23862);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23863);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50962);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23864);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50963);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79593);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 102911);
											s.is_empty()
										}
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
					super::push(out, 23865);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50964);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6473);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23866);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 50965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6474);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23867);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50966);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79594);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 79595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23868);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79596);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 79597);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 79598);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102912);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 119214);
													match s {
														[] => true,
														['н', s @ ..] => super::t32::n130756(s, out),
														_ => false,
													}
												}
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
							super::push(out, 50967);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79599);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 102913);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 50968);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79600);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102914);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 102915);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 119215);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n130757(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', 'с', s @ ..] => {
									super::push(out, 79601);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102916);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119216);
													match s {
														[] => true,
														['т', s @ ..] => super::t32::n130758(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', 'и', 'б', '\u{1}', s @ ..] => {
											super::push(out, 102917);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 50969);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79602);
									s.is_empty()
								}
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
			super::push(out, 6475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23869);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23870);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 50970);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79603);
									s.is_empty()
								}
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
			super::push(out, 6476);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 23871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 6477);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23872);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йса.
pub(crate) fn n878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 877);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6478);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23873);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 23874);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 23875);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кса.
pub(crate) fn n879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 878);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6479);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23876);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 50972);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79604);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102918);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119217);
													match s {
														[] => true,
														['а', s @ ..] => super::t32::n130759(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23877);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6480);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					super::push(out, 23879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6481);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 23881);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 23882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6482);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6483);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23883);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 23884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 6484);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 6485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мса.
pub(crate) fn n880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 879);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6486);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23885);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 23886);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 23887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6487);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 23888);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6488);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23889);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нса.
pub(crate) fn n881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 880);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6489);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23890);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6490);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23891);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6491);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 23892);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 50980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 23893);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оса.
pub(crate) fn n882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 881);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6492);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6493);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 6494);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23894);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50982);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79606);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102919);
											s.is_empty()
										}
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
					super::push(out, 23895);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 50983);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79607);
									s.is_empty()
								}
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
			super::push(out, 6495);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23896);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50984);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79608);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 79609);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102920);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119218);
													match s {
														[] => true,
														['л', s @ ..] => super::t32::n130760(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6496);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23897);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 23898);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 50985);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79610);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102921);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6497);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 23899);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 50986);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79611);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102922);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пса.
pub(crate) fn n883(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 882);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6498);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23900);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50987);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79612);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 23901);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 50988);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 79613);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102923);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6499);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рса.
pub(crate) fn n884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 883);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6500);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 23903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6501);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23904);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 23905);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50990);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79614);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102924);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119219);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6502);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23906);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 23907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6503);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 23908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сса.
pub(crate) fn n885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 884);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6504);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23910);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 23911);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 50993);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79615);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 102925);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119220);
													s.is_empty()
												}
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
							super::push(out, 50994);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79616);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 23912);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 50995);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79617);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102926);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 119221);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 23913);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50996);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79618);
									s.is_empty()
								}
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
			super::push(out, 6505);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23914);
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50997);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 50998);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 50999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23915);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51001);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 51002);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 51003);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79621);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102927);
											s.is_empty()
										}
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
					super::push(out, 23917);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23918);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51005);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 51006);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 51007);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 51008);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 79622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 51009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23919);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 23920);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 51010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6506);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23921);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51011);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 51012);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79623);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102928);
											s.is_empty()
										}
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
					super::push(out, 23922);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51013);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79624);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102929);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 79625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 51014);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79626);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 102930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 51015);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79627);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 51016);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79628);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102931);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 119222);
													match s {
														[] => true,
														['а', s @ ..] => super::t32::n130761(s, out),
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
									super::push(out, 79629);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 102932);
											s.is_empty()
										}
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
						['а', s @ ..] => super::t32::n51533(s, out),
						['н', s @ ..] => super::t32::n51534(s, out),
						_ => false,
					}
				}
				['ц', s @ ..] => super::t32::n24069(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t32::n6523(s, out),
		['у', s @ ..] => super::t32::n6524(s, out),
		['я', s @ ..] => super::t32::n6525(s, out),
		_ => false,
	}
}

/// Words ending -тса.
pub(crate) fn n886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 885);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 6510);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уса.
pub(crate) fn n887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 886);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6511);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 23932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6512);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 23933);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51029);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79636);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102939);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 119225);
													match s {
														[] => true,
														['л', s @ ..] => super::t32::n130763(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6513);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23934);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51030);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79637);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 102940);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119226);
													match s {
														[] => true,
														['к', s @ ..] => super::t32::n130764(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6515);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 23936);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51031);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 79638);
									s.is_empty()
								}
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
			super::push(out, 6516);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23937);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 51032);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23938);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51033);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79640);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102941);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рыса.
pub(crate) fn n6534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6518);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 23939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьса.
pub(crate) fn n889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 888);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6519);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23940);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яса.
pub(crate) fn n890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 889);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 6520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирьят-ата.
pub(crate) fn n120489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119227);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 129413);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -алгожата.
pub(crate) fn n104034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102944);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 119228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьжата.
pub(crate) fn n24096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23950);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 51041);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 79645);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 102945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжата.
pub(crate) fn n24097(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23951);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 51042);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79646);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ката.
pub(crate) fn n6545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6529);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23952);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 51043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23953);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51044);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 79647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 51045);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79648);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 102947);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119229);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79649);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102948);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119230);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 79650);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 102949);
									s.is_empty()
								}
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
			super::push(out, 23954);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 51046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 23955);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лата.
pub(crate) fn n6546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6530);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23956);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 23957);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 23958);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51048);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 79651);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102950);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119231);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 129414);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 23959);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51049);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 51050);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 79652);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 102951);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119232);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мата.
pub(crate) fn n6547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6531);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23960);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 23961);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 51051);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79653);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 23962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ната.
pub(crate) fn n6548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6532);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 23963);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51052);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 79654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 23964);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 51053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 23965);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51054);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 23966);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оата.
pub(crate) fn n6549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6533);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 23967);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51055);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пата.
pub(crate) fn n6550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6534);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 23968);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51056);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 79656);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102952);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 119233);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 129415);
													match s {
														[] => true,
														['к', s @ ..] => super::t32::n136884(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рата.
pub(crate) fn n6551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6535);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 23969);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 23970);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 51058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 79657);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тата.
pub(crate) fn n6552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6536);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23971);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 23972);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 51059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 23973);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51060);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 51061);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 79658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уата.
pub(crate) fn n6553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6537);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 23974);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 51062);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 51063);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 79659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фата.
pub(crate) fn n6554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6538);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 23975);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 23976);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 51064);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79660);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хата.
pub(crate) fn n6555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6539);
	s.is_empty()
}

/// Words ending -чата.
pub(crate) fn n6556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6540);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 23977);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 23978);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 51066);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 79661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 23979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51067);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 79662);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102955);
									s.is_empty()
								}
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
			super::push(out, 23980);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51068);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 79663);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102956);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 119234);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 23981);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51069);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 79664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шата.
pub(crate) fn n6557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6541);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 23982);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 51070);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 79665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ета.
pub(crate) fn n892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6542);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6543);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 23984);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 51072);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79666);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 102958);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6544);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23985);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 51073);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79667);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102959);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 51074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23986);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51075);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23987);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 51076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23988);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51077);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79669);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102960);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 119235);
													match s {
														[] => true,
														['в', s @ ..] => super::t32::n130768(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23989);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 23990);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 23991);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6545);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6546);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23994);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 51081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23995);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 23996);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23997);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23998);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51084);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79670);
									s.is_empty()
								}
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
			super::push(out, 6547);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24000);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24001);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 24002);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6549);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24003);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 24004);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51087);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79671);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102961);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 119236);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24005);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 51088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79672);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 24006);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24007);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24008);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51090);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79673);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 51091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6550);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6551);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24011);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51092);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 51093);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24012);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 51094);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79675);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 51095);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24013);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24014);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 51096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t32::n6568(s, out),
		['с', s @ ..] => {
			super::push(out, 6553);
			match s {
				[] => true,
				['с', s @ ..] => super::t32::n24164(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t32::n6570(s, out),
		['у', s @ ..] => super::t32::n6571(s, out),
		['ф', s @ ..] => super::t32::n6572(s, out),
		['х', s @ ..] => super::t32::n6573(s, out),
		['ч', s @ ..] => super::t32::n6574(s, out),
		['щ', s @ ..] => super::t32::n6575(s, out),
		['ь', s @ ..] => super::t32::n6576(s, out),
		_ => false,
	}
}

/// Words ending -ита.
pub(crate) fn n893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 892);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24029);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24030);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51117);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79686);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24031);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6562);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24032);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6564);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24035);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24036);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51119);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 79687);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102967);
											s.is_empty()
										}
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
					super::push(out, 24037);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6565);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6566);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24038);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24039);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51122);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 51123);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 51124);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24040);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 51125);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24041);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51126);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79690);
									s.is_empty()
								}
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
			super::push(out, 6568);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24042);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24043);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51128);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 79691);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102968);
											s.is_empty()
										}
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
					super::push(out, 24044);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24045);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51130);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79692);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102969);
											s.is_empty()
										}
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
					super::push(out, 24046);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 24047);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51132);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 51133);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 51134);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79695);
									s.is_empty()
								}
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
			super::push(out, 6569);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24048);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 51135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24049);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51136);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79696);
									s.is_empty()
								}
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
			super::push(out, 6570);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24050);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51137);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79697);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102970);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 119238);
													s.is_empty()
												}
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
							super::push(out, 51138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6571);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6572);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24051);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 51139);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 51140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24052);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51142);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79698);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 102971);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 51143);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79699);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102972);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 119239);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t32::n24202(s, out),
				['ф', s @ ..] => {
					super::push(out, 24054);
					match s {
						[] => true,
						['и', s @ ..] => super::t32::n51662(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t32::n6589(s, out),
		['ф', s @ ..] => super::t32::n6590(s, out),
		['х', s @ ..] => super::t32::n6591(s, out),
		['ч', s @ ..] => super::t32::n6592(s, out),
		['щ', s @ ..] => super::t32::n6593(s, out),
		['ю', s @ ..] => super::t32::n6594(s, out),
		_ => false,
	}
}

/// Words ending -йта.
pub(crate) fn n894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 893);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6579);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24062);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51149);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79703);
									s.is_empty()
								}
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
			super::push(out, 6580);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24063);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 51150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кта.
pub(crate) fn n895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 894);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6581);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6582);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24065);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 24066);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51151);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 79704);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102973);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 119240);
													match s {
														[] => true,
														['а', s @ ..] => super::t32::n130770(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6583);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 24067);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51152);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79705);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102974);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нта.
pub(crate) fn n896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 895);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6584);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24069);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 24070);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 24071);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24072);
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							super::push(out, 51154);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79706);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102975);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 51155);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79707);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102976);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'а', '\u{1}', s @ ..] => {
							super::push(out, 51156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24073);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51157);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79708);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 51158);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79709);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 102977);
											s.is_empty()
										}
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
					super::push(out, 24074);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24075);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51160);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79710);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102978);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 79711);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102979);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 51161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24077);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 24078);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 51164);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79712);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102980);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6585);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6586);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24080);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 24081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6587);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 24082);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 24083);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51166);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 79713);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 102981);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 119241);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51167);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79714);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 102982);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119242);
													match s {
														[] => true,
														['м', s @ ..] => super::t32::n130771(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 24085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6588);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24086);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ота.
pub(crate) fn n897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 896);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6589);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24087);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24088);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 24089);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 51170);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24090);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51171);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 51172);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24091);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51173);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79717);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102983);
											s.is_empty()
										}
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
					super::push(out, 24092);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51174);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79718);
									s.is_empty()
								}
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
			super::push(out, 6590);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24093);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51175);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24094);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 51176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24095);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 51177);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79720);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 24096);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6591);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24098);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 51178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24099);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 51179);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24100);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51180);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 79722);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 79723);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102984);
											s.is_empty()
										}
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
					super::push(out, 24101);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51181);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 79724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24102);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 51182);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 51183);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79726);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 102985);
											s.is_empty()
										}
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
					super::push(out, 24103);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51184);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 79727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 24104);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 24105);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 24106);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79728);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 79729);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102986);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6592);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24107);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 51188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 24108);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51189);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79730);
									s.is_empty()
								}
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
			super::push(out, 6593);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24109);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51190);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79731);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102987);
											s.is_empty()
										}
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
					super::push(out, 24110);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51191);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79732);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 102988);
											s.is_empty()
										}
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
					super::push(out, 24111);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51192);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79733);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102989);
											s.is_empty()
										}
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
					super::push(out, 24112);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51193);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79734);
									match s {
										[] => true,
										['к', s @ ..] => super::t32::n104080(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t32::n6610(s, out),
		['й', s @ ..] => super::t32::n6611(s, out),
		['к', s @ ..] => super::t32::n6612(s, out),
		['л', s @ ..] => super::t32::n6613(s, out),
		['м', s @ ..] => super::t32::n6614(s, out),
		['н', s @ ..] => super::t32::n6615(s, out),
		['п', s @ ..] => super::t32::n6616(s, out),
		['р', s @ ..] => super::t32::n6617(s, out),
		['с', s @ ..] => super::t32::n6618(s, out),
		['т', s @ ..] => super::t32::n6619(s, out),
		['ф', s @ ..] => super::t32::n6620(s, out),
		['х', s @ ..] => super::t32::n6621(s, out),
		['ш', s @ ..] => super::t32::n6622(s, out),
		_ => false,
	}
}

/// Words ending -пта.
pub(crate) fn n898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 897);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6607);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6608);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6609);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24193);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24194);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 51295);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79818);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рта.
pub(crate) fn n899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 898);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6610);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24195);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 24196);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24197);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51297);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79819);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103024);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6611);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24198);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51298);
							s.is_empty()
						}
						['ь', 'л', 'а', '\u{1}', s @ ..] => {
							super::push(out, 51299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 24199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 24200);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51301);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 24201);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 24202);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 24203);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6612);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6613);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 24204);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51304);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 79821);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 103025);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 119257);
													match s {
														[] => true,
														['о', s @ ..] => super::t32::n130778(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ста.
pub(crate) fn n900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 899);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6614);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24205);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 24206);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24207);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 51305);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24208);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 24209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6615);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51307);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 51308);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 51309);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24211);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51310);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79824);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 79825);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103026);
											s.is_empty()
										}
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
					super::push(out, 24212);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 24213);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51311);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 51312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 24214);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51313);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79826);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 79827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24215);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51314);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79828);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103028);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 51315);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 24216);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6616);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24217);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 24218);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 51317);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79830);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 103029);
											s.is_empty()
										}
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
					super::push(out, 24219);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51318);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 51319);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79831);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 103030);
											s.is_empty()
										}
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
					super::push(out, 24220);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 51320);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79832);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 103031);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 119258);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n130779(s, out),
														_ => false,
													}
												}
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
							super::push(out, 51321);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79833);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 51322);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 79834);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 103032);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 119259);
													match s {
														[] => true,
														['а', s @ ..] => super::t32::n130780(s, out),
														_ => false,
													}
												}
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
							super::push(out, 51323);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79835);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 103033);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 103034);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 51324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t32::n24371(s, out),
				['т', s @ ..] => super::t32::n24372(s, out),
				['ц', s @ ..] => {
					super::push(out, 24223);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t32::n6633(s, out),
		['к', s @ ..] => super::t32::n6634(s, out),
		['м', s @ ..] => {
			super::push(out, 6619);
			s.is_empty()
		}
		['н', s @ ..] => super::t32::n6636(s, out),
		['о', s @ ..] => super::t32::n6637(s, out),
		['р', s @ ..] => super::t32::n6638(s, out),
		['у', s @ ..] => super::t32::n6639(s, out),
		_ => false,
	}
}

/// Words ending -тта.
pub(crate) fn n901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 900);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6624);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24234);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 24235);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24236);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6625);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24237);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51342);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 79846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', '\u{1}', s @ ..] => {
					super::push(out, 24238);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 24239);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', s @ ..] => {
					super::push(out, 24240);
					match s {
						[] => true,
						['а', 'м', s @ ..] => {
							super::push(out, 51344);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 51345);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79848);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 103042);
											s.is_empty()
										}
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
					super::push(out, 24241);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51347);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 51348);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79849);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 103043);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 79850);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 103044);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 119262);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24242);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51349);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 79851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 51350);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 79852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 51351);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 51352);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 79854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 51353);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79855);
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
						['г', s @ ..] => {
							super::push(out, 51354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 51355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 24243);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 51356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 24244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'г', s @ ..] => {
			super::push(out, 6626);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24245);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51357);
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
					super::push(out, 24246);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'р', s @ ..] => {
					super::push(out, 24247);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51359);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 79857);
									s.is_empty()
								}
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
			super::push(out, 6627);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 6628);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24248);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 24249);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51360);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79858);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 103045);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ута.
pub(crate) fn n902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 901);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6629);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24250);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 51361);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79859);
									s.is_empty()
								}
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
			super::push(out, 6630);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24251);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 24252);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24253);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51362);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103046);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6631);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24254);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6632);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24256);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 51363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6633);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6634);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24257);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 51364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79861);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24258);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51365);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79862);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 103047);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 119263);
													match s {
														[] => true,
														['-', s @ ..] => super::t32::n130783(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6635);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24259);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51366);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 79863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фта.
pub(crate) fn n903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 902);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6636);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24260);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24261);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6637);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24263);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 6638);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хта.
pub(crate) fn n904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 903);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6639);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24266);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 24267);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 24268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6640);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24269);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 51372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6641);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24270);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 24271);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 24272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чта.
pub(crate) fn n905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 904);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6643);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24273);
					match s {
						[] => true,
						['-', 'а', s @ ..] => {
							super::push(out, 51373);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79864);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 103048);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119264);
													match s {
														[] => true,
														['т', s @ ..] => super::t32::n130784(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6644);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6645);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24275);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51375);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шта.
pub(crate) fn n906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6646);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 24276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 24277);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51377);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79866);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 103049);
											s.is_empty()
										}
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
					super::push(out, 24278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6647);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24279);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6648);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыта.
pub(crate) fn n907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 906);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 6649);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24281);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьта.
pub(crate) fn n908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 907);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эта.
pub(crate) fn n909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 908);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6651);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6652);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юта.
pub(crate) fn n910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 909);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6654);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6655);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24286);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 24287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ята.
pub(crate) fn n911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 910);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6656);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24288);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51380);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79867);
									s.is_empty()
								}
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
			super::push(out, 6657);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24289);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24290);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6658);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 24291);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 51383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6659);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 6660);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 24292);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 51384);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79868);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 103050);
											s.is_empty()
										}
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
					super::push(out, 24293);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51385);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79869);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 103051);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6661);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6662);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24295);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 51386);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79870);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 103052);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6663);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24296);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 51387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -а-де-труа.
pub(crate) fn n120527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119265);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 129433);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -байхуа.
pub(crate) fn n51920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51402);
	s.is_empty()
}

/// Words ending -нхуа.
pub(crate) fn n6698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6680);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 24315);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 51403);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79881);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103056);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -охуа.
pub(crate) fn n6699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6681);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 24316);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьхуа.
pub(crate) fn n6700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6682);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 24317);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 51404);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 79882);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 103057);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 51405);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 79883);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 103058);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 119266);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 129434);
													match s {
														[] => true,
														['в', s @ ..] => super::t32::n136896(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуа.
pub(crate) fn n924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 923);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6683);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24318);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трофа.
pub(crate) fn n24495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24343);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 51421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79888);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 79889);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 103061);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119267);
									match s {
										[] => true,
										['к', '-', s @ ..] => {
											super::push(out, 129435);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135491);
													match s {
														[] => true,
														['ь', s @ ..] => super::t32::n140421(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 79890);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103062);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 119268);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129436);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рфа.
pub(crate) fn n934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 933);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6706);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 51422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6707);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 24347);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уфа.
pub(crate) fn n935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 934);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6708);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 6709);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 6710);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 6712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ффа.
pub(crate) fn n936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 935);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6713);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 24350);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 24351);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51424);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79891);
									s.is_empty()
								}
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
			super::push(out, 6714);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьфа.
pub(crate) fn n937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 936);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6715);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24353);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эфа.
pub(crate) fn n938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 937);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6716);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юфа.
pub(crate) fn n939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 938);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яфа.
pub(crate) fn n940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 939);
	s.is_empty()
}

/// Words ending -андромаха.
pub(crate) fn n120531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119269);
	s.is_empty()
}

/// Words ending -осомаха.
pub(crate) fn n80733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79900);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			super::push(out, 103067);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ссомаха.
pub(crate) fn n80734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79901);
	s.is_empty()
}

/// Words ending -наха.
pub(crate) fn n6743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6724);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 24370);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 51440);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -паха.
pub(crate) fn n6744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6725);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24371);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 24372);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 51441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 24373);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 51442);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 79902);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 103068);
									s.is_empty()
								}
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
			super::push(out, 24374);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 51443);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 79903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24375);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 51444);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -раха.
pub(crate) fn n6745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6726);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24376);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 24377);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 24378);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51445);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79905);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 103069);
									s.is_empty()
								}
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
			super::push(out, 24379);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 51446);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79906);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 103070);
									s.is_empty()
								}
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
			super::push(out, 24380);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -таха.
pub(crate) fn n6746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6727);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 24382);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 24383);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 51448);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 79907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 24384);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 24385);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 51449);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 79908);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 103071);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 119270);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дха.
pub(crate) fn n943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 942);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6728);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 24386);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еха.
pub(crate) fn n944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 943);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6729);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24387);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 51451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 24388);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 6730);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24389);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51453);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79909);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 103072);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119271);
													match s {
														[] => true,
														['н', s @ ..] => super::t32::n130789(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6732);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24390);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24391);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 51454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6733);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 24392);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24393);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 51455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 24394);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51456);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79910);
									s.is_empty()
								}
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
			super::push(out, 6734);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24395);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6735);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24396);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 51458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 24397);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 51459);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79911);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 103073);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 119272);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24398);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51460);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 24399);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51461);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79913);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 79914);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 103074);
											s.is_empty()
										}
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
					super::push(out, 24400);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 51462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6736);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24401);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24402);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 51463);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 51464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 24403);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51465);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24404);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 51466);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24405);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 51467);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 79916);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 103075);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119273);
													match s {
														[] => true,
														['р', s @ ..] => super::t32::n130790(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 24406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 6737);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 6738);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 24408);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51468);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79917);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103076);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24409);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 51469);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79918);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 79919);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 103077);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 119274);
													match s {
														[] => true,
														['ь', s @ ..] => super::t32::n130791(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ь', s @ ..] => {
											super::push(out, 103078);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 119275);
													match s {
														[] => true,
														['а', s @ ..] => super::t32::n130792(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иха.
pub(crate) fn n945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 944);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6740);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 24410);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 24411);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 51470);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 79920);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103079);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6741);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24412);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 51471);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 79921);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 103080);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 119276);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 24413);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24414);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51472);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 79922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 24415);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6742);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 6743);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24416);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 51473);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79923);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 103081);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 79924);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 103082);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 119277);
													match s {
														[] => true,
														['и', s @ ..] => super::t32::n130793(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 24417);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 51474);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79925);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 103083);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 119278);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 24418);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 51475);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 51476);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 79927);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 103084);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 51477);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 51478);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 79929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24419);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 51479);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 51480);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24420);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 24421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6745);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 24422);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 51482);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79932);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 103085);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 119279);
													s.is_empty()
												}
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
							super::push(out, 51483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6746);
			match s {
				[] => true,
				['а', s @ ..] => super::t32::n24576(s, out),
				['е', s @ ..] => super::t32::n24577(s, out),
				['и', s @ ..] => super::t32::n24578(s, out),
				['о', s @ ..] => super::t32::n24579(s, out),
				['т', s @ ..] => super::t32::n24580(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t32::n6766(s, out),
		['р', s @ ..] => super::t32::n6767(s, out),
		['с', s @ ..] => super::t32::n6768(s, out),
		['т', s @ ..] => super::t32::n6769(s, out),
		['ф', s @ ..] => super::t32::n6770(s, out),
		['ч', s @ ..] => super::t32::n6771(s, out),
		['ш', s @ ..] => super::t32::n6772(s, out),
		['ь', s @ ..] => {
			super::push(out, 6754);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йха.
pub(crate) fn n946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 945);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6755);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 24458);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 51534);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79980);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103123);
											s.is_empty()
										}
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
					super::push(out, 24459);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мха.
pub(crate) fn n947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 946);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6756);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24460);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 51535);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 79981);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 103124);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нха.
pub(crate) fn n948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 947);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6757);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24461);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 51536);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 79982);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 103125);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6758);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 24462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

