use flexion::Path;

/// Words ending -егать.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1404);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5028);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5029);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 12451);
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							super::push(out, 12452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5030);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12453);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 5031);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5032);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12454);
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
					super::push(out, 5033);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12455);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 21404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5034);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12456);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12457);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21405);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5035);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12458);
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
					super::push(out, 5036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12459);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12460);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12461);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12462);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 21406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5037);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5038);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5039);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5040);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21407);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30020);
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
		['л', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5041);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5042);
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
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 21408);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30021);
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
				['б', s @ ..] => {
					super::push(out, 5043);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12465);
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
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5044);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5045);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 12466);
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1406);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5046);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5047);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12467);
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
					super::push(out, 5048);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12468);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t12::n5741(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t12::n1490(s, out),
		_ => false,
	}
}

/// Words ending -згать.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1408);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5052);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21413);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21414);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30024);
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
		['у', s @ ..] => {
			super::push(out, 1409);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12479);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
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
		['ы', s @ ..] => {
			super::push(out, 1410);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12480);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5055);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12481);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21415);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21416);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30025);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 35997);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30026);
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 21417);
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
								['о', s @ ..] => {
									super::push(out, 21418);
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
								['ы', s @ ..] => {
									super::push(out, 21419);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30027);
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
						['д', s @ ..] => {
							super::push(out, 12482);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21420);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21421);
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
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1411);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12483);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12484);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21422);
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
							super::push(out, 12485);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21423);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21424);
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
		_ => false,
	}
}

/// Words ending -игать.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1412);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12486);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12487);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 21425);
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30028);
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12488);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									super::push(out, 21426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 12489);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12490);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12491);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21427);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30029);
											s.is_empty()
										}
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21428);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 30030);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 21429);
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
							super::push(out, 12492);
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
						['ы', s @ ..] => {
							super::push(out, 12493);
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
		['ж', s @ ..] => {
			super::push(out, 1413);
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
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['с', '\u{1}', s @ ..] => {
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
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1414);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5057);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5058);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12494);
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
					super::push(out, 5059);
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
				['т', s @ ..] => {
					super::push(out, 5060);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12495);
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
			super::push(out, 1415);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => super::t12::n15985(s, out),
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
			super::push(out, 1416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t12::n1500(s, out),
		_ => false,
	}
}

/// Words ending -лгать.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1418);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1419);
			match s {
				[] => true,
				['н', '\u{1}', s @ ..] => {
					super::push(out, 5062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1420);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5063);
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
		['и', s @ ..] => {
			super::push(out, 1421);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5064);
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
			super::push(out, 1422);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5065);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					super::push(out, 5067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мгать.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5068);
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

/// Words ending -нгать.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 250);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5069);
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

/// Words ending -огать.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 1425);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5070);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12497);
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
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12498);
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12499);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 21430);
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
				['з', s @ ..] => {
					super::push(out, 5071);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5072);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5074);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12502);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21431);
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
					super::push(out, 5076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12503);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12504);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21432);
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
							super::push(out, 12505);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21433);
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
							super::push(out, 12506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21434);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21435);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30031);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 30032);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30033);
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
								['д', s @ ..] => {
									super::push(out, 21436);
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
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21437);
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
								['с', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 21438);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30034);
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
		_ => false,
	}
}

/// Words ending -ргать.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1427);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5077);
					match s {
						[] => true,
						['з', s @ ..] => {
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
								_ => false,
							}
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
										['с', 'и', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', 'с', 'и', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5078);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12507);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12508);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 12509);
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
						['о', s @ ..] => {
							super::push(out, 12510);
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
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12511);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21439);
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
		['о', s @ ..] => {
			super::push(out, 1428);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12512);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12513);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21440);
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
							super::push(out, 12514);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21441);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21442);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30035);
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
						['ш', s @ ..] => {
							super::push(out, 12515);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5080);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12516);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угать.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5081);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5082);
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
									super::push(out, 21444);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30036);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5083);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5084);
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
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5085);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12517);
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
			super::push(out, 1430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5086);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5087);
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
					super::push(out, 5088);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5089);
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
				['о', s @ ..] => {
					super::push(out, 5090);
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
				['т', s @ ..] => {
					super::push(out, 5091);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21445);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21446);
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
									super::push(out, 21447);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 30037);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 35998);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76409(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30038);
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
									super::push(out, 21448);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 21449);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30039);
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
								['о', s @ ..] => super::t12::n31887(s, out),
								['с', s @ ..] => {
									super::push(out, 21451);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => super::t12::n31889(s, out),
								['у', s @ ..] => super::t12::n31890(s, out),
								['ы', s @ ..] => super::t12::n31891(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => super::t12::n5802(s, out),
				_ => false,
			}
		}
		['ш', s @ ..] => super::t12::n1514(s, out),
		_ => false,
	}
}

