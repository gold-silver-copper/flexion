use flexion::Path;

/// Words ending -сл.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2537);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2538);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11999);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32629);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60847);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 88444);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 32630);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60848);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 88445);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32631);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60849);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 88446);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тл.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2539);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32633);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 32634);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60850);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88447);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 109606);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12001);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60851);
									s.is_empty()
								}
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
			super::push(out, 2540);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12002);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ул.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2541);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12003);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12004);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32636);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 60852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12005);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32637);
							match s {
								[] => true,
								['ъ', s @ ..] => {
									super::push(out, 60853);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88448);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 32638);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60854);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 88449);
											s.is_empty()
										}
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
					super::push(out, 12006);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32639);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60855);
									s.is_empty()
								}
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
			super::push(out, 2542);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12007);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 12008);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32641);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60856);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88450);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 109607);
													match s {
														[] => true,
														['с', s @ ..] => super::t19::n124767(s, out),
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
									super::push(out, 60857);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 88451);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2543);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12009);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32642);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 60858);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 88452);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 109608);
													match s {
														[] => true,
														['в', s @ ..] => super::t19::n124768(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2544);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12010);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32643);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32644);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 32645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 12011);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32646);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60859);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88453);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 60860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12012);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32647);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60861);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 32648);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32649);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12013);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 12014);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32651);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60864);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88454);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109609);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2545);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12015);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32652);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12016);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32653);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 32654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2546);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 12017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2547);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12018);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32655);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60865);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88455);
											s.is_empty()
										}
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
					super::push(out, 12019);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32656);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12020);
					match s {
						[] => true,
						['у', s @ ..] => super::t19::n32945(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t19::n12074(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t19::n2557(s, out),
		['м', s @ ..] => super::t19::n2558(s, out),
		['н', s @ ..] => super::t19::n2559(s, out),
		['о', s @ ..] => super::t19::n2560(s, out),
		['п', s @ ..] => super::t19::n2561(s, out),
		['р', s @ ..] => super::t19::n2562(s, out),
		['с', s @ ..] => super::t19::n2563(s, out),
		['т', s @ ..] => super::t19::n2564(s, out),
		['ц', s @ ..] => super::t19::n2565(s, out),
		_ => false,
	}
}

/// Words ending -фл.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 2557);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыл.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2558);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2559);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 12046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12047);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 12048);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32680);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12049);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32681);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32682);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 60880);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88464);
											s.is_empty()
										}
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
					super::push(out, 12050);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32683);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60881);
									s.is_empty()
								}
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
			super::push(out, 2561);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12051);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 32684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'о', 'т', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32685);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60882);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 88465);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32686);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12052);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 12053);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2562);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12055);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 12056);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 32688);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60884);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12057);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32689);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 60885);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88466);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 109616);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2564);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эл.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2565);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12059);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 12060);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2566);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2567);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2568);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12062);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12063);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32694);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 32695);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 32696);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 60886);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88467);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2569);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12064);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32697);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2570);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12065);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32698);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 60887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 12066);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32699);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60888);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 88468);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 109617);
													match s {
														[] => true,
														['с', s @ ..] => super::t19::n124770(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2571);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12067);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32700);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60889);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88469);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ял.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 250);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2572);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32701);
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
				['н', s @ ..] => {
					super::push(out, 12069);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32702);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 60890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2573);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12071);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32703);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 60891);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 88470);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2574);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -игвам.
pub(crate) fn n32994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32707);
	s.is_empty()
}

/// Words ending -гам.
pub(crate) fn n2588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2578);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12076);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32708);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60894);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88472);
									s.is_empty()
								}
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
			super::push(out, 12077);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32709);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60895);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 60896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12078);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 32710);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60897);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88474);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109618);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дам.
pub(crate) fn n2589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2579);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12079);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32711);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 32712);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60898);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 32714);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60900);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88476);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109619);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12080);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32715);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60901);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88477);
									s.is_empty()
								}
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
			super::push(out, 12081);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32716);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зам.
pub(crate) fn n2590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2580);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12082);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 12083);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32717);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иам.
pub(crate) fn n2591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2581);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12084);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32718);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12085);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32719);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12086);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32720);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60903);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 32721);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12087);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кам.
pub(crate) fn n2592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2582);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12088);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32722);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60905);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88479);
									s.is_empty()
								}
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
			super::push(out, 12089);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32723);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88480);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лам.
pub(crate) fn n2593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2583);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12090);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32724);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32725);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60907);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 60908);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88481);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109620);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123488);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12091);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12092);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12093);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32728);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60910);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88482);
									s.is_empty()
								}
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
			super::push(out, 12094);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32729);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 60911);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88483);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 109621);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 88484);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109622);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 12096);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 12097);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32731);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60912);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88485);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109623);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123489);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мам.
pub(crate) fn n2594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2584);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12098);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12099);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32733);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нам.
pub(crate) fn n2595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2585);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 12100);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32734);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 60914);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88486);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109624);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12101);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32735);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60915);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88487);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 109625);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 123490);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пам.
pub(crate) fn n2596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2586);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 12102);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 12103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рам.
pub(crate) fn n2597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2587);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12104);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32736);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 32737);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60916);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32738);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 60917);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88489);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109626);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123491);
													match s {
														[] => true,
														['к', s @ ..] => super::t19::n133203(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32739);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 12105);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 12106);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 12107);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12108);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 32741);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60920);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88490);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109627);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 12109);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32742);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 12110);
			match s {
				[] => true,
				['а', 'б', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 60922);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 60923);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 88491);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109628);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123492);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 12111);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 12112);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 12113);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 12114);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 12115);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 12116);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сам.
pub(crate) fn n2598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2588);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12117);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12118);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 12119);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12120);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32745);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -там.
pub(crate) fn n2599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2589);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 12121);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 12122);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32747);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 12123);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12124);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32749);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12125);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12126);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32751);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уам.
pub(crate) fn n2600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2590);
	s.is_empty()
}

/// Words ending -хам.
pub(crate) fn n2601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2591);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12127);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 32752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 12128);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 32753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 12129);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32754);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60928);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12130);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32755);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 12131);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32756);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60930);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чам.
pub(crate) fn n2602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2592);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 12132);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32757);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60931);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шам.
pub(crate) fn n2603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2593);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ем.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2594);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12134);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32758);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60932);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 88493);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32759);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60933);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 88494);
											s.is_empty()
										}
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
					super::push(out, 12135);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32760);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 60934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12136);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32761);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32762);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 32763);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 60935);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 88495);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2595);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12137);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32764);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60936);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 88496);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 109629);
													match s {
														[] => true,
														['н', s @ ..] => super::t19::n124776(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2596);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12138);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32765);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 60937);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88497);
											s.is_empty()
										}
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
					super::push(out, 12139);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32766);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 60938);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88498);
											s.is_empty()
										}
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
					super::push(out, 12140);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32767);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 60939);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88499);
											s.is_empty()
										}
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
					super::push(out, 12141);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32768);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 60940);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88500);
											s.is_empty()
										}
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
					super::push(out, 12142);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 12143);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32770);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 60941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32771);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 60942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 12144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2597);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 2598);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2599);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12146);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32772);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 60943);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88501);
											s.is_empty()
										}
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
					super::push(out, 12147);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12148);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32773);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 60944);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 88502);
											match s {
												[] => true,
												['ч', '\u{1}', s @ ..] => {
													super::push(out, 109630);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2600);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12149);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32774);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 60945);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 88503);
											s.is_empty()
										}
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
					super::push(out, 12150);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2601);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12151);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32776);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 60946);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88504);
											s.is_empty()
										}
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
					super::push(out, 12152);
					match s {
						[] => true,
						['г', s @ ..] => super::t19::n33065(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t19::n12208(s, out),
				['ш', s @ ..] => super::t19::n12209(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t19::n2612(s, out),
		['н', s @ ..] => super::t19::n2613(s, out),
		['о', s @ ..] => super::t19::n2614(s, out),
		['р', s @ ..] => super::t19::n2615(s, out),
		['с', s @ ..] => super::t19::n2616(s, out),
		['т', s @ ..] => super::t19::n2617(s, out),
		['у', s @ ..] => super::t19::n2618(s, out),
		['ф', s @ ..] => super::t19::n2619(s, out),
		['х', s @ ..] => super::t19::n2620(s, out),
		['ъ', s @ ..] => super::t19::n2621(s, out),
		['ы', s @ ..] => super::t19::n2622(s, out),
		['ь', s @ ..] => super::t19::n2623(s, out),
		_ => false,
	}
}

/// Words ending -зм.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2614);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12178);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12179);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32804);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 60963);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88509);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 109633);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12180);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t19::n2625(s, out),
		_ => false,
	}
}