/// Words ending -ыгать.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1433);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5094);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5095);
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
		['м', s @ ..] => {
			super::push(out, 1434);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 5096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12520);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12521);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21455);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21456);
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
							super::push(out, 12522);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21457);
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
							super::push(out, 12523);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21458);
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
							super::push(out, 12524);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21459);
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
		['р', s @ ..] => {
			super::push(out, 1435);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5097);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12525);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12526);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12527);
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
					super::push(out, 5099);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12528);
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
					super::push(out, 5100);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12529);
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
					super::push(out, 5101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12530);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12531);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21460);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21461);
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
							super::push(out, 12532);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21462);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12533);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5102);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12534);
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
				['у', s @ ..] => {
					super::push(out, 5103);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12535);
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
				['ы', s @ ..] => {
					super::push(out, 5104);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12536);
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

/// Words ending -ьгать.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 1436);
			match s {
				[] => true,
				['е', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['я', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -югать.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12537);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 12538);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21463);
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
							super::push(out, 12539);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21464);
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

/// Words ending -ягать.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1438);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5105);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5106);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12540);
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
					super::push(out, 5107);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12541);
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1439);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12543);
							match s {
								[] => true,
								['з', s @ ..] => {
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
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21465);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21466);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30047);
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
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
		['с', s @ ..] => {
			super::push(out, 1440);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5110);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5111);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12545);
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
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 12546);
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							super::push(out, 12547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5112);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5113);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12548);
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
					super::push(out, 5114);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12549);
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
					super::push(out, 5115);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12550);
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
				['и', s @ ..] => {
					super::push(out, 5116);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5117);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12552);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12553);
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
					super::push(out, 5118);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5119);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12554);
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
				['у', s @ ..] => {
					super::push(out, 5120);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5121);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12555);
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

/// Words ending -ладать.
pub(crate) fn n1529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1445);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5135);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21468);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 21469);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 30048);
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5136);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12565);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 21470);
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
		['к', s @ ..] => {
			super::push(out, 5137);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12566);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 12567);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21471);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30049);
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
					super::push(out, 12568);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 21472);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							super::push(out, 21473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12570);
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

/// Words ending -падать.
pub(crate) fn n1530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1446);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5138);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5139);
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
		['в', s @ ..] => {
			super::push(out, 5140);
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
		['д', s @ ..] => {
			super::push(out, 5141);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5142);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					super::push(out, 12571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5143);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12573);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 30050);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 36000);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5145);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 12574);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 21474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5146);
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
			super::push(out, 5147);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21475);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30051);
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

/// Words ending -радать.
pub(crate) fn n1531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1447);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 5148);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21476);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 21477);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 30052);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 36001);
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
						['о', s @ ..] => {
							super::push(out, 21478);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 30053);
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
								['с', s @ ..] => {
									super::push(out, 30054);
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
							super::push(out, 21479);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 21480);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 30055);
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

/// Words ending -ддать.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1450);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5150);
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
			super::push(out, 1451);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12575);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12576);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 21481);
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

/// Words ending -едать.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1452);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1453);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
			super::push(out, 1454);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12577);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12578);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 21482);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21483);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30056);
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
							super::push(out, 12579);
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
		['в', s @ ..] => {
			super::push(out, 1455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5153);
					s.is_empty()
				}
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5154);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
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
									s.is_empty()
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
				['о', s @ ..] => {
					super::push(out, 5155);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 12580);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21484);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 30057);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'и', s @ ..] => {
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
						['р', s @ ..] => {
							super::push(out, 12581);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30058);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36002);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76413(s, out),
														_ => false,
													}
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
					super::push(out, 5156);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12582);
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
			super::push(out, 1456);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5157);
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
		['и', s @ ..] => {
			super::push(out, 1457);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5158);
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
		['н', s @ ..] => {
			super::push(out, 1458);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5159);
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
			super::push(out, 1459);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21485);
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5161);
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
		['р', s @ ..] => super::t12::n1544(s, out),
		['с', s @ ..] => super::t12::n1545(s, out),
		['у', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => super::t12::n1547(s, out),
		['ы', s @ ..] => {
			super::push(out, 1464);
			match s {
				[] => true,
				['в', s @ ..] => super::t12::n5905(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ждать.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1465);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 5173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12595);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21494);
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
						['з', s @ ..] => {
							super::push(out, 12596);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21495);
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
						['о', s @ ..] => {
							super::push(out, 12597);
							match s {
								[] => true,
								['в', 'о', 'р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5174);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12598);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									super::push(out, 21496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 12599);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				['н', s @ ..] => {
					super::push(out, 5175);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5176);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12601);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 21497);
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
												['з', 'о', 'в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 21498);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 36003);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t12::n76414(s, out),
														_ => false,
													}
												}
												['п', s @ ..] => {
													super::push(out, 36004);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21499);
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
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12602);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 21500);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5177);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12603);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 21501);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30062);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30063);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1467);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5178);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12604);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30064);
											s.is_empty()
										}
										['е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 12605);
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
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t12::n5912(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t12::n1552(s, out),
		['о', s @ ..] => super::t12::n1553(s, out),
		['р', s @ ..] => super::t12::n1554(s, out),
		['у', s @ ..] => super::t12::n1555(s, out),
		['ы', s @ ..] => super::t12::n1556(s, out),
		_ => false,
	}
}

/// Words ending -здать.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1473);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5196);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21510);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['р', s @ ..] => {
					super::push(out, 5197);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12625);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12626);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21511);
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
			super::push(out, 1474);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5198);
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
		['о', s @ ..] => {
			super::push(out, 1475);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 5200);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12627);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 30070);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 36008);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t12::n76418(s, out),
														_ => false,
													}
												}
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
							super::push(out, 12628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12629);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12630);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12631);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 21512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12632);
							match s {
								[] => true,
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1476);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 5202);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12633);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12634);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21513);
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
				['д', s @ ..] => {
					super::push(out, 5203);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12635);
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
				['н', s @ ..] => {
					super::push(out, 5204);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идать.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1477);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5205);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5206);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12636);
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
					super::push(out, 5207);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12637);
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
				['у', s @ ..] => {
					super::push(out, 5208);
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
			super::push(out, 1478);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5209);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1479);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5211);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1480);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5212);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5213);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12639);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 12640);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12641);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12643);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12644);
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
					super::push(out, 5215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12645);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12646);
							match s {
								[] => true,
								['р', s @ ..] => {
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
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5216);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12647);
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
		['р', s @ ..] => {
			super::push(out, 1481);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5217);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндать.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1482);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5218);
					match s {
						[] => true,
						['а', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21515);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21516);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 30071);
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1483);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5219);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 1484);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5220);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 12649);
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
					super::push(out, 5221);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12650);
							match s {
								[] => true,
								['ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1485);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5222);
					match s {
						[] => true,
						['ш', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одать.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1486);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5223);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5224);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12651);
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
					super::push(out, 5225);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12652);
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
					super::push(out, 5226);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12653);
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
					super::push(out, 5227);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12654);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12655);
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
		['д', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5228);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5229);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5230);
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
		['л', s @ ..] => {
			super::push(out, 1488);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12656);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12657);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 12658);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 21517);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12659);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 12660);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21518);
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
							super::push(out, 12661);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12662);
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
								['р', s @ ..] => {
									super::push(out, 21519);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30072);
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
							super::push(out, 12663);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12664);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12665);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21521);
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
					super::push(out, 5232);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12666);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21522);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21523);
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21524);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21525);
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
								['о', s @ ..] => {
									super::push(out, 21526);
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
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1489);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5233);
					match s {
						[] => true,
						['р', s @ ..] => super::t12::n16281(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t12::n1574(s, out),
		_ => false,
	}
}

/// Words ending -сдать.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -тдать.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1491);
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

/// Words ending -удать.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 1492);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5235);
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
		['х', s @ ..] => {
			super::push(out, 1493);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5236);
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5237);
					match s {
						[] => true,
						['и', s @ ..] => {
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

/// Words ending -ыдать.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1494);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5238);
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
		['р', s @ ..] => {
			super::push(out, 1495);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5239);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5240);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5241);
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
				['о', s @ ..] => {
					super::push(out, 5242);
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
				['т', s @ ..] => {
					super::push(out, 5243);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12668);
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

/// Words ending -юдать.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1496);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21530);
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
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядать.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1497);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5244);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5245);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5246);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5247);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5248);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'г', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12670);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12671);
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
					super::push(out, 5250);
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
		['р', s @ ..] => {
			super::push(out, 1498);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12672);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12673);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21531);
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
						['т', s @ ..] => {
							super::push(out, 12674);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21532);
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