/// Words ending -им.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2616);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12198);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61044);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', '\u{1}', s @ ..] => {
							super::push(out, 32878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12199);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32879);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 61045);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 88583);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 109683);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 88584);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 109684);
													match s {
														[] => true,
														['е', s @ ..] => super::t19::n124819(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12200);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32880);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 61046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 12201);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32881);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 61047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 12202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32882);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 32883);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 61048);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 88585);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 109685);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2617);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 12203);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2618);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 12204);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32885);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 61049);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 88586);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2619);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12205);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2620);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12206);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32887);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 32888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12207);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32889);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61051);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 88587);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109686);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12208);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32890);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 61052);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88588);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 61053);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 32891);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 61054);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 88589);
											s.is_empty()
										}
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
					super::push(out, 12209);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2621);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12210);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 12211);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12212);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 12213);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61055);
									s.is_empty()
								}
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
			super::push(out, 2622);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12214);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32895);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 61056);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88590);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 109687);
													match s {
														[] => true,
														['а', s @ ..] => super::t19::n124820(s, out),
														_ => false,
													}
												}
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
							super::push(out, 32896);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61057);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 12215);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12216);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 32898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12217);
					s.is_empty()
				}
				['я', s @ ..] => super::t19::n12278(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t19::n2633(s, out),
		['л', s @ ..] => super::t19::n2634(s, out),
		['м', s @ ..] => super::t19::n2635(s, out),
		['н', s @ ..] => super::t19::n2636(s, out),
		['п', s @ ..] => super::t19::n2637(s, out),
		['р', s @ ..] => super::t19::n2638(s, out),
		['с', s @ ..] => super::t19::n2639(s, out),
		['т', s @ ..] => super::t19::n2640(s, out),
		['у', s @ ..] => super::t19::n2641(s, out),
		['ф', s @ ..] => super::t19::n2642(s, out),
		['х', s @ ..] => super::t19::n2643(s, out),
		['ц', s @ ..] => super::t19::n2644(s, out),
		['ч', s @ ..] => super::t19::n2645(s, out),
		['ш', s @ ..] => super::t19::n2646(s, out),
		_ => false,
	}
}