/// Words ending -бражать.
pub(crate) fn n6020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5264);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'ы', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 21535);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36009);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 39636);
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
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36010);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39637);
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
		_ => false,
	}
}

/// Words ending -зражать.
pub(crate) fn n6022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5266);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12685);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 30075);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 36011);
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

/// Words ending -тражать.
pub(crate) fn n6025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5269);
	s.is_empty()
}

/// Words ending -ыражать.
pub(crate) fn n6026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5270);
	s.is_empty()
}

/// Words ending -сажать.
pub(crate) fn n1596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1506);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5271);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5272);
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
		['д', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5273);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12686);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 21537);
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
		['о', s @ ..] => {
			super::push(out, 5274);
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
		_ => false,
	}
}

/// Words ending -бжать.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1507);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 5275);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', 'о', 'р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1508);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 5276);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12687);
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

/// Words ending -вжать.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1509);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -джать.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1510);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5277);
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

/// Words ending -ежать.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1511);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5278);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5279);
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
				['б', s @ ..] => {
					super::push(out, 5280);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12688);
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
					super::push(out, 5281);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5282);
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
						['р', s @ ..] => {
							super::push(out, 12689);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5283);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12690);
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
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5284);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12691);
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
		['в', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5285);
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1513);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5286);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5287);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12692);
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
					super::push(out, 5288);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12693);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5289);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12694);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 30076);
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
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12695);
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
					super::push(out, 5290);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12696);
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
					super::push(out, 5291);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12697);
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
				['и', s @ ..] => {
					super::push(out, 5292);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12698);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 21539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5293);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12699);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 21540);
									match s {
										[] => true,
										['т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12700);
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
							super::push(out, 12701);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12702);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 21541);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5294);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12703);
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
				['у', s @ ..] => {
					super::push(out, 5295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12704);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 12705);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21542);
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
				['ы', s @ ..] => {
					super::push(out, 5296);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12706);
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
		['м', s @ ..] => super::t12::n1604(s, out),
		['н', s @ ..] => super::t12::n1605(s, out),
		['р', s @ ..] => super::t12::n1606(s, out),
		_ => false,
	}
}

/// Words ending -жжать.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1517);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5301);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1518);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 5302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12707);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12708);
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
						['о', s @ ..] => {
							super::push(out, 12709);
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

/// Words ending -зжать.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1519);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5303);
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
			super::push(out, 1520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5304);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5305);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 12710);
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							super::push(out, 12711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5306);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12712);
							match s {
								[] => true,
								['р', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30077);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30078);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 30079);
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
					super::push(out, 5307);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12713);
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
				['и', s @ ..] => {
					super::push(out, 5308);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12714);
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
					super::push(out, 5309);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12716);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21545);
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
					super::push(out, 5310);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12717);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30080);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 36012);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76422(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12718);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21546);
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
				['ъ', s @ ..] => {
					super::push(out, 5312);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12719);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21547);
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
							super::push(out, 12720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12721);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 12722);
							match s {
								[] => true,
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
						['с', '\u{1}', s @ ..] => {
							super::push(out, 12723);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12724);
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
				['ы', s @ ..] => {
					super::push(out, 5313);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21548);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21549);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30081);
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
		['и', s @ ..] => {
			super::push(out, 1521);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12725);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12726);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12727);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1522);
			match s {
				[] => true,
				['в', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t12::n1613(s, out),
		_ => false,
	}
}

/// Words ending -ижать.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1524);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'з', s @ ..] => {
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
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1525);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1526);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5317);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12728);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21554);
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
		['р', s @ ..] => {
			super::push(out, 1528);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5319);
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
		['т', s @ ..] => {
			super::push(out, 1529);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5320);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12729);
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
		['ч', 'и', 'н', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лжать.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1530);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12730);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21555);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30083);
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
								['н', s @ ..] => {
									super::push(out, 21556);
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
							super::push(out, 12731);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21557);
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
							super::push(out, 12732);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -нжать.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1531);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 5321);
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
		['у', s @ ..] => {
			super::push(out, 1532);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5322);
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

/// Words ending -ожать.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1533);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12733);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21558);
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
		['в', s @ ..] => {
			super::push(out, 1534);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1535);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5324);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1536);
			match s {
				[] => true,
				['о', 'м', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1537);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5325);
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'п', '\u{1}', s @ ..] => {
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
		['п', s @ ..] => {
			super::push(out, 1538);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1539);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5327);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5328);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12734);
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
					super::push(out, 5329);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12736);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12737);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21559);
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
							super::push(out, 12738);
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
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5331);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12739);
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
					super::push(out, 5332);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12740);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 21560);
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
						['д', s @ ..] => {
							super::push(out, 12741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 21561);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 30084);
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
									super::push(out, 21562);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30085);
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
								['у', s @ ..] => {
									super::push(out, 21563);
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
				['п', s @ ..] => {
					super::push(out, 5333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5334);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21564);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 21565);
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
			super::push(out, 1540);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 5335);
					match s {
						[] => true,
						['и', 'н', 'з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржать.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1541);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1542);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5336);
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
			super::push(out, 1543);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12743);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12744);
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
							super::push(out, 12745);
							match s {
								[] => true,
								['и', s @ ..] => {
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
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12746);
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
							super::push(out, 12747);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21566);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 21567);
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
									super::push(out, 21568);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21569);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 21570);
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
							super::push(out, 12748);
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
						['у', s @ ..] => {
							super::push(out, 12749);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12750);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21571);
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
			super::push(out, 1544);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5337);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12751);
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
					super::push(out, 5338);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5339);
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

/// Words ending -сжать.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тжать.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1545);
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

/// Words ending -ужать.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1546);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1547);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5340);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12752);
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
		['л', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5341);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12753);
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
		['н', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 1549);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5342);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5343);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12754);
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
					super::push(out, 5345);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5346);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1550);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5347);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5348);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12755);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'о', 'в', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1551);
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
		['т', s @ ..] => {
			super::push(out, 1552);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1553);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5349);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12756);
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

/// Words ending -ыжать.
pub(crate) fn n295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1555);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5350);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
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
		_ => false,
	}
}

/// Words ending -южать.
pub(crate) fn n296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1556);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1557);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5351);
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

/// Words ending -яжать.
pub(crate) fn n297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1558);
			match s {
				[] => true,
				['а', s @ ..] => {
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
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
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
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
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
		['т', s @ ..] => {
			super::push(out, 1559);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5352);
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

/// Words ending -алазать.
pub(crate) fn n6160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5359);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 12761);
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

/// Words ending -блазать.
pub(crate) fn n6161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5360);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12762);
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

/// Words ending -елазать.
pub(crate) fn n6162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5361);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 12763);
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

/// Words ending -злазать.
pub(crate) fn n6163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5362);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12764);
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

/// Words ending -олазать.
pub(crate) fn n6164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5363);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 12765);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12766);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 21574);
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

/// Words ending -слазать.
pub(crate) fn n6165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5364);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мазать.
pub(crate) fn n1656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1562);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5365);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5366);
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
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5367);
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
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5368);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'и', 'м', '\u{1}', s @ ..] => {
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
			super::push(out, 5369);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бзать.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 295);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1563);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -езать.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1564);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12768);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 12769);
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
					super::push(out, 5370);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 5371);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5372);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12771);
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
					super::push(out, 5373);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12772);
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
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5374);
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
					super::push(out, 5375);
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5376);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21576);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30086);
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
		['р', s @ ..] => {
			super::push(out, 1565);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5377);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5378);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12773);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 12774);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 5379);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21579);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21580);
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
				['в', s @ ..] => {
					super::push(out, 5380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5381);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12776);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 21581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12777);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 21582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5382);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5383);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 12779);
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							super::push(out, 12780);
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							super::push(out, 12781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5384);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5385);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21583);
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12783);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5387);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 12786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5388);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12787);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12788);
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
				['ы', s @ ..] => {
					super::push(out, 5389);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12789);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21584);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21585);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30087);
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
		['х', s @ ..] => {
			super::push(out, 1566);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5390);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5391);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12790);
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
					super::push(out, 5392);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12791);
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
			super::push(out, 1567);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 5393);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изать.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1568);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5394);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12792);
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
		['л', s @ ..] => {
			super::push(out, 1569);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5395);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5396);
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
					super::push(out, 5397);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
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
				['з', s @ ..] => {
					super::push(out, 5398);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12793);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21586);
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
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5399);
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
					super::push(out, 5400);
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
					super::push(out, 5401);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12794);
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
		['н', s @ ..] => {
			super::push(out, 1570);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5402);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5403);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5404);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12795);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21587);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5405);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12796);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12797);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5406);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 12798);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21588);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12799);
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
					super::push(out, 5407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12800);
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
					super::push(out, 5408);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12801);
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