/// Words ending -йм.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2637);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12272);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 12273);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12274);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12275);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 32973);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 61117);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88629);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 109710);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t19::n124830(s, out),
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
									super::push(out, 61118);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 88630);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109711);
													match s {
														[] => true,
														['р', s @ ..] => super::t19::n124831(s, out),
														_ => false,
													}
												}
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
							super::push(out, 32974);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 61119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 12276);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2638);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12277);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 32976);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 61120);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											super::push(out, 88631);
											s.is_empty()
										}
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
					super::push(out, 12278);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 32977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 12279);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32978);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 61121);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 88632);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 109712);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лм.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2640);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12280);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2641);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 12281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мм.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2642);
			match s {
				[] => true,
				['р', 'г', 'о', s @ ..] => {
					super::push(out, 12282);
					match s {
						[] => true,
						['л', 'е', s @ ..] => {
							super::push(out, 32981);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 32982);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 61122);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88633);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109713);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12283);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нм.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ом.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2644);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 12284);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32983);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 61123);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 88634);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109714);
													match s {
														[] => true,
														['з', s @ ..] => super::t19::n124832(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12285);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32984);
							match s {
								[] => true,
								['а', '-', 'а', s @ ..] => {
									super::push(out, 61124);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 88635);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 109715);
													match s {
														[] => true,
														['н', s @ ..] => super::t19::n124833(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2645);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12286);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12287);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12288);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 12289);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32986);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 61125);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 88636);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 109716);
													match s {
														[] => true,
														['и', s @ ..] => super::t19::n124834(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12290);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 32987);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 61126);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 88637);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 109717);
													match s {
														[] => true,
														['а', s @ ..] => super::t19::n124835(s, out),
														_ => false,
													}
												}
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
							super::push(out, 32988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 32989);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12291);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32990);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 61129);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 88638);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 109718);
													match s {
														[] => true,
														['и', s @ ..] => super::t19::n124836(s, out),
														_ => false,
													}
												}
												['у', s @ ..] => {
													super::push(out, 109719);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12292);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32991);
							match s {
								[] => true,
								['д', s @ ..] => super::t19::n61782(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t19::n33292(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t19::n12354(s, out),
				['и', s @ ..] => super::t19::n12355(s, out),
				['л', s @ ..] => super::t19::n12356(s, out),
				['м', s @ ..] => super::t19::n12357(s, out),
				['н', s @ ..] => super::t19::n12358(s, out),
				['о', s @ ..] => super::t19::n12359(s, out),
				['р', s @ ..] => super::t19::n12360(s, out),
				['т', s @ ..] => super::t19::n12361(s, out),
				['э', s @ ..] => super::t19::n12362(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t19::n2657(s, out),
		['и', s @ ..] => super::t19::n2658(s, out),
		['й', s @ ..] => super::t19::n2659(s, out),
		['к', s @ ..] => super::t19::n2660(s, out),
		['л', s @ ..] => super::t19::n2661(s, out),
		['н', s @ ..] => super::t19::n2662(s, out),
		['о', s @ ..] => super::t19::n2663(s, out),
		['п', s @ ..] => super::t19::n2664(s, out),
		['р', s @ ..] => super::t19::n2665(s, out),
		['с', s @ ..] => super::t19::n2666(s, out),
		['т', s @ ..] => super::t19::n2667(s, out),
		['х', s @ ..] => super::t19::n2668(s, out),
		_ => false,
	}
}

/// Words ending -рм.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2659);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12348);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33063);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 61200);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88681);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 109747);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12349);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 33064);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61201);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88682);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 109748);
													s.is_empty()
												}
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
							super::push(out, 33065);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61202);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 88683);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 109749);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12350);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12351);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12352);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 33067);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61203);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 88684);
											s.is_empty()
										}
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
					super::push(out, 12353);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 33068);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 61204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 12354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2660);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2661);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33069);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33070);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 61205);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 33071);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 61206);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88686);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 109750);
													match s {
														[] => true,
														['к', s @ ..] => super::t19::n124848(s, out),
														_ => false,
													}
												}
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
							super::push(out, 33072);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 61207);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88687);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 109751);
													match s {
														[] => true,
														['п', s @ ..] => super::t19::n124849(s, out),
														_ => false,
													}
												}
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
							super::push(out, 33073);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 33074);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 61208);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 33075);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 61209);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88688);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 109752);
													s.is_empty()
												}
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
							super::push(out, 33076);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 61210);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88689);
											s.is_empty()
										}
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
					super::push(out, 12357);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2662);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12358);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -см.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 2663);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тм.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2664);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12360);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ум.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2665);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2666);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12361);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 33078);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61211);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 88690);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 109753);
													s.is_empty()
												}
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
							super::push(out, 33079);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 61212);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 88691);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 109754);
													match s {
														[] => true,
														['а', s @ ..] => super::t19::n124850(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2667);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 12363);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33081);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 61213);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 88692);
											s.is_empty()
										}
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
					super::push(out, 12364);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12365);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 12366);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2668);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 12367);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 33085);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 61215);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 88693);
											s.is_empty()
										}
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
					super::push(out, 12368);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 33086);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61216);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 12369);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33087);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 61217);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 88694);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2669);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 12370);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 33088);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 61218);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88695);
											s.is_empty()
										}
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
					super::push(out, 12371);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 33089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12372);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 33090);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 61219);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 88696);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109755);
													s.is_empty()
												}
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
							super::push(out, 33091);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 61220);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88697);
											s.is_empty()
										}
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
					super::push(out, 12373);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33092);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 61221);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88698);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 33093);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 61222);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12374);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33094);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 33095);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 33096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2670);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12375);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 33097);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 61223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12376);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 33098);
							match s {
								[] => true,
								['о', s @ ..] => super::t19::n61877(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t19::n12440(s, out),
				['н', s @ ..] => super::t19::n12441(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t19::n2681(s, out),
		['з', s @ ..] => super::t19::n2682(s, out),
		['и', s @ ..] => super::t19::n2683(s, out),
		['к', s @ ..] => super::t19::n2684(s, out),
		['л', s @ ..] => super::t19::n2685(s, out),
		['м', s @ ..] => super::t19::n2686(s, out),
		['н', s @ ..] => super::t19::n2687(s, out),
		['о', s @ ..] => super::t19::n2688(s, out),
		['п', s @ ..] => super::t19::n2689(s, out),
		['р', s @ ..] => super::t19::n2690(s, out),
		['с', s @ ..] => super::t19::n2691(s, out),
		['т', s @ ..] => super::t19::n2692(s, out),
		['у', s @ ..] => super::t19::n2693(s, out),
		['х', s @ ..] => super::t19::n2694(s, out),
		['ч', s @ ..] => super::t19::n2695(s, out),
		['ш', s @ ..] => super::t19::n2696(s, out),
		_ => false,
	}
}

/// Words ending -фм.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2687);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ым.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2688);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2689);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2690);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12451);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 33178);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33179);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 33180);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 61280);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 88736);
											s.is_empty()
										}
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
					super::push(out, 12453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2691);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12454);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33182);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61281);
									s.is_empty()
								}
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
			super::push(out, 2692);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12455);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33183);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 12456);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 33184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 2693);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12457);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьм.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2694);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12458);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12459);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 33185);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 61283);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 88737);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 109778);
													match s {
														[] => true,
														['в', s @ ..] => super::t19::n124865(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12460);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12461);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33186);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 61284);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 88738);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 109779);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ь', s @ ..] => {
									super::push(out, 61285);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 88739);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 109780);
													match s {
														[] => true,
														['м', s @ ..] => super::t19::n124866(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эм.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2695);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2696);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12463);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33187);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 61286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88740);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юм.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2697);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2698);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2699);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12466);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33190);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 61287);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 88741);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 109781);
													match s {
														[] => true,
														['ь', s @ ..] => super::t19::n124867(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2700);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2701);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 12468);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 61288);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 88742);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 109782);
													match s {
														[] => true,
														['т', s @ ..] => super::t19::n124868(s, out),
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
									super::push(out, 61289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ям.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 2702);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2703);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12469);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33191);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 61290);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88743);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2704);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12470);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33192);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 33193);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 61291);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88744);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 109783);
													match s {
														[] => true,
														['а', s @ ..] => super::t19::n124869(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2705);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12471);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33194);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 61292);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 61293);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12472);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33195);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 61294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88745);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ан.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2708);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 12474);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 33197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 12475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33198);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 33199);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 33200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 33201);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 33202);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 33203);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 33204);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 33205);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 12476);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33206);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 61299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 12477);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33208);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 33209);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 61300);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 88746);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 33210);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 61301);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 88747);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109784);
													s.is_empty()
												}
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
							super::push(out, 33211);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 12479);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33212);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 61303);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 88748);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 33213);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 61304);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88749);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109785);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12480);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 33214);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 61305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12481);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 33215);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61306);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 88750);
											s.is_empty()
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 61307);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 88751);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 33216);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 61308);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 88752);
											s.is_empty()
										}
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
					super::push(out, 12482);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33217);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 61309);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 88753);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 109786);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 61310);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 88754);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t19::n12546(s, out),
				['н', s @ ..] => super::t19::n12547(s, out),
				['о', s @ ..] => super::t19::n12548(s, out),
				['р', s @ ..] => super::t19::n12549(s, out),
				['т', s @ ..] => super::t19::n12550(s, out),
				['у', s @ ..] => super::t19::n12551(s, out),
				['х', s @ ..] => super::t19::n12552(s, out),
				['ь', s @ ..] => super::t19::n12553(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t19::n2719(s, out),
		['г', s @ ..] => super::t19::n2720(s, out),
		['д', s @ ..] => super::t19::n2721(s, out),
		['е', s @ ..] => super::t19::n2722(s, out),
		['ж', s @ ..] => super::t19::n2723(s, out),
		['з', s @ ..] => super::t19::n2724(s, out),
		['и', s @ ..] => super::t20::n2725(s, out),
		['й', s @ ..] => super::t20::n2726(s, out),
		['к', s @ ..] => super::t20::n2727(s, out),
		['л', s @ ..] => super::t20::n2728(s, out),
		['м', s @ ..] => super::t20::n2729(s, out),
		['н', s @ ..] => super::t20::n2730(s, out),
		['о', s @ ..] => super::t20::n2731(s, out),
		['п', s @ ..] => super::t20::n2732(s, out),
		['р', s @ ..] => super::t20::n2733(s, out),
		['с', s @ ..] => super::t20::n2734(s, out),
		['т', s @ ..] => super::t20::n2735(s, out),
		['у', s @ ..] => super::t20::n2736(s, out),
		['ф', s @ ..] => super::t20::n2737(s, out),
		['х', s @ ..] => super::t20::n2738(s, out),
		['ц', s @ ..] => super::t20::n2739(s, out),
		['ч', s @ ..] => super::t20::n2740(s, out),
		['ш', s @ ..] => super::t20::n2741(s, out),
		_ => false,
	}
}

/// Words ending -ен.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2734);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12734);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12735);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 33736);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 61832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12736);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33737);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 61833);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 89054);
											s.is_empty()
										}
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
					super::push(out, 12737);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 33738);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 61834);
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
						['р', s @ ..] => {
							super::push(out, 33739);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 61835);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 89055);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 109941);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12739);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 33740);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 61836);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 89056);
											s.is_empty()
										}
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
					super::push(out, 12740);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 33741);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 33742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 61837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 12741);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 33743);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 61838);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 89057);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 109942);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12742);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2735);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12743);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33745);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 61839);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 89058);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 109943);
													match s {
														[] => true,
														['е', s @ ..] => super::t20::n124921(s, out),
														_ => false,
													}
												}
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
							super::push(out, 33746);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 61840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12744);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 33747);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 61841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 33748);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 33749);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 61842);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 89059);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 109944);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12746);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 33750);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 61843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12747);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 12748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t20::n2746(s, out),
		['д', s @ ..] => {
			super::push(out, 2737);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t20::n34099(s, out),
						['б', s @ ..] => super::t20::n34100(s, out),
						['л', s @ ..] => super::t20::n34101(s, out),
						_ => false,
					}
				}
				['б', s @ ..] => super::t20::n12828(s, out),
				['з', s @ ..] => super::t20::n12829(s, out),
				['и', s @ ..] => super::t20::n12830(s, out),
				['й', s @ ..] => super::t20::n12831(s, out),
				['к', s @ ..] => super::t20::n12832(s, out),
				['н', s @ ..] => super::t20::n12833(s, out),
				['о', s @ ..] => super::t20::n12834(s, out),
				['р', s @ ..] => super::t20::n12835(s, out),
				['ь', s @ ..] => super::t20::n12836(s, out),
				['э', s @ ..] => super::t20::n12837(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t20::n2748(s, out),
		['з', s @ ..] => super::t20::n2749(s, out),
		['и', s @ ..] => super::t20::n2750(s, out),
		['й', s @ ..] => super::t20::n2751(s, out),
		['к', s @ ..] => super::t20::n2752(s, out),
		['л', s @ ..] => super::t20::n2753(s, out),
		['м', s @ ..] => super::t20::n2754(s, out),
		['н', s @ ..] => super::t20::n2755(s, out),
		['о', s @ ..] => super::t20::n2756(s, out),
		['п', s @ ..] => super::t20::n2757(s, out),
		['р', s @ ..] => super::t20::n2758(s, out),
		['с', s @ ..] => super::t20::n2759(s, out),
		['т', s @ ..] => super::t20::n2760(s, out),
		['у', 'р', '\u{1}', s @ ..] => super::t20::n2761(s, out),
		['ф', s @ ..] => super::t20::n2762(s, out),
		['х', s @ ..] => super::t20::n2763(s, out),
		['ц', s @ ..] => super::t20::n2764(s, out),
		['ч', s @ ..] => super::t20::n2765(s, out),
		['ш', s @ ..] => super::t20::n2766(s, out),
		['щ', s @ ..] => super::t20::n2767(s, out),
		['ь', s @ ..] => super::t20::n2768(s, out),
		_ => false,
	}
}

/// Words ending -ин.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2761);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12904);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 34000);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 62093);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 89215);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 110014);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12905);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12906);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 2762);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12907);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 12908);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34003);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 62094);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89216);
											s.is_empty()
										}
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
					super::push(out, 12909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34004);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 34005);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62095);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 89217);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 110015);
													match s {
														[] => true,
														['н', s @ ..] => super::t20::n124957(s, out),
														_ => false,
													}
												}
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
							super::push(out, 34006);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 62096);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 34007);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 62097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12910);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2763);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12912);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34008);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 12913);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34009);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 62099);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 89219);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 110016);
													match s {
														[] => true,
														['х', s @ ..] => super::t20::n124958(s, out),
														_ => false,
													}
												}
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
							super::push(out, 34010);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 62100);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 89220);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 110017);
													match s {
														[] => true,
														['х', s @ ..] => super::t20::n124959(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 12914);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 34011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12915);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34012);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 62101);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 89221);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ж', s @ ..] => {
									super::push(out, 62102);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89222);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 34013);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 62103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12916);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34014);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 62104);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89223);
											s.is_empty()
										}
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
					super::push(out, 12917);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 12918);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34016);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62105);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89224);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t20::n2774(s, out),
		['г', s @ ..] => super::t20::n2775(s, out),
		['д', s @ ..] => super::t20::n2776(s, out),
		['е', s @ ..] => {
			super::push(out, 2767);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12957);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12958);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12959);
					s.is_empty()
				}
				['ф', s @ ..] => super::t20::n13031(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t20::n2778(s, out),
		['з', s @ ..] => super::t20::n2779(s, out),
		['й', s @ ..] => {
			super::push(out, 2770);
			match s {
				[] => true,
				['и', s @ ..] => super::t20::n13048(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t20::n2781(s, out),
		['л', s @ ..] => super::t20::n2782(s, out),
		['м', s @ ..] => super::t20::n2783(s, out),
		['н', s @ ..] => super::t20::n2784(s, out),
		['о', s @ ..] => super::t20::n2785(s, out),
		['п', s @ ..] => super::t20::n2786(s, out),
		['р', s @ ..] => super::t20::n2787(s, out),
		['с', s @ ..] => super::t20::n2788(s, out),
		['т', s @ ..] => super::t20::n2789(s, out),
		['у', s @ ..] => super::t20::n2790(s, out),
		['ф', s @ ..] => super::t20::n2791(s, out),
		['х', s @ ..] => super::t20::n2792(s, out),
		['ц', s @ ..] => super::t20::n2793(s, out),
		['ч', s @ ..] => super::t20::n2794(s, out),
		['ш', s @ ..] => super::t20::n2795(s, out),
		['я', s @ ..] => super::t20::n2796(s, out),
		_ => false,
	}
}

/// Words ending -драглайн.
pub(crate) fn n111368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110200);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 123768);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кэролайн.
pub(crate) fn n111369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110201);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 123769);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фрейлейн.
pub(crate) fn n111370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110202);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 123770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уссейн.
pub(crate) fn n63189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62533);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 89540);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усейн.
pub(crate) fn n34748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34416);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 62534);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 62535);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тейн.
pub(crate) fn n13203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13132);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 34417);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фейн.
pub(crate) fn n13204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13133);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 34418);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62536);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89541);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110203);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чейн.
pub(crate) fn n13205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13134);
	s.is_empty()
}

/// Words ending -ойн.
pub(crate) fn n2799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2789);
	s.is_empty()
}