/// Words ending -лзать.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1571);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12802);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12803);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21589);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', '\u{1}', s @ ..] => {
									super::push(out, 21590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 12804);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21591);
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
							super::push(out, 12805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12806);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21592);
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
							super::push(out, 12807);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21593);
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
						['и', s @ ..] => {
							super::push(out, 12808);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21594);
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
							super::push(out, 12809);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21595);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 21596);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 21597);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21598);
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
							super::push(out, 12810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 21599);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30088);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 21600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30089);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12811);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 21601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 12812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12813);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21602);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30090);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30091);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36013);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76423(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нзать.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1572);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рзать.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5409);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5410);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12814);
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
					super::push(out, 5411);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5412);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5413);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12815);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21603);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 30092);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36014);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76424(s, out),
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
									super::push(out, 21604);
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
							super::push(out, 12816);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 12817);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12818);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 12819);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12820);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12821);
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
						['с', s @ ..] => {
							super::push(out, 12822);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12823);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12824);
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
				['о', s @ ..] => {
					super::push(out, 5414);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12825);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12826);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21605);
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
					super::push(out, 5415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12827);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12828);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12829);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21606);
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
							super::push(out, 12830);
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
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 5416);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12831);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 12832);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1574);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5417);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12833);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узать.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1575);
			match s {
				[] => true,
				['г', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ызать.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1576);
			match s {
				[] => true,
				['о', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 12834);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21607);
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
							super::push(out, 12835);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21608);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1577);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
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
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
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

/// Words ending -ьзать.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1578);
			match s {
				[] => true,
				['о', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12836);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 12837);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12838);
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
		_ => false,
	}
}

/// Words ending -юзать.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1579);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1580);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5418);
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
			super::push(out, 1581);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5419);
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

/// Words ending -язать.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1582);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1583);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5420);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5421);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 12839);
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							super::push(out, 12840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					super::push(out, 5422);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5423);
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
					super::push(out, 5424);
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5425);
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
					super::push(out, 5426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', 'и', 'а', 'з', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5427);
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
				['у', s @ ..] => {
					super::push(out, 5428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5429);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12842);
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
			super::push(out, 1584);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5430);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12843);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12844);
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
			super::push(out, 1585);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1586);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5432);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12845);
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
					super::push(out, 5433);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12846);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21609);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30093);
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
		_ => false,
	}
}

/// The word закудакать.
pub(crate) fn n65206(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -какать.
pub(crate) fn n1686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1592);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5442);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5443);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12849);
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
			super::push(out, 5444);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12850);
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
			super::push(out, 5445);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12851);
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
			super::push(out, 5446);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12852);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12853);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21613);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 21614);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 12854);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21615);
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
					super::push(out, 12855);
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
				['и', s @ ..] => {
					super::push(out, 12856);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12857);
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
				['т', s @ ..] => {
					super::push(out, 12858);
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
					super::push(out, 12859);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21616);
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
		['ы', s @ ..] => {
			super::push(out, 5447);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12860);
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

/// Words ending -лакать.
pub(crate) fn n1687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1593);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5448);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5449);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21617);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21618);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 30096);
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
					super::push(out, 12862);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 21619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 21620);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 30097);
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
					super::push(out, 12864);
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
			super::push(out, 5450);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12865);
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
			super::push(out, 5451);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12866);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12867);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 21621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 21622);
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
					super::push(out, 12868);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 21623);
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
					super::push(out, 12869);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 21624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 21625);
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
					super::push(out, 12870);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21626);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 21627);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 21628);
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
				['т', s @ ..] => {
					super::push(out, 12871);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 21629);
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
					super::push(out, 12872);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 21630);
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
			super::push(out, 5452);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12873);
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

/// Words ending -омакать.
pub(crate) fn n6287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5454);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 12874);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12875);
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

/// Words ending -ымакать.
pub(crate) fn n6289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5456);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 12876);
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

/// Words ending -пакать.
pub(crate) fn n1689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1595);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 5457);
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