/// Words ending -эйн.
pub(crate) fn n2800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2790);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 13135);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яйн.
pub(crate) fn n2801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2791);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13136);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 34420);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62538);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89542);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110204);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лн.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2792);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 13137);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мн.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 2793);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нн.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2794);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13139);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 13140);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34422);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 34423);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 34424);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 62539);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 89543);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 34425);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 62540);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 89544);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 110205);
													match s {
														[] => true,
														['ш', s @ ..] => super::t20::n125055(s, out),
														_ => false,
													}
												}
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
							super::push(out, 34426);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 62541);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 89545);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110206);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 62542);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 89546);
											s.is_empty()
										}
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
					super::push(out, 13141);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13142);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 34427);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13143);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 34428);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62543);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 89547);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 110207);
													match s {
														[] => true,
														['с', s @ ..] => super::t20::n125056(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 13144);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 34429);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 62544);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89548);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2795);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2796);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 13146);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 34430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13147);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34431);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62545);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 89549);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110208);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2797);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 13150);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 34433);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 62546);
									s.is_empty()
								}
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
			super::push(out, 2798);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13151);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 13152);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34434);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 62547);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 89550);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110209);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -он.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2799);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13153);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34435);
							match s {
								[] => true,
								['ф', '\u{1}', s @ ..] => {
									super::push(out, 62548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 13154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34436);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 62549);
									s.is_empty()
								}
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
			super::push(out, 2800);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 13155);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 34437);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 62550);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89551);
											s.is_empty()
										}
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
					super::push(out, 13156);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34438);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62551);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 89552);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110210);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 62552);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 89553);
											s.is_empty()
										}
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
					super::push(out, 13157);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 34439);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 62553);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89554);
											s.is_empty()
										}
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
					super::push(out, 13158);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 34440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 13159);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13160);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13161);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 34441);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 34442);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 62554);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 89555);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110211);
													s.is_empty()
												}
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
							super::push(out, 34443);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 62555);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89556);
											s.is_empty()
										}
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
					super::push(out, 13162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2801);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13163);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 34444);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 34445);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 62556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13164);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34446);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 62557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 34447);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 62558);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 89557);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 110212);
													match s {
														[] => true,
														['д', s @ ..] => super::t20::n125057(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', 'у', s @ ..] => {
									super::push(out, 62559);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 89558);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110213);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13165);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 34448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t20::n2812(s, out),
		['д', s @ ..] => super::t20::n2813(s, out),
		['е', s @ ..] => {
			super::push(out, 2804);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 34502);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 62624);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89611);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 34503);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 62625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34504);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 34505);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 34506);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 62626);
									match s {
										[] => true,
										['а', s @ ..] => super::t20::n90571(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => super::t20::n34842(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t20::n13266(s, out),
				['н', s @ ..] => super::t20::n13267(s, out),
				['п', s @ ..] => super::t20::n13268(s, out),
				['р', s @ ..] => super::t20::n13269(s, out),
				['т', s @ ..] => super::t20::n13270(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t20::n2815(s, out),
		['з', s @ ..] => super::t21::n2816(s, out),
		['и', s @ ..] => super::t21::n2817(s, out),
		['й', s @ ..] => super::t21::n2818(s, out),
		['к', s @ ..] => super::t21::n2819(s, out),
		['л', s @ ..] => super::t21::n2820(s, out),
		['м', s @ ..] => super::t21::n2821(s, out),
		['н', s @ ..] => super::t21::n2822(s, out),
		['п', s @ ..] => super::t21::n2823(s, out),
		['р', s @ ..] => super::t21::n2824(s, out),
		['с', s @ ..] => super::t21::n2825(s, out),
		['т', s @ ..] => super::t21::n2826(s, out),
		['ф', s @ ..] => super::t21::n2827(s, out),
		['х', s @ ..] => super::t21::n2828(s, out),
		['ч', s @ ..] => super::t21::n2829(s, out),
		['ш', s @ ..] => super::t21::n2830(s, out),
		['ь', s @ ..] => super::t21::n2831(s, out),
		['ю', s @ ..] => super::t21::n2832(s, out),
		_ => false,
	}
}

/// Words ending -рн.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2823);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13352);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34856);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 62956);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 89847);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 110380);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13353);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34857);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62957);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89848);
											s.is_empty()
										}
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
					super::push(out, 13354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34858);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 34859);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 62958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13355);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13356);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34860);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62959);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 89849);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 110381);
													match s {
														[] => true,
														['с', s @ ..] => super::t21::n125155(s, out),
														_ => false,
													}
												}
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
							super::push(out, 34861);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 62960);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89850);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 89851);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 110382);
													match s {
														[] => true,
														['а', s @ ..] => super::t21::n125156(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 89852);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 110383);
													match s {
														[] => true,
														['к', s @ ..] => super::t21::n125157(s, out),
														_ => false,
													}
												}
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
							super::push(out, 34862);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 62961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 34863);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 62962);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 89853);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 62963);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 89854);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110384);
													s.is_empty()
												}
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
							super::push(out, 34864);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 62964);
									match s {
										[] => true,
										['п', 'и', s @ ..] => {
											super::push(out, 89855);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 110385);
													match s {
														[] => true,
														['н', s @ ..] => super::t21::n125158(s, out),
														_ => false,
													}
												}
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
							super::push(out, 34865);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 62965);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 89856);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 110386);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13357);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 34866);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 62966);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 89857);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2824);
			match s {
				[] => true,
				['б', s @ ..] => super::t21::n13431(s, out),
				['г', s @ ..] => super::t21::n13432(s, out),
				['к', s @ ..] => {
					super::push(out, 13360);
					match s {
						[] => true,
						['п', s @ ..] => super::t21::n35211(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t21::n13434(s, out),
				['ь', s @ ..] => super::t21::n13435(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t21::n2835(s, out),
		['ю', s @ ..] => super::t21::n2836(s, out),
		_ => false,
	}
}

/// Words ending -ун.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2827);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34877);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 34878);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 62974);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 34879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13367);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 34880);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 62975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13368);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 34881);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 34882);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 34883);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 62976);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 89862);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 110391);
													match s {
														[] => true,
														['а', s @ ..] => super::t21::n125161(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2828);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13369);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13370);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 34884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 34885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13372);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34886);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 62977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13373);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 34887);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 62978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 13374);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 34888);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2829);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13375);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 34889);
							s.is_empty()
						}
						['л', 'п', '\u{1}', s @ ..] => {
							super::push(out, 34890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13376);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 34891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13377);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 34892);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 62979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13378);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 34893);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 62980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 13379);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34894);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 62981);
									s.is_empty()
								}
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
			super::push(out, 2830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13380);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 13381);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 34895);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 62982);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 89863);
											s.is_empty()
										}
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
					super::push(out, 13382);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 34896);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 34897);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 62983);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89864);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 89865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 34898);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 62984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13383);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 34899);
							match s {
								[] => true,
								['-', s @ ..] => super::t21::n63645(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 34900);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 62986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t21::n13457(s, out),
				['и', s @ ..] => super::t21::n13458(s, out),
				['л', s @ ..] => super::t21::n13459(s, out),
				['н', s @ ..] => super::t21::n13460(s, out),
				['р', s @ ..] => super::t21::n13461(s, out),
				['у', s @ ..] => super::t21::n13462(s, out),
				['ы', s @ ..] => super::t21::n13463(s, out),
				['я', s @ ..] => super::t21::n13464(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t21::n2841(s, out),
		['е', s @ ..] => super::t21::n2842(s, out),
		['ж', s @ ..] => super::t21::n2843(s, out),
		['з', s @ ..] => super::t21::n2844(s, out),
		['и', s @ ..] => super::t21::n2845(s, out),
		['к', s @ ..] => super::t21::n2846(s, out),
		['л', s @ ..] => super::t21::n2847(s, out),
		['м', s @ ..] => super::t21::n2848(s, out),
		['н', s @ ..] => super::t21::n2849(s, out),
		['о', s @ ..] => super::t21::n2850(s, out),
		['п', s @ ..] => super::t21::n2851(s, out),
		['р', s @ ..] => super::t21::n2852(s, out),
		['с', s @ ..] => super::t21::n2853(s, out),
		['т', s @ ..] => super::t21::n2854(s, out),
		['ф', s @ ..] => super::t21::n2855(s, out),
		['х', s @ ..] => super::t21::n2856(s, out),
		['ц', s @ ..] => super::t21::n2857(s, out),
		['ч', s @ ..] => super::t21::n2858(s, out),
		['ш', s @ ..] => super::t21::n2859(s, out),
		['щ', s @ ..] => super::t21::n2860(s, out),
		_ => false,
	}
}

/// Words ending -шн.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2851);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13476);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35028);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63084);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 89903);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2852);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13477);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13478);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35030);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63085);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 89904);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 110409);
													match s {
														[] => true,
														['е', s @ ..] => super::t21::n125171(s, out),
														_ => false,
													}
												}
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
							super::push(out, 35031);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 63086);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 89905);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 110410);
													match s {
														[] => true,
														['у', s @ ..] => super::t21::n125172(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2853);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13479);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 13480);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 35033);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63087);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 89906);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 110411);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2854);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13481);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35034);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 35035);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63088);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 89907);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 110412);
													match s {
														[] => true,
														['э', s @ ..] => super::t21::n125173(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 13482);
					match s {
						[] => true,
						['ф', '-', 'н', s @ ..] => {
							super::push(out, 35036);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63089);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 89908);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110413);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2855);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2856);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13484);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35037);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 89909);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 110414);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', s @ ..] => {
											super::push(out, 89910);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 110415);
													match s {
														[] => true,
														['с', s @ ..] => super::t21::n125174(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2857);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13485);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35038);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63090);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 89911);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 110416);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2858);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ын.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13486);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 35039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 13487);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13488);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2860);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 13489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2861);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 13490);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 2862);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35042);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 35043);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 63091);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 89912);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2863);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 13492);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35044);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63092);
									s.is_empty()
								}
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
			super::push(out, 2864);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13493);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35045);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 63093);
									s.is_empty()
								}
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
			super::push(out, 2865);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13494);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2866);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13496);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 13497);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35046);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63094);
									s.is_empty()
								}
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
			super::push(out, 2867);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13498);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13499);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13500);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35048);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 63095);
									s.is_empty()
								}
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
			super::push(out, 2868);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13501);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35049);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63096);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 89913);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110417);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьн.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2869);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13502);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35050);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 35051);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63097);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 89914);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 110418);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13503);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35052);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 63098);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 89915);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 110419);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эн.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2870);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2871);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 2872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13505);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13506);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35053);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 13507);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35054);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63100);
									s.is_empty()
								}
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
			super::push(out, 2873);
			match s {
				[] => true,
				['е', 'с', '\u{1}', s @ ..] => {
					super::push(out, 13508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2874);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2875);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13510);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13511);
					s.is_empty()
				}
				['э', s @ ..] => {
					super::push(out, 13512);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35057);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63101);
									s.is_empty()
								}
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
			super::push(out, 2876);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13513);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 13514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 2877);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13515);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 13516);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35059);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 63102);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 89916);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2878);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юн.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2879);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13517);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35060);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 63103);
									s.is_empty()
								}
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
			super::push(out, 2880);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13518);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 13519);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35062);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63104);
									s.is_empty()
								}
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
			super::push(out, 2881);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13520);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2882);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13521);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2883);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13522);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35065);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 63105);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 89917);
											s.is_empty()
										}
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
					super::push(out, 13523);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2884);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13524);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13525);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35068);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 63106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ян.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2885);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2886);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 13526);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 13527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2887);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13528);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2888);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2889);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 13530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2890);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13531);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 35071);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63107);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13532);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13533);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35073);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 63109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 35074);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63110);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89918);
											s.is_empty()
										}
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
					super::push(out, 13534);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35075);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63111);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 89919);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 35076);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 63112);
									s.is_empty()
								}
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
			super::push(out, 2891);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13535);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13536);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13537);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35078);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 63113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 35079);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 35080);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 35081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13538);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2892);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13539);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13540);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13541);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35084);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63114);
									s.is_empty()
								}
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
			super::push(out, 2893);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13542);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35085);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13543);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35086);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 35087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2894);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13544);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13545);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35089);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63116);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 89920);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2895);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13546);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35090);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2896);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 2897);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13548);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35091);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2898);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13549);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35092);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63117);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 13550);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 35093);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 63118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13551);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 35094);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 63119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t21::n13625(s, out),
				['м', s @ ..] => super::t21::n13626(s, out),
				['н', s @ ..] => super::t21::n13627(s, out),
				['р', s @ ..] => super::t21::n13628(s, out),
				['с', s @ ..] => super::t21::n13629(s, out),
				['т', s @ ..] => super::t21::n13630(s, out),
				['ф', s @ ..] => super::t21::n13631(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -або.
pub(crate) fn n2919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2909);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13572);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 13573);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35119);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 13574);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 13575);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 13576);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13577);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -игбо.
pub(crate) fn n13653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13579);
	s.is_empty()
}

/// Words ending -ебо.
pub(crate) fn n2922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2912);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 13580);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 13581);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35123);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63143);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35124);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ибо.
pub(crate) fn n2923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2913);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13582);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 13583);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13584);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 35127);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63145);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89931);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110421);
											s.is_empty()
										}
										['ч', s @ ..] => {
											super::push(out, 110422);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13585);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35128);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63146);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89932);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110423);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123891);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13586);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35129);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63147);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йбо.
pub(crate) fn n2924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2914);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13587);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35130);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63148);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89934);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 110424);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123892);
													match s {
														[] => true,
														['д', s @ ..] => super::t21::n133398(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мбо.
pub(crate) fn n2925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2915);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13588);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35131);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 35132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13589);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35133);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 35134);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63149);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89935);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110425);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123893);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13590);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35135);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63150);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89936);
									s.is_empty()
								}
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
			super::push(out, 13591);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 13592);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 35137);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63151);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89937);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110426);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -обо.
pub(crate) fn n2926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2916);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13593);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 13594);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13595);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 13596);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35139);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 13597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рбо.
pub(crate) fn n2927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2917);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 13598);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35140);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -во.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2918);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13599);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35141);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 35142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 2919);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13600);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35143);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13601);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 35144);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 35145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13602);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35146);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63156);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 89938);
											s.is_empty()
										}
										['ц', s @ ..] => {
											super::push(out, 89939);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 35147);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 63157);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 89940);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2920);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 13603);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35148);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63158);
									s.is_empty()
								}
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
			super::push(out, 2921);
			match s {
				[] => true,
				['а', s @ ..] => super::t21::n13678(s, out),
				['д', s @ ..] => super::t21::n13679(s, out),
				['е', s @ ..] => super::t21::n13680(s, out),
				['ж', s @ ..] => super::t21::n13681(s, out),
				['з', s @ ..] => super::t21::n13682(s, out),
				['и', s @ ..] => super::t21::n13683(s, out),
				['л', s @ ..] => super::t21::n13684(s, out),
				['н', s @ ..] => super::t21::n13685(s, out),
				['р', s @ ..] => super::t21::n13686(s, out),
				['с', s @ ..] => super::t21::n13687(s, out),
				['т', s @ ..] => super::t21::n13688(s, out),
				['у', s @ ..] => super::t21::n13689(s, out),
				['ц', s @ ..] => super::t21::n13690(s, out),
				['ч', s @ ..] => super::t21::n13691(s, out),
				['ш', s @ ..] => super::t21::n13692(s, out),
				['ь', s @ ..] => super::t21::n13693(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t21::n2932(s, out),
		['й', s @ ..] => super::t21::n2933(s, out),
		['к', s @ ..] => super::t21::n2934(s, out),
		['о', s @ ..] => super::t21::n2935(s, out),
		['р', s @ ..] => super::t21::n2936(s, out),
		['т', s @ ..] => super::t21::n2937(s, out),
		['у', s @ ..] => super::t21::n2938(s, out),
		['ь', s @ ..] => super::t21::n2939(s, out),
		_ => false,
	}
}

/// Words ending -го.
pub(crate) fn n295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 294);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2930);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 13651);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35281);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 63373);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 90236);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 110795);
													match s {
														[] => true,
														['н', s @ ..] => super::t21::n125479(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2931);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13652);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35282);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 63374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 35283);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 63375);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90237);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 35284);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13653);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35285);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63377);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 90238);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110796);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13654);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13655);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35287);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 63378);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13656);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35288);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 63379);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 35289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 13657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35290);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 35291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13658);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 13659);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35292);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63381);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90239);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 110797);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 63382);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90240);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 110798);
													s.is_empty()
												}
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
							super::push(out, 35293);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63383);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 90241);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 63384);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 90242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 35294);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63385);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 90243);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 35295);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 63386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13660);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35296);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13662);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35298);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 63389);
									s.is_empty()
								}
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
			super::push(out, 2932);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13663);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2933);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13665);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35300);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 63390);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t21::n2944(s, out),
		['к', s @ ..] => {
			super::push(out, 2935);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13672);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35307);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 63395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t21::n2946(s, out),
		['о', s @ ..] => super::t21::n2947(s, out),
		['р', s @ ..] => super::t21::n2948(s, out),
		['у', s @ ..] => super::t21::n2949(s, out),
		['ь', s @ ..] => super::t21::n2950(s, out),
		['э', s @ ..] => {
			super::push(out, 2941);
			s.is_empty()
		}
		['ю', s @ ..] => super::t21::n2952(s, out),
		['я', s @ ..] => super::t21::n2953(s, out),
		_ => false,
	}
}