/// Words ending -ракать.
pub(crate) fn n1690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1596);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5458);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12877);
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
			super::push(out, 5459);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12878);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 30098);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 30099);
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

/// Words ending -такать.
pub(crate) fn n1691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1597);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5460);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12879);
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
			super::push(out, 5461);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12881);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 21631);
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

/// Words ending -хакать.
pub(crate) fn n1692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1598);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бкать.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1599);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5463);
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

/// Words ending -вкать.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1600);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12882);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12883);
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
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12884);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12885);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12886);
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
						['с', s @ ..] => {
							super::push(out, 12887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21632);
									s.is_empty()
								}
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
			super::push(out, 1601);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12888);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21633);
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
					super::push(out, 5467);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12889);
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
				['т', s @ ..] => {
					super::push(out, 5468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12890);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12891);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12892);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -екать.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1602);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1603);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5469);
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
			super::push(out, 1604);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5470);
					match s {
						[] => true,
						['а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1605);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5471);
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
			super::push(out, 1606);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5473);
					match s {
						[] => true,
						['з', 'и', s @ ..] => {
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
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5474);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5475);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12893);
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
					super::push(out, 5476);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12894);
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
					super::push(out, 5477);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12895);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12896);
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
					super::push(out, 5478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12897);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12898);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21634);
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
					super::push(out, 5479);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12899);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21635);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 21636);
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
								['с', s @ ..] => {
									super::push(out, 21637);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30100);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 30101);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 36015);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76425(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1610);
			match s {
				[] => true,
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
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['а', s @ ..] => super::t12::n6327(s, out),
				['б', s @ ..] => {
					super::push(out, 5481);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5482);
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t12::n6330(s, out),
				['з', s @ ..] => {
					super::push(out, 5484);
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5485);
					s.is_empty()
				}
				['т', s @ ..] => super::t12::n6334(s, out),
				['у', s @ ..] => super::t12::n6335(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t12::n1706(s, out),
		['т', s @ ..] => super::t12::n1707(s, out),
		['х', s @ ..] => super::t12::n1708(s, out),
		['ц', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t12::n6361(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t12::n1710(s, out),
		['ш', s @ ..] => super::t12::n1711(s, out),
		_ => false,
	}
}

/// Words ending -икать.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5505);
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
			super::push(out, 1620);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5507);
					match s {
						[] => true,
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21647);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1622);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5508);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12912);
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
			super::push(out, 1623);
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
		['з', s @ ..] => {
			super::push(out, 1624);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5509);
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
		['к', s @ ..] => {
			super::push(out, 1625);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1626);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 5511);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12913);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 21648);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 12914);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21649);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21650);
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
								['о', s @ ..] => {
									super::push(out, 21651);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30106);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30107);
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
						['т', s @ ..] => {
							super::push(out, 12915);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 12916);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21652);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21653);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 21654);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30108);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 30109);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 36018);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t12::n76428(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 5512);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12917);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12918);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21655);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30110);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21656);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30111);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 12919);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21657);
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
							super::push(out, 12920);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 21658);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											super::push(out, 30112);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t12::n16763(s, out),
						['о', s @ ..] => super::t12::n16764(s, out),
						['с', s @ ..] => super::t12::n16765(s, out),
						['ы', s @ ..] => super::t12::n16766(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t12::n6376(s, out),
				['ш', s @ ..] => super::t12::n6377(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t12::n1721(s, out),
		['о', s @ ..] => super::t12::n1722(s, out),
		['п', s @ ..] => super::t12::n1723(s, out),
		['р', s @ ..] => super::t12::n1724(s, out),
		['с', s @ ..] => super::t12::n1725(s, out),
		['т', s @ ..] => super::t12::n1726(s, out),
		['х', s @ ..] => super::t12::n1727(s, out),
		['ч', s @ ..] => super::t12::n1728(s, out),
		['ш', s @ ..] => super::t12::n1729(s, out),
		['щ', s @ ..] => super::t12::n1730(s, out),
		_ => false,
	}
}

/// Words ending -йкать.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1637);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 5541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12941);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12942);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21672);
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
					super::push(out, 5542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12943);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12944);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21673);
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
		['е', s @ ..] => {
			super::push(out, 1638);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5543);
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
			super::push(out, 1639);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5544);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12945);
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