/// Words ending -до.
pub(crate) fn n296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 295);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2944);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13687);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35332);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63413);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90259);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 110807);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13688);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35333);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 63414);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 90260);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 110808);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2945);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 13689);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 35334);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63415);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 90261);
											s.is_empty()
										}
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
					super::push(out, 13690);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35335);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 63416);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 35336);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 63417);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90262);
											s.is_empty()
										}
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
					super::push(out, 13691);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35337);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63418);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90263);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 110809);
													match s {
														[] => true,
														['к', s @ ..] => super::t21::n125485(s, out),
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
									super::push(out, 63419);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 90264);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 110810);
													match s {
														[] => true,
														['р', s @ ..] => super::t21::n125486(s, out),
														_ => false,
													}
												}
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
							super::push(out, 35338);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63420);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90265);
											s.is_empty()
										}
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
					super::push(out, 13692);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35339);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63421);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90266);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 35340);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 63422);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 90267);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 110811);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 63423);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 90268);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 110812);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13693);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35341);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63424);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90269);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 110813);
													match s {
														[] => true,
														['д', s @ ..] => super::t21::n125487(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ф', s @ ..] => {
											super::push(out, 90270);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 110814);
													match s {
														[] => true,
														['и', s @ ..] => super::t21::n125488(s, out),
														_ => false,
													}
												}
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
							super::push(out, 35342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 13694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2946);
			match s {
				[] => true,
				['е', s @ ..] => super::t21::n13769(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t21::n2957(s, out),
		['з', s @ ..] => super::t21::n2958(s, out),
		['и', s @ ..] => super::t21::n2959(s, out),
		['й', s @ ..] => super::t21::n2960(s, out),
		['л', s @ ..] => super::t21::n2961(s, out),
		['н', s @ ..] => super::t21::n2962(s, out),
		['о', s @ ..] => super::t21::n2963(s, out),
		['р', s @ ..] => super::t21::n2964(s, out),
		['у', s @ ..] => super::t21::n2965(s, out),
		['ь', s @ ..] => super::t21::n2966(s, out),
		['э', s @ ..] => {
			super::push(out, 2957);
			s.is_empty()
		}
		['ю', s @ ..] => super::t21::n2968(s, out),
		['я', s @ ..] => super::t21::n2969(s, out),
		_ => false,
	}
}

/// Words ending -ео.
pub(crate) fn n297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13733);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35397);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63468);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 90296);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2961);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13734);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13735);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35399);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63469);
									s.is_empty()
								}
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
			super::push(out, 2962);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13736);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35400);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63470);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 90297);
											s.is_empty()
										}
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
					super::push(out, 13737);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2963);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13738);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2964);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13740);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35403);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63471);
									s.is_empty()
								}
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
			super::push(out, 2965);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13741);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13742);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35405);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 63472);
									s.is_empty()
								}
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
			super::push(out, 2966);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13743);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 13744);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35407);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63473);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90298);
											s.is_empty()
										}
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
					super::push(out, 13745);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35408);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 63474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 13746);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35409);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 63475);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90299);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 110830);
													match s {
														[] => true,
														['ш', s @ ..] => super::t21::n125494(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жо.
pub(crate) fn n298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 2967);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13747);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35410);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63476);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90300);
											s.is_empty()
										}
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
					super::push(out, 13748);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35411);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13749);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35412);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13750);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35413);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63479);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 90301);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 110831);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13751);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35414);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 63480);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 90302);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2968);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13752);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зо.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2969);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13753);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35415);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 63481);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90303);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2970);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13754);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35416);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 63482);
									s.is_empty()
								}
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
			super::push(out, 2971);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13755);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35417);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 63483);
									s.is_empty()
								}
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
			super::push(out, 2972);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13756);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 13757);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35418);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 63484);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90304);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 35419);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63485);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90305);
											s.is_empty()
										}
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
					super::push(out, 13758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2973);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13759);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2974);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13760);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35422);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 35423);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63487);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 90306);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2975);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13761);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ио.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2976);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13762);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 35424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13763);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35425);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 63488);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 90307);
											s.is_empty()
										}
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
					super::push(out, 13764);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35426);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13765);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35427);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 35428);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 63490);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90308);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110832);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2977);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13766);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35429);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 63491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 13767);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35430);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 63492);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90309);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2978);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13768);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35431);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63493);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90310);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 63494);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90311);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 110833);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13769);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35432);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13770);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35433);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63496);
									s.is_empty()
								}
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
			super::push(out, 2979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13771);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 13772);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35435);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 35436);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63497);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90312);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2980);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13773);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35437);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 63498);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 90313);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 110834);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13774);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2981);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13775);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35439);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 63499);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90314);
											s.is_empty()
										}
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
					super::push(out, 13776);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13777);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 35440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2982);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2983);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13778);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13779);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35442);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 63500);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90315);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2984);
			match s {
				[] => true,
				['а', s @ ..] => super::t21::n13854(s, out),
				['в', s @ ..] => {
					super::push(out, 13781);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13782);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 35448);
							match s {
								[] => true,
								['к', s @ ..] => super::t21::n64166(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t21::n2995(s, out),
		['т', s @ ..] => super::t21::n2996(s, out),
		['ц', s @ ..] => super::t21::n2997(s, out),
		['ч', s @ ..] => super::t21::n2998(s, out),
		_ => false,
	}
}

/// Words ending -йо.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2989);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13788);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 35454);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63507);
									s.is_empty()
								}
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
			super::push(out, 2990);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13789);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ко.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2991);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 13790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35456);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13791);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35457);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13792);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 35458);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 13793);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35459);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 13794);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35460);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 63511);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 90321);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 35461);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 63512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13795);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 35462);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13796);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35463);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 63514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 13797);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63515);
									s.is_empty()
								}
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
			super::push(out, 2992);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13798);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35465);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 63516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13799);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 35466);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 35467);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63517);
									s.is_empty()
								}
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
			super::push(out, 2993);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13800);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35468);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63518);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 90322);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2994);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13801);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35469);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 63519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13802);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35470);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 63520);
									s.is_empty()
								}
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
			super::push(out, 2995);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13803);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 13804);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 13805);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 35471);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63521);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90323);
											s.is_empty()
										}
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
					super::push(out, 13806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13807);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2996);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 13808);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2997);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13809);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35475);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63522);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 90324);
											s.is_empty()
										}
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
					super::push(out, 13810);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35476);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63523);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 90325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t21::n3008(s, out),
		['й', s @ ..] => {
			super::push(out, 2999);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13824);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35499);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63541);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90340);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 110846);
													match s {
														[] => true,
														['с', s @ ..] => super::t21::n125501(s, out),
														_ => false,
													}
												}
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
							super::push(out, 35500);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63542);
									match s {
										[] => true,
										['п', s @ ..] => super::t21::n91309(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => super::t21::n13899(s, out),
				['я', s @ ..] => super::t21::n13900(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t21::n3010(s, out),
		['л', s @ ..] => super::t21::n3011(s, out),
		['н', s @ ..] => super::t21::n3012(s, out),
		['о', s @ ..] => super::t21::n3013(s, out),
		['р', s @ ..] => super::t21::n3014(s, out),
		['с', s @ ..] => super::t21::n3015(s, out),
		['т', s @ ..] => super::t21::n3016(s, out),
		['у', s @ ..] => super::t21::n3017(s, out),
		['ц', s @ ..] => super::t21::n3018(s, out),
		['ч', s @ ..] => super::t21::n3019(s, out),
		['ш', s @ ..] => super::t21::n3020(s, out),
		['ы', s @ ..] => super::t21::n3021(s, out),
		['ь', s @ ..] => super::t21::n3022(s, out),
		['э', s @ ..] => super::t21::n3023(s, out),
		['ю', s @ ..] => super::t21::n3024(s, out),
		_ => false,
	}
}

/// Words ending -ло.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3015);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13891);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35617);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 35618);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 63701);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 35619);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63702);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 90499);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 110960);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13892);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35620);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 63703);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90500);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 110961);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t21::n125559(s, out),
														_ => false,
													}
												}
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
							super::push(out, 35621);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 35622);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63704);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 90501);
											s.is_empty()
										}
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
					super::push(out, 13893);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35623);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 35624);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 63705);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 90502);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 110962);
													s.is_empty()
												}
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
							super::push(out, 35625);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 63706);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 90503);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110963);
													s.is_empty()
												}
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
							super::push(out, 35626);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 63707);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 13894);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35627);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 63708);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 90504);
											s.is_empty()
										}
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
					super::push(out, 13895);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 35628);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 63709);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 90505);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 110964);
													s.is_empty()
												}
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
							super::push(out, 35629);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13896);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 13897);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35630);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 63711);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t21::n35972(s, out),
						['р', s @ ..] => {
							super::push(out, 35632);
							match s {
								[] => true,
								['е', s @ ..] => super::t21::n64375(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t21::n35974(s, out),
						['ш', s @ ..] => super::t21::n35975(s, out),
						['ю', s @ ..] => super::t21::n35976(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t21::n13972(s, out),
				['р', s @ ..] => super::t21::n13973(s, out),
				['с', s @ ..] => super::t21::n13974(s, out),
				['т', s @ ..] => super::t21::n13975(s, out),
				['ф', s @ ..] => super::t21::n13976(s, out),
				['х', s @ ..] => super::t21::n13977(s, out),
				['ц', s @ ..] => super::t21::n13978(s, out),
				['ч', s @ ..] => super::t21::n13979(s, out),
				['ш', s @ ..] => super::t21::n13980(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t21::n3026(s, out),
		['в', s @ ..] => super::t21::n3027(s, out),
		['г', s @ ..] => super::t21::n3028(s, out),
		['д', s @ ..] => super::t21::n3029(s, out),
		['е', s @ ..] => super::t21::n3030(s, out),
		['з', s @ ..] => super::t21::n3031(s, out),
		['и', s @ ..] => super::t21::n3032(s, out),
		['й', s @ ..] => super::t21::n3033(s, out),
		['к', s @ ..] => super::t21::n3034(s, out),
		['л', s @ ..] => super::t21::n3035(s, out),
		['м', s @ ..] => super::t21::n3036(s, out),
		['н', s @ ..] => super::t21::n3037(s, out),
		['о', s @ ..] => super::t21::n3038(s, out),
		['п', s @ ..] => super::t21::n3039(s, out),
		['р', s @ ..] => super::t21::n3040(s, out),
		['с', s @ ..] => super::t21::n3041(s, out),
		['т', s @ ..] => super::t21::n3042(s, out),
		['у', s @ ..] => super::t21::n3043(s, out),
		['ф', s @ ..] => super::t21::n3044(s, out),
		['х', s @ ..] => super::t21::n3045(s, out),
		['ш', s @ ..] => super::t21::n3046(s, out),
		['ы', s @ ..] => super::t21::n3047(s, out),
		['я', s @ ..] => super::t21::n3048(s, out),
		_ => false,
	}
}

/// Words ending -мо.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3039);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 14003);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35787);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63832);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 90567);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 111005);
													match s {
														[] => true,
														['у', s @ ..] => super::t21::n125576(s, out),
														_ => false,
													}
												}
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
							super::push(out, 35788);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 63833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 14004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 3040);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 3041);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 14006);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35790);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 35791);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 63834);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90568);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 111006);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 3042);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 14007);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35792);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63835);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 90569);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 111007);
													match s {
														[] => true,
														['ч', s @ ..] => super::t21::n125577(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14008);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35793);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 63836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 14009);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35794);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 63837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 14010);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35795);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63838);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 90570);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 111008);
													s.is_empty()
												}
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
							super::push(out, 35796);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 63839);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90571);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 111009);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14011);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 35797);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63840);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 90572);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 35798);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 63841);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 90573);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3043);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14012);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 14013);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35799);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63842);
									s.is_empty()
								}
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
			super::push(out, 3044);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14014);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 35800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3045);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 14015);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35801);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 63843);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 90574);
											s.is_empty()
										}
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
					super::push(out, 14016);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35802);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 35803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 14017);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 3046);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14018);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35805);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 63844);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 90575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => super::t21::n36147(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 14019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t21::n3057(s, out),
		['у', s @ ..] => super::t21::n3058(s, out),
		['ч', s @ ..] => super::t21::n3059(s, out),
		['ь', s @ ..] => super::t21::n3060(s, out),
		['э', s @ ..] => super::t22::n3061(s, out),
		['ю', s @ ..] => super::t22::n3062(s, out),
		_ => false,
	}
}

/// Words ending -но.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3053);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 14029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35816);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 35817);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 35818);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 63852);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90578);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 111011);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14030);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35819);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 63853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 35820);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63854);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 90579);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 35821);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63855);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 90580);
											s.is_empty()
										}
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
					super::push(out, 14031);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 35822);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63856);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 90581);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 111012);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14032);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 35823);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 63857);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90582);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 111013);
													match s {
														[] => true,
														['б', s @ ..] => super::t22::n125578(s, out),
														_ => false,
													}
												}
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
							super::push(out, 35824);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 63858);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 90583);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 35825);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63859);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 90584);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 35826);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 63860);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90585);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 111014);
													match s {
														[] => true,
														['м', s @ ..] => super::t22::n125579(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 14033);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 35827);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 63861);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 90586);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 111015);
													match s {
														[] => true,
														['а', s @ ..] => super::t22::n125580(s, out),
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
									super::push(out, 63862);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 35828);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63863);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 90587);
											s.is_empty()
										}
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
					super::push(out, 14034);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 35829);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63864);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 90588);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 35830);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 63865);
									match s {
										[] => true,
										['с', s @ ..] => super::t22::n91558(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t22::n36172(s, out),
						['у', s @ ..] => super::t22::n36173(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t22::n14109(s, out),
				['у', s @ ..] => super::t22::n14110(s, out),
				['ф', s @ ..] => super::t22::n14111(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t22::n3064(s, out),
		['г', s @ ..] => super::t22::n3065(s, out),
		['д', s @ ..] => super::t22::n3066(s, out),
		['е', s @ ..] => super::t22::n3067(s, out),
		['ж', s @ ..] => super::t22::n3068(s, out),
		['з', s @ ..] => super::t22::n3069(s, out),
		['и', s @ ..] => super::t22::n3070(s, out),
		['й', s @ ..] => super::t22::n3071(s, out),
		['к', s @ ..] => super::t22::n3072(s, out),
		['м', s @ ..] => super::t22::n3073(s, out),
		['н', s @ ..] => super::t22::n3074(s, out),
		['о', s @ ..] => super::t22::n3075(s, out),
		['р', s @ ..] => super::t22::n3076(s, out),
		['с', s @ ..] => super::t22::n3077(s, out),
		['т', s @ ..] => super::t22::n3078(s, out),
		['у', s @ ..] => super::t22::n3079(s, out),
		['х', s @ ..] => super::t22::n3080(s, out),
		['ч', s @ ..] => super::t22::n3081(s, out),
		['ш', s @ ..] => super::t22::n3082(s, out),
		['ы', s @ ..] => super::t22::n3083(s, out),
		['ь', s @ ..] => super::t22::n3084(s, out),
		['ю', s @ ..] => super::t22::n3085(s, out),
		['я', s @ ..] => super::t22::n3086(s, out),
		_ => false,
	}
}

/// Words ending -оо.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 3077);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 14105);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35963);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 63996);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 90695);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 111080);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -по.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 3078);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14106);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 14107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 35965);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 35966);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 63997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 14108);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 35967);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 63998);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 90696);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3079);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 14109);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 35968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 14110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 3080);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 14111);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 35969);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 63999);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 90697);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 111081);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 14112);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 3081);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 14113);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 35971);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64000);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 90698);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 64001);
									s.is_empty()
								}
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
			super::push(out, 3082);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14114);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 14115);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 35973);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 64002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 14116);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 35974);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 64003);
									s.is_empty()
								}
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
			super::push(out, 3083);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 14117);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 35975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 3084);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 14118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 35976);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 64004);
									s.is_empty()
								}
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
			super::push(out, 3085);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 14119);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 35977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 3086);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 14120);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 35978);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 64005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