/// Words ending -лкать.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5545);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5546);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12946);
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
			super::push(out, 1641);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 5547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12947);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12948);
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
							super::push(out, 12949);
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 12950);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21674);
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
			super::push(out, 1642);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 5548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['т', s @ ..] => {
					super::push(out, 5549);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12951);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12952);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21675);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21676);
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
							super::push(out, 12953);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
							super::push(out, 12954);
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
							super::push(out, 12955);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21677);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 36019);
													match s {
														[] => true,
														['п', s @ ..] => super::t12::n76429(s, out),
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
									super::push(out, 21678);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 30116);
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
						['ы', s @ ..] => {
							super::push(out, 12956);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21679);
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

/// Words ending -мкать.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1643);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5551);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 5552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12957);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12958);
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
				['м', s @ ..] => {
					super::push(out, 5553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 5554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12959);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12960);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21680);
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
							super::push(out, 12961);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21681);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21682);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30117);
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
		['о', s @ ..] => {
			super::push(out, 1644);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12962);
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
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12963);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1645);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5556);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5557);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5558);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 12964);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21683);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 21684);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 30118);
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
									super::push(out, 21685);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 21686);
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
					super::push(out, 5559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12965);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12966);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 21687);
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
		['ы', s @ ..] => {
			super::push(out, 1646);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5560);
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

/// Words ending -нкать.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1647);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5561);
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

/// Words ending -окать.
pub(crate) fn n319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 315);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1648);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1649);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5562);
					match s {
						[] => true,
						['ц', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1650);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5563);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
		['д', s @ ..] => {
			super::push(out, 1651);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 5564);
					match s {
						[] => true,
						['а', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21688);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 21689);
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
				['н', s @ ..] => {
					super::push(out, 5565);
					match s {
						[] => true,
						['а', 'х', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1652);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5566);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5567);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5568);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5569);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5570);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5571);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12967);
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
				['у', s @ ..] => {
					super::push(out, 5572);
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
		['л', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5573);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5574);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12968);
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
			super::push(out, 1654);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5575);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5576);
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
					super::push(out, 5577);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12969);
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
					super::push(out, 5578);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12970);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12971);
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
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5579);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 12972);
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
				['о', s @ ..] => {
					super::push(out, 5580);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 12973);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 21690);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12974);
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 5581);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12975);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12976);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12977);
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
				['ы', s @ ..] => {
					super::push(out, 5582);
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
		['н', s @ ..] => {
			super::push(out, 1655);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1656);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 5583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 12978);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21691);
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
				['ш', s @ ..] => {
					super::push(out, 5584);
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
			super::push(out, 1657);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 5585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 1658);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5586);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 5587);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12979);
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
				['о', s @ ..] => super::t12::n6461(s, out),
				['т', s @ ..] => {
					super::push(out, 5589);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12982);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t12::n32536(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t12::n1753(s, out),
		['ш', s @ ..] => super::t12::n1754(s, out),
		_ => false,
	}
}

/// Words ending -ркать.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1661);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5591);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5592);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 12984);
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
					super::push(out, 5593);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12985);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12986);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21693);
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
				['к', s @ ..] => {
					super::push(out, 5594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12987);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12988);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 21694);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 21695);
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
							super::push(out, 12989);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21696);
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
							super::push(out, 12990);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 21697);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 21698);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 30119);
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
				['х', s @ ..] => {
					super::push(out, 5595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12991);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12992);
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
							super::push(out, 12993);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12994);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21699);
									match s {
										[] => true,
										['о', 'р', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12995);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21700);
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
							super::push(out, 12996);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21701);
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
				['ш', s @ ..] => {
					super::push(out, 5596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12997);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12998);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									super::push(out, 21702);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 21703);
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
							super::push(out, 12999);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 21704);
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
							super::push(out, 13000);
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
						['с', s @ ..] => {
							super::push(out, 13001);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 21705);
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
							super::push(out, 13002);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 21706);
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
		['е', s @ ..] => super::t12::n1756(s, out),
		['и', s @ ..] => {
			super::push(out, 1663);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5600);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 13016);
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
					super::push(out, 5601);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 13017);
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
				['ч', s @ ..] => super::t12::n6477(s, out),
				['ш', s @ ..] => super::t12::n6478(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t12::n1758(s, out),
		['у', s @ ..] => super::t12::n1759(s, out),
		['ы', s @ ..] => super::t12::n1760(s, out),
		['ю', s @ ..] => {
			super::push(out, 1667);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t12::n6495(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

