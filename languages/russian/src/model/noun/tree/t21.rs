use flexion::Path;

/// Words ending -зон.
pub(crate) fn n2816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2806);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13204);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34521);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 34522);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62635);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34523);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 62637);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89620);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110241);
											s.is_empty()
										}
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
					super::push(out, 34524);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13205);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13206);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62639);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 62640);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89621);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110242);
											s.is_empty()
										}
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
					super::push(out, 34526);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13207);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34528);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62643);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89622);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110243);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13208);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34529);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 34530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ион.
pub(crate) fn n2817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2807);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 13210);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 13211);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 13212);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13213);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34533);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62646);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89623);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110244);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 123784);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133341(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62647);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34534);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62648);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89625);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110245);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123785);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133342(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62649);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89626);
									s.is_empty()
								}
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
			super::push(out, 13214);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34535);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62650);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89627);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110246);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123786);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34536);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 62651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 34537);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62652);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89628);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110247);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123787);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13215);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34538);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62653);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89629);
									s.is_empty()
								}
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
			super::push(out, 13216);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34539);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62654);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89630);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110248);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123788);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'ч', s @ ..] => {
							super::push(out, 62655);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89631);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110249);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123789);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34540);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 34541);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62656);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89632);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110250);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123790);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34542);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62657);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 89633);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110251);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 123791);
													match s {
														[] => true,
														['н', s @ ..] => super::t36::n133343(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62658);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89634);
									match s {
										[] => true,
										['д', s @ ..] => super::t36::n111420(s, out),
										['т', s @ ..] => {
											super::push(out, 110253);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => super::t36::n90594(s, out),
								['ц', s @ ..] => super::t36::n90595(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t36::n13290(s, out),
		['н', s @ ..] => super::t36::n13291(s, out),
		['п', s @ ..] => super::t36::n13292(s, out),
		['р', s @ ..] => super::t36::n13293(s, out),
		['с', s @ ..] => super::t36::n13294(s, out),
		['т', s @ ..] => super::t36::n13295(s, out),
		['х', s @ ..] => super::t36::n13296(s, out),
		['ц', s @ ..] => super::t36::n13297(s, out),
		_ => false,
	}
}

/// Words ending -йон.
pub(crate) fn n2818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2808);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13225);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13226);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34569);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62694);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89669);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110278);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13227);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13228);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34570);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 34571);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кон.
pub(crate) fn n2819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2809);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13229);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13230);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34572);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62696);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 34573);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62697);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89671);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 62698);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89672);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110279);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123809);
													match s {
														[] => true,
														['е', s @ ..] => super::t36::n133352(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34574);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 62699);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34575);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 62701);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89674);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110280);
											s.is_empty()
										}
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
					super::push(out, 34576);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62702);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89675);
									s.is_empty()
								}
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
			super::push(out, 13231);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34577);
					s.is_empty()
				}
				['г', '\u{1}', s @ ..] => {
					super::push(out, 34578);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34579);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62703);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89676);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110281);
											s.is_empty()
										}
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
					super::push(out, 34580);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13232);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34581);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 34582);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 34583);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34584);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34585);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 34586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13233);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 13234);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34588);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13235);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34589);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13236);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34590);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62708);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89677);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 34591);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34592);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62710);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89678);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 110282);
											s.is_empty()
										}
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
					super::push(out, 34593);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34594);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62712);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89679);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110283);
											s.is_empty()
										}
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
					super::push(out, 34595);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62713);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 89680);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110284);
											s.is_empty()
										}
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
					super::push(out, 34596);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62714);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89681);
									s.is_empty()
								}
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
			super::push(out, 13237);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34597);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 62715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13238);
			s.is_empty()
		}
		['э', s @ ..] => {
			super::push(out, 13239);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34598);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62716);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 13240);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34599);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 34600);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62717);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89682);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110285);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 123810);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13241);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 34601);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лон.
pub(crate) fn n2820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2810);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13242);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34602);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62719);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 34603);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34604);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 62721);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89685);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110286);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 123811);
													match s {
														[] => true,
														['а', s @ ..] => super::t36::n133353(s, out),
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
									super::push(out, 89686);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110287);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 123812);
													match s {
														[] => true,
														['е', s @ ..] => super::t36::n133354(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 13243);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 13244);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13245);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34606);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62723);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89687);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110288);
											s.is_empty()
										}
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
					super::push(out, 34607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34608);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 34609);
					match s {
						[] => true,
						['к', 'а', s @ ..] => {
							super::push(out, 62724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34610);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62725);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89688);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110289);
											s.is_empty()
										}
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
					super::push(out, 34611);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 34612);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 34613);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62726);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89689);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 62727);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89690);
									s.is_empty()
								}
								['э', s @ ..] => {
									super::push(out, 89691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 34614);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13247);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34615);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 13248);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34616);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 34617);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 34618);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 34619);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 34620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13249);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34621);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62730);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 89692);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110290);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 123813);
													match s {
														[] => true,
														['т', s @ ..] => super::t36::n133355(s, out),
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
									super::push(out, 89693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 62731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34622);
					match s {
						[] => true,
						['п', s @ ..] => super::t36::n63389(s, out),
						_ => false,
					}
				}
				['э', s @ ..] => super::t36::n34958(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t36::n13323(s, out),
		['р', s @ ..] => super::t36::n13324(s, out),
		['с', s @ ..] => super::t36::n13325(s, out),
		['т', s @ ..] => super::t36::n13326(s, out),
		['у', s @ ..] => super::t36::n13327(s, out),
		['ф', s @ ..] => super::t36::n13328(s, out),
		_ => false,
	}
}

/// Words ending -мон.
pub(crate) fn n2821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2811);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13256);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13257);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34638);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 34639);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 34640);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62744);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89701);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110295);
											s.is_empty()
										}
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
					super::push(out, 34641);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 62745);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89702);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 34642);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62747);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89703);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110296);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 62748);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34643);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62749);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 34644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13258);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34645);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62750);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 89706);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110297);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13259);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34646);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62751);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 62752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 34647);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62753);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 62754);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89708);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110298);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123817);
													s.is_empty()
												}
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
							super::push(out, 62755);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89709);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110299);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 123818);
													match s {
														[] => true,
														['к', s @ ..] => super::t36::n133357(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34648);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62756);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89710);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 34649);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34650);
					match s {
						[] => true,
						['р', 'а', '\u{1}', s @ ..] => {
							super::push(out, 62758);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 62759);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 89711);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110300);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13260);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34651);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62760);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89712);
									s.is_empty()
								}
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
			super::push(out, 13261);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34652);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 34653);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62761);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 34654);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62762);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 89714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34655);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62763);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89715);
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
						['\u{1}', s @ ..] => {
							super::push(out, 62764);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 62765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13262);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34656);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62766);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89716);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 110301);
											match s {
												[] => true,
												['н', s @ ..] => super::t36::n125103(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t36::n13336(s, out),
		['о', s @ ..] => super::t36::n13337(s, out),
		['р', s @ ..] => super::t36::n13338(s, out),
		['у', s @ ..] => super::t36::n13339(s, out),
		['х', s @ ..] => super::t36::n13340(s, out),
		['ш', s @ ..] => super::t36::n13341(s, out),
		['э', s @ ..] => super::t36::n13342(s, out),
		_ => false,
	}
}

/// Words ending -нон.
pub(crate) fn n2822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2812);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13270);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34668);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 34669);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 34670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13271);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13272);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 34673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13273);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34674);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 62782);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89727);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 110304);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123820);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13274);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34675);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 62783);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89728);
									s.is_empty()
								}
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
			super::push(out, 13275);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 34677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13276);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34678);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 62785);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89729);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110305);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123821);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пон.
pub(crate) fn n2823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2813);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13277);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 13278);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 13279);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34679);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62786);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89730);
									s.is_empty()
								}
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
			super::push(out, 13280);
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					super::push(out, 34680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13281);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34681);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рон.
pub(crate) fn n2824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2814);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13282);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34682);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 34683);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 34684);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 34685);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62789);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89731);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110306);
											s.is_empty()
										}
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
					super::push(out, 34686);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62790);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89732);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110307);
											s.is_empty()
										}
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
					super::push(out, 34687);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34688);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62792);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89733);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 62793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62794);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89734);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110308);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123822);
													s.is_empty()
												}
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
							super::push(out, 62795);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89735);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110309);
											s.is_empty()
										}
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
					super::push(out, 34689);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 34690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13284);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13285);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34692);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34693);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 62798);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89736);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110310);
											s.is_empty()
										}
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
					super::push(out, 34694);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 34695);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62799);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89737);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 62800);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89738);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110311);
											s.is_empty()
										}
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
					super::push(out, 34696);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13286);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34697);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 34698);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 34699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34700);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62803);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89739);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110312);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123823);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133359(s, out),
														_ => false,
													}
												}
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
							super::push(out, 62804);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 34701);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 34702);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 34703);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62806);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 89741);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110313);
											s.is_empty()
										}
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
					super::push(out, 34704);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62807);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89742);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110314);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123824);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13287);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34705);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62808);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34706);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34707);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 34708);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 34709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t36::n63467(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => super::t36::n13361(s, out),
		['к', s @ ..] => super::t36::n13362(s, out),
		['о', s @ ..] => super::t36::n13363(s, out),
		['п', s @ ..] => super::t36::n13364(s, out),
		['р', s @ ..] => super::t36::n13365(s, out),
		['т', s @ ..] => super::t36::n13366(s, out),
		['у', s @ ..] => super::t36::n13367(s, out),
		['ф', s @ ..] => super::t36::n13368(s, out),
		['х', s @ ..] => super::t36::n13369(s, out),
		['э', s @ ..] => super::t36::n13370(s, out),
		_ => false,
	}
}

/// Words ending -сон.
pub(crate) fn n2825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2815);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13298);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13299);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34727);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 34728);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62827);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89751);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110320);
											s.is_empty()
										}
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
					super::push(out, 34729);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62828);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89752);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110321);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 123829);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13300);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34731);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62829);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 89753);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110322);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 123830);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133361(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13301);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34733);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13302);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34734);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62832);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 62833);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89755);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110323);
											s.is_empty()
										}
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
					super::push(out, 34735);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34736);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 62835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34737);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62836);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89756);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110324);
											s.is_empty()
										}
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
					super::push(out, 34738);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62837);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34739);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62838);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89758);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 62839);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89759);
									s.is_empty()
								}
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
			super::push(out, 13303);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34740);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34741);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 62841);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89760);
									s.is_empty()
								}
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
			super::push(out, 13304);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34742);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62842);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 62843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 34743);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 62844);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89762);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110326);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 62845);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 34744);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62846);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13305);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34746);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 62847);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89765);
									s.is_empty()
								}
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
			super::push(out, 13306);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34747);
					match s {
						[] => true,
						['б', s @ ..] => super::t36::n63505(s, out),
						['у', s @ ..] => {
							super::push(out, 62849);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 62850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t36::n35087(s, out),
				['о', s @ ..] => super::t36::n35088(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t36::n13380(s, out),
		['п', s @ ..] => super::t36::n13381(s, out),
		['р', s @ ..] => super::t36::n13382(s, out),
		['с', s @ ..] => super::t36::n13383(s, out),
		['т', s @ ..] => super::t36::n13384(s, out),
		['у', s @ ..] => super::t36::n13385(s, out),
		['ь', s @ ..] => super::t36::n13386(s, out),
		['я', s @ ..] => super::t36::n13387(s, out),
		_ => false,
	}
}

/// Words ending -тон.
pub(crate) fn n2826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2816);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13315);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13316);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34766);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 34767);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62871);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89781);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110335);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 123835);
													match s {
														[] => true,
														['г', s @ ..] => super::t36::n133364(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34768);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 34769);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62872);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 34770);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62873);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 89783);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110336);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 62874);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89784);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'т', s @ ..] => {
					super::push(out, 34771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13317);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34772);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 13318);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34773);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89785);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110338);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123836);
													match s {
														[] => true,
														['б', s @ ..] => super::t36::n133365(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 110339);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123837);
													match s {
														[] => true,
														['в', s @ ..] => super::t36::n133366(s, out),
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
									super::push(out, 89786);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 89787);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110340);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123838);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 89788);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 89789);
									s.is_empty()
								}
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
			super::push(out, 13319);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34774);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62876);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89790);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110341);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 123839);
													match s {
														[] => true,
														['д', s @ ..] => super::t36::n133367(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13320);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34775);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62877);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 89791);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110342);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 123840);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 89792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 34776);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62878);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89793);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110343);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 62879);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89794);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110344);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123841);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34777);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 34778);
					s.is_empty()
				}
				['ц', s @ ..] => super::t36::n35118(s, out),
				['ь', s @ ..] => super::t36::n35119(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t36::n13394(s, out),
		['й', s @ ..] => super::t36::n13395(s, out),
		['к', s @ ..] => super::t36::n13396(s, out),
		['л', s @ ..] => super::t36::n13397(s, out),
		['н', s @ ..] => super::t36::n13398(s, out),
		['о', s @ ..] => super::t36::n13399(s, out),
		['п', s @ ..] => super::t36::n13400(s, out),
		['р', s @ ..] => super::t36::n13401(s, out),
		['с', s @ ..] => super::t36::n13402(s, out),
		['т', s @ ..] => super::t36::n13403(s, out),
		['у', s @ ..] => super::t36::n13404(s, out),
		['ф', s @ ..] => super::t36::n13405(s, out),
		['х', s @ ..] => super::t37::n13406(s, out),
		['ш', s @ ..] => super::t37::n13407(s, out),
		['ь', s @ ..] => super::t37::n13408(s, out),
		['э', s @ ..] => super::t37::n13409(s, out),
		['ю', s @ ..] => super::t37::n13410(s, out),
		_ => false,
	}
}

/// Words ending -фон.
pub(crate) fn n2827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2817);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13338);
			match s {
				[] => true,
				['г', 'а', s @ ..] => {
					super::push(out, 34826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 34827);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62935);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89828);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110368);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123864);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13339);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13340);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34829);
					s.is_empty()
				}
				['л', 'е', s @ ..] => {
					super::push(out, 34830);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 62936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34831);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 34832);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 62938);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 62939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34833);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 34834);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 34835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13341);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34836);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62940);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89832);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110369);
											s.is_empty()
										}
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
					super::push(out, 34837);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62941);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89833);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110370);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 123865);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t37::n133385(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'г', s @ ..] => {
					super::push(out, 34838);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 62942);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89834);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110371);
											s.is_empty()
										}
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
					super::push(out, 34839);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34840);
					s.is_empty()
				}
				['с', 'у', s @ ..] => {
					super::push(out, 34841);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 62943);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 13342);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 34843);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 62944);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хон.
pub(crate) fn n2828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2818);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13343);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34844);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 62945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 34845);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 62946);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 89837);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 110372);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123866);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13344);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34846);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 34847);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 62947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34848);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13345);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34849);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 62949);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чон.
pub(crate) fn n2829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2819);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13346);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34850);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 62950);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шон.
pub(crate) fn n2830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2820);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13347);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 13348);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьон.
pub(crate) fn n2831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2821);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13349);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34851);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89840);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110373);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 89841);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110374);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123867);
													s.is_empty()
												}
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
							super::push(out, 62951);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89842);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110375);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 89843);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110376);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123868);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34852);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 62952);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89844);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110377);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123869);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13350);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34854);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 62953);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 62954);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89845);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110378);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123870);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13351);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34855);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62955);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89846);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110379);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юон.
pub(crate) fn n2832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2822);
	s.is_empty()
}

/// Words ending -субалтерн.
pub(crate) fn n125155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123871);
	s.is_empty()
}

/// Words ending -ач-интерн.
pub(crate) fn n125156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123872);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 132026);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 136846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коминтерн.
pub(crate) fn n125157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123873);
	s.is_empty()
}

/// Words ending -нтипаттерн.
pub(crate) fn n125158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123874);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -борн.
pub(crate) fn n13431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13358);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 34867);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62967);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 89858);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110387);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -горн.
pub(crate) fn n13432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13359);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 34868);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 62968);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89859);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110388);
									s.is_empty()
								}
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
			super::push(out, 34869);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62969);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89860);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 110389);
									s.is_empty()
								}
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
			super::push(out, 34870);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 62970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пкорн.
pub(crate) fn n35211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34871);
	s.is_empty()
}

/// Words ending -норн.
pub(crate) fn n13434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13361);
	s.is_empty()
}

/// Words ending -ьорн.
pub(crate) fn n13435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13362);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 34872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62971);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урн.
pub(crate) fn n2835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2825);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13363);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34873);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 34874);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 62972);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89861);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110390);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123876);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13364);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34875);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 62973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрн.
pub(crate) fn n2836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2826);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 13365);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34876);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -арлоттаун.
pub(crate) fn n125161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123877);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 132028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --бегун.
pub(crate) fn n63645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62985);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 89866);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 110392);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 123878);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132029);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136847);
											match s {
												[] => true,
												['щ', s @ ..] => {
													super::push(out, 139737);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -згун.
pub(crate) fn n13457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13384);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 34901);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 34902);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 34903);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62987);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -игун.
pub(crate) fn n13458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13385);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 34904);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 62988);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 34905);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62989);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лгун.
pub(crate) fn n13459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13386);
	s.is_empty()
}

/// Words ending -нгун.
pub(crate) fn n13460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13387);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34906);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 62990);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34907);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 62991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргун.
pub(crate) fn n13461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13388);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 34908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 34909);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 62993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 34910);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 62994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угун.
pub(crate) fn n13462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13389);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 34911);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 62995);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 34912);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 62996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 34913);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыгун.
pub(crate) fn n13463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13390);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 34914);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 62997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягун.
pub(crate) fn n13464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13391);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 34915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дун.
pub(crate) fn n2841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2831);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13392);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34916);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 62998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13393);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 13394);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13395);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 13396);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34918);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 62999);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89869);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110393);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13397);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34919);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 34920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 13398);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 34921);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63002);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 89870);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110394);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13399);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34922);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 63003);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еун.
pub(crate) fn n2842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2832);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 13400);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жун.
pub(crate) fn n2843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2833);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13401);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34923);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63004);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 63005);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 89872);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110395);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123879);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зун.
pub(crate) fn n2844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2834);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13402);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13403);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 13404);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34925);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34926);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63006);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89873);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110396);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13405);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34927);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13406);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34928);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иун.
pub(crate) fn n2845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2835);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 13407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кун.
pub(crate) fn n2846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2836);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13408);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13409);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34930);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34931);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63010);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 89874);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110397);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 123880);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 34932);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63011);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89875);
									s.is_empty()
								}
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
			super::push(out, 13410);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 34933);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13411);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34934);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63013);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 89876);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 110398);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 123881);
													match s {
														[] => true,
														['е', s @ ..] => super::t37::n133390(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34935);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13412);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 34936);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 63015);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 89877);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110399);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 123882);
													match s {
														[] => true,
														['ж', s @ ..] => super::t37::n133391(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 34937);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63016);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89878);
									s.is_empty()
								}
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
			super::push(out, 13413);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13414);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34939);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 34940);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63018);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13415);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34942);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34943);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63020);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89880);
									match s {
										[] => true,
										['п', '-', s @ ..] => {
											super::push(out, 110400);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123883);
													match s {
														[] => true,
														['о', s @ ..] => super::t37::n133392(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13416);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34944);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63021);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 63022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 34945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13417);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 34946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лун.
pub(crate) fn n2847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2837);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13418);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34947);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 34948);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 34949);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 34950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 34951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13419);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 13420);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34953);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63026);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89881);
									s.is_empty()
								}
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
			super::push(out, 13421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 34954);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мун.
pub(crate) fn n2848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2838);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 13422);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34955);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63027);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89882);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 63028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13423);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34956);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нун.
pub(crate) fn n2849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2839);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13424);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34957);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оун.
pub(crate) fn n2850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2840);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13425);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34958);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 63030);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89883);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110401);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 123884);
													match s {
														[] => true,
														['р', s @ ..] => super::t37::n133393(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13426);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 34959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13427);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 34960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пун.
pub(crate) fn n2851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2841);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13428);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34961);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 63031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 34962);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 34963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13429);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34964);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13430);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 34965);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 34966);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63033);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 63034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34967);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 34968);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63035);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 63036);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 89885);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110402);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123885);
													match s {
														[] => true,
														['б', s @ ..] => super::t37::n133394(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13431);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 34969);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 34970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13432);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34971);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13433);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 34972);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рун.
pub(crate) fn n2852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13434);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 34973);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34974);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63040);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 89886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 34976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13435);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 13436);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 34977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13437);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34978);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 34979);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 34980);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63043);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89887);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110403);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 34981);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 34982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13438);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 34983);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63044);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89888);
									s.is_empty()
								}
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
			super::push(out, 13439);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 34984);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 63046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13440);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13441);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34985);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 34986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63047);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сун.
pub(crate) fn n2853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2843);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13442);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 13443);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 13444);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 13445);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 34987);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89890);
									s.is_empty()
								}
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
			super::push(out, 13446);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13447);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 13448);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 34988);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89891);
									s.is_empty()
								}
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
			super::push(out, 13449);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13450);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тун.
pub(crate) fn n2854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2844);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13451);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13452);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 34991);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63052);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89892);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110404);
											s.is_empty()
										}
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
					super::push(out, 34992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13453);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 34993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 34994);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 34995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 34996);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 13454);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 13455);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 34997);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 63058);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 63059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13456);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 34998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 34999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35000);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 35001);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63062);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89893);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110405);
											s.is_empty()
										}
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
					super::push(out, 35002);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63063);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 89894);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110406);
											s.is_empty()
										}
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
					super::push(out, 35003);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63064);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 89895);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 110407);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123886);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n133395(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13457);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35004);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63065);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 63066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35005);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13458);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35006);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 63068);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35007);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 35008);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 35009);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 63070);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 89897);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 63071);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 89898);
									s.is_empty()
								}
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
			super::push(out, 13460);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 13461);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35010);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фун.
pub(crate) fn n2855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2845);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 13462);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13463);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35012);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 35013);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63074);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 89900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хун.
pub(crate) fn n2856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2846);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35014);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 35015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13465);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35016);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 35017);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13466);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35018);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 35019);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13467);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35020);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 63078);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89901);
									s.is_empty()
								}
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
			super::push(out, 13468);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35021);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цун.
pub(crate) fn n2857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2847);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 13469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чун.
pub(crate) fn n2858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2848);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13470);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35022);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63080);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 89902);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 63081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13471);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35023);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шун.
pub(crate) fn n2859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2849);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13472);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 35024);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13473);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35025);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13474);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 35026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щун.
pub(crate) fn n2860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2850);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 13475);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35027);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етранзишн.
pub(crate) fn n125171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123887);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 132036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ул-позишн.
pub(crate) fn n125172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123888);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 132037);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -экстракшн.
pub(crate) fn n125173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123889);
	s.is_empty()
}

/// Words ending -сексторшн.
pub(crate) fn n125174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123890);
	s.is_empty()
}

/// Words ending -льян.
pub(crate) fn n13625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13552);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35095);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 35096);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 35097);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 63121);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мьян.
pub(crate) fn n13626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13553);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 63123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35098);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 63124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ньян.
pub(crate) fn n13627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13554);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 35099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рьян.
pub(crate) fn n13628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13555);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35100);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 63125);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 35101);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 63126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35102);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 63128);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 89924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сьян.
pub(crate) fn n13629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13556);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35103);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63129);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тьян.
pub(crate) fn n13630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13557);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 35104);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 63130);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 89925);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 110420);
									s.is_empty()
								}
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
			super::push(out, 35105);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35106);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 63132);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фьян.
pub(crate) fn n13631(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35107);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 63133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35108);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 63134);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -де-бодайбо.
pub(crate) fn n133398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132038);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 136852);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139742);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 141442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аево.
pub(crate) fn n13678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13604);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 35149);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63159);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 89941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 35150);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63160);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89942);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 110427);
									s.is_empty()
								}
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
			super::push(out, 35151);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 35152);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63161);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дево.
pub(crate) fn n13679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13605);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 35153);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63162);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 89944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еево.
pub(crate) fn n13680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13606);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 35154);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 63163);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 89945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 35155);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63164);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 35156);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 63165);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жево.
pub(crate) fn n13681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13607);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 35157);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63166);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89948);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зево.
pub(crate) fn n13682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13608);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 35158);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63167);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 89949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иево.
pub(crate) fn n13683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13609);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 35159);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63168);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89950);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 110428);
									s.is_empty()
								}
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
			super::push(out, 35160);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63169);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 89951);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лево.
pub(crate) fn n13684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35161);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 63170);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 63171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 35162);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 63172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35163);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63173);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 89952);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110430);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123894);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 132039);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n138266(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35164);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63174);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 89953);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нево.
pub(crate) fn n13685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13611);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35165);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 63175);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 89954);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110432);
									s.is_empty()
								}
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
			super::push(out, 35166);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 63176);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рево.
pub(crate) fn n13686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13612);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35167);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 63177);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 63178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 35168);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 63179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 35169);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 63180);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 89955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 63181);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 89956);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 110433);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 89957);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 110434);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 123895);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 89958);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110435);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 89959);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 110436);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123897);
											s.is_empty()
										}
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
					super::push(out, 63182);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 63183);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 89960);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110437);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123898);
											s.is_empty()
										}
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
					super::push(out, 63184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35170);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 63185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 35171);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 63186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35172);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 63187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 35173);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 35174);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 63188);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89961);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110438);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сево.
pub(crate) fn n13687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13613);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35175);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 63189);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тево.
pub(crate) fn n13688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13614);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35176);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 63190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 35177);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 63191);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уево.
pub(crate) fn n13689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13615);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 35178);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 63192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89963);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 110439);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123900);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 132040);
													match s {
														[] => true,
														['е', s @ ..] => super::t37::n138267(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цево.
pub(crate) fn n13690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13616);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 35179);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63193);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 89964);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110440);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123901);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35180);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63194);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 89965);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 110441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 63195);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89966);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110442);
									s.is_empty()
								}
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
			super::push(out, 35181);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63196);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 89967);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 110443);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123902);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чево.
pub(crate) fn n13691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13617);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 35182);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шево.
pub(crate) fn n13692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13618);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35183);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 63197);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89968);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110444);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123903);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35184);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63198);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 89969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 63199);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89970);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 110445);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123904);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35185);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63200);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 89971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьево.
pub(crate) fn n13693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13619);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 35186);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 63201);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89972);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 110446);
									s.is_empty()
								}
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
			super::push(out, 35187);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 63202);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 89973);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 110447);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 123905);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 132041);
													match s {
														[] => true,
														['ш', s @ ..] => super::t37::n138268(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 63203);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 89974);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 110448);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123906);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 132042);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n138269(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иво.
pub(crate) fn n2932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2922);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13620);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35188);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63204);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 13621);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35189);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63205);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89975);
									s.is_empty()
								}
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
			super::push(out, 13622);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35190);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63206);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 89976);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 35191);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63207);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89977);
									s.is_empty()
								}
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
			super::push(out, 13623);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', 'о', s @ ..] => {
					super::push(out, 35193);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 13624);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 35194);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63210);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 89978);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110449);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 123907);
													match s {
														[] => true,
														['е', s @ ..] => super::t37::n133403(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35195);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63211);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 89979);
									s.is_empty()
								}
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
			super::push(out, 13625);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35196);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 63212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13626);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35197);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13627);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 35198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 13628);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35199);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йво.
pub(crate) fn n2933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2923);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13629);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кво.
pub(crate) fn n2934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2924);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 13630);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35201);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 63216);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 89980);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110450);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123908);
													match s {
														[] => true,
														['с', s @ ..] => super::t37::n133404(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13631);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35202);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63217);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 89981);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110451);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123909);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ово.
pub(crate) fn n2935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2925);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13632);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35203);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63218);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 89982);
									s.is_empty()
								}
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
			super::push(out, 13633);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35204);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63219);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89983);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110452);
											s.is_empty()
										}
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
					super::push(out, 35205);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63220);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89984);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110453);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 123910);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35206);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13634);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35207);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63222);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89985);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110454);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123911);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35208);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63223);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 89986);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110455);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13635);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35209);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63224);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89987);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110456);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13636);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35210);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63225);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 89988);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110457);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 123912);
													s.is_empty()
												}
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
							super::push(out, 63226);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 89989);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110458);
											s.is_empty()
										}
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
					super::push(out, 35211);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63227);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89990);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110459);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13637);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35212);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63228);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 89991);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110460);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 63229);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 89992);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110461);
											s.is_empty()
										}
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
					super::push(out, 35213);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63230);
							match s {
								[] => true,
								['ь', s @ ..] => super::t37::n90953(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t37::n63892(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t37::n35555(s, out),
				['с', s @ ..] => super::t37::n35556(s, out),
				['у', s @ ..] => super::t37::n35557(s, out),
				['ш', s @ ..] => super::t37::n35558(s, out),
				['ь', s @ ..] => super::t37::n35559(s, out),
				['я', s @ ..] => super::t37::n35560(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t37::n13712(s, out),
		['н', s @ ..] => super::t37::n13713(s, out),
		['п', s @ ..] => super::t37::n13714(s, out),
		['р', s @ ..] => super::t37::n13715(s, out),
		['с', s @ ..] => super::t37::n13716(s, out),
		['т', s @ ..] => super::t37::n13717(s, out),
		['х', s @ ..] => super::t37::n13718(s, out),
		_ => false,
	}
}

/// Words ending -рво.
pub(crate) fn n2936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2926);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 13645);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35255);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тво.
pub(crate) fn n2937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2927);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 13646);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35256);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 63296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 35257);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t37::n13721(s, out),
		_ => false,
	}
}

/// Words ending -уво.
pub(crate) fn n2938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2928);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 13648);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35278);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13649);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 35279);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63371);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьво.
pub(crate) fn n2939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2929);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13650);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35280);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 63372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ньчжоу-го.
pub(crate) fn n125479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124194);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132244);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иго.
pub(crate) fn n2944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2934);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13666);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 13667);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35301);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63391);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90244);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 90245);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 63392);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 90246);
									s.is_empty()
								}
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
			super::push(out, 13668);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13669);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35303);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 35304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13670);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35305);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13671);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35306);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нго.
pub(crate) fn n2946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13673);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35308);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 35309);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63396);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90247);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 110800);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 124195);
													match s {
														[] => true,
														['э', s @ ..] => super::t37::n133607(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13674);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35311);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 35312);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13675);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35313);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 35314);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 35315);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 35316);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63398);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90248);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110801);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 63399);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90249);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 110802);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124196);
													match s {
														[] => true,
														['т', s @ ..] => super::t37::n133608(s, out),
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
									super::push(out, 90250);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110803);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124197);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35317);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13676);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35319);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63402);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 63403);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35320);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 63404);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90253);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 110804);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124198);
													s.is_empty()
												}
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
							super::push(out, 63405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ого.
pub(crate) fn n2947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2937);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13677);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13678);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35321);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63406);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90254);
									s.is_empty()
								}
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
			super::push(out, 13679);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рго.
pub(crate) fn n2948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2938);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13680);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35322);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 35323);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 35324);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63407);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 63408);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90255);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110805);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 124199);
													match s {
														[] => true,
														['с', s @ ..] => super::t37::n133609(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13681);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35326);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уго.
pub(crate) fn n2949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2939);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 13682);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35327);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13683);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35328);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63410);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90256);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110806);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьго.
pub(crate) fn n2950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2940);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13684);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35329);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63411);
							match s {
								[] => true,
								['и', 'г', s @ ..] => {
									super::push(out, 90257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юго.
pub(crate) fn n2952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2942);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 13685);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 35330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яго.
pub(crate) fn n2953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2943);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 13686);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35331);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63412);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -карбонадо.
pub(crate) fn n125485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124200);
	s.is_empty()
}

/// Words ending -ртесонадо.
pub(crate) fn n125486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124201);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132248);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -делантадо.
pub(crate) fn n125487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124202);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132249);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инфантадо.
pub(crate) fn n125488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124203);
	s.is_empty()
}

/// Words ending -егдо.
pub(crate) fn n13769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13695);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 35344);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63426);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -едо.
pub(crate) fn n2957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2947);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13696);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 35345);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63427);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90271);
									s.is_empty()
								}
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
			super::push(out, 13697);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35346);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 63428);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90272);
									s.is_empty()
								}
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
			super::push(out, 13698);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35347);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63429);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90273);
									s.is_empty()
								}
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
			super::push(out, 13699);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13700);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 35349);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63430);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90274);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110815);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13701);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35350);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -здо.
pub(crate) fn n2958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2948);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 13702);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35351);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идо.
pub(crate) fn n2959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2949);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13703);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35352);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13704);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 13705);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35354);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63434);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90275);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110816);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13706);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 35355);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 35356);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63436);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 90276);
									s.is_empty()
								}
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
			super::push(out, 13707);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13708);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35357);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63437);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90277);
									s.is_empty()
								}
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
			super::push(out, 13709);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35358);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 13710);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35359);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 63439);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90278);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 110817);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йдо.
pub(crate) fn n2960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2950);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13711);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35360);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63440);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90279);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдо.
pub(crate) fn n2961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2951);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 13712);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35361);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндо.
pub(crate) fn n2962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2952);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13713);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63441);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 63442);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90280);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110819);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124204);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35363);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63443);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90281);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110820);
											s.is_empty()
										}
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
					super::push(out, 35364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63444);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 63445);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 35365);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63446);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90283);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110821);
											s.is_empty()
										}
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
					super::push(out, 35366);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63447);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90284);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110822);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 124205);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35367);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63448);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90285);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110823);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 124206);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13714);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 35368);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63449);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90286);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110824);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13715);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35369);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63450);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 90287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 63451);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90288);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110825);
											s.is_empty()
										}
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
					super::push(out, 35370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 13716);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одо.
pub(crate) fn n2963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2953);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13717);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 13718);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35375);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63455);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90289);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110826);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 124207);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13719);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35376);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 63456);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90290);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110827);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124208);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35377);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдо.
pub(crate) fn n2964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2954);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13720);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35379);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63459);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90291);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35380);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63460);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90292);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110828);
											s.is_empty()
										}
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
					super::push(out, 35381);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63461);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 90293);
									s.is_empty()
								}
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
			super::push(out, 13721);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35382);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 35383);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13722);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35384);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удо.
pub(crate) fn n2965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2955);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13723);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35385);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 35386);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 13724);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35387);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 63464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13725);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 13726);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 13727);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35389);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдо.
pub(crate) fn n2966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2956);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13728);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35390);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 35391);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 35392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдо.
pub(crate) fn n2968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2958);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 13729);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35393);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63465);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 90294);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 110829);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13730);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35394);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13731);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35395);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядо.
pub(crate) fn n2969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2959);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13732);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35396);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63467);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеллефтео.
pub(crate) fn n125494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124209);
	s.is_empty()
}

/// Words ending -арио.
pub(crate) fn n13854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13780);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 35443);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 35444);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 63501);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90316);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110835);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 124210);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 132250);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n138420(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 35445);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 63502);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 90317);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 110836);
									s.is_empty()
								}
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
			super::push(out, 35446);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 63503);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 35447);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -к-трио.
pub(crate) fn n64166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63504);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 90319);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110837);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 124211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сио.
pub(crate) fn n2995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2985);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13783);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35449);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13784);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тио.
pub(crate) fn n2996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2986);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13785);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35451);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цио.
pub(crate) fn n2997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2987);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13786);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35452);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чио.
pub(crate) fn n2998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2988);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 13787);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35453);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63506);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90320);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ико.
pub(crate) fn n3008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2998);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13811);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35477);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13812);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 35478);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63525);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90326);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110838);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124212);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13813);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35479);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13814);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35480);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 35481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13815);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35482);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 63527);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 90327);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110839);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124213);
													match s {
														[] => true,
														['б', s @ ..] => super::t37::n133613(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13816);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35483);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63528);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 90328);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35484);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63529);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 90329);
									s.is_empty()
								}
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
			super::push(out, 13817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13818);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35486);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63530);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90330);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110840);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 90331);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110841);
											s.is_empty()
										}
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
					super::push(out, 35487);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 63531);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13819);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35489);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63532);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 35490);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63533);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90334);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110842);
											s.is_empty()
										}
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
					super::push(out, 35491);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63534);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 90335);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 110843);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 124214);
													match s {
														[] => true,
														['ь', s @ ..] => super::t37::n133614(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35492);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 63535);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 90336);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110844);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124215);
													match s {
														[] => true,
														['в', s @ ..] => super::t37::n133615(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13820);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35493);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63536);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90337);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110845);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13821);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35494);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 13822);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 35495);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63538);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90338);
									s.is_empty()
								}
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
			super::push(out, 13823);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35496);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63539);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 35497);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 35498);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -соловейко.
pub(crate) fn n125501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124216);
	s.is_empty()
}

/// Words ending -патейко.
pub(crate) fn n91309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90341);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 110847);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 124217);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 132254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эйко.
pub(crate) fn n13899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13825);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 35501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 63543);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яйко.
pub(crate) fn n13900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13826);
	s.is_empty()
}

/// Words ending -кко.
pub(crate) fn n3010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3000);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13827);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63544);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13828);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35503);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 63545);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90342);
									s.is_empty()
								}
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
			super::push(out, 13829);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13830);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35505);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 35506);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63546);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90343);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 90344);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 63547);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90345);
									s.is_empty()
								}
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
			super::push(out, 13831);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35507);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 35508);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лко.
pub(crate) fn n3011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3001);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13832);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35509);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 35510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13833);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35511);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63551);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90347);
									s.is_empty()
								}
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
			super::push(out, 13834);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35512);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63552);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нко.
pub(crate) fn n3012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3002);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13835);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35513);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 35514);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 63553);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 90349);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110848);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124218);
													match s {
														[] => true,
														['р', s @ ..] => super::t37::n133617(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13836);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35515);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 63554);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 35516);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63555);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90351);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110849);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124219);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35517);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63556);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90352);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 110850);
											s.is_empty()
										}
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
					super::push(out, 35518);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 63557);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90353);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110851);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 90354);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 110852);
											s.is_empty()
										}
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
					super::push(out, 35519);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63558);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 63559);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90356);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110853);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124220);
													match s {
														[] => true,
														['о', s @ ..] => super::t37::n133618(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13837);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35520);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63560);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90357);
									s.is_empty()
								}
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
			super::push(out, 13838);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35521);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63561);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 35522);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63562);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 90359);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 110854);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 90360);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110855);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13839);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -око.
pub(crate) fn n3013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3003);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13840);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 13841);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35523);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 63563);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90361);
									s.is_empty()
								}
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
			super::push(out, 13842);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35524);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 35525);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13843);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35526);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 63566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35527);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35528);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63568);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90362);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110856);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 124221);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13844);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35529);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63569);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90363);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110857);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 63570);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90364);
									s.is_empty()
								}
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
			super::push(out, 13845);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35530);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 35531);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13846);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35532);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63573);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 63574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 13847);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 13848);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 35533);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63575);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рко.
pub(crate) fn n3014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3004);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13849);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35534);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 35535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13850);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35536);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63576);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 35537);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63577);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 35538);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 35539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 13851);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35540);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ско.
pub(crate) fn n3015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3005);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13852);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35541);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63581);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13853);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35543);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63582);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90370);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110858);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124222);
													match s {
														[] => true,
														['п', s @ ..] => super::t37::n133619(s, out),
														_ => false,
													}
												}
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
							super::push(out, 63583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13854);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35544);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 63584);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90371);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 110859);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 63585);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90372);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110860);
											s.is_empty()
										}
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
					super::push(out, 35545);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63586);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 90373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 35546);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63587);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90374);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110861);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 124223);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35547);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63588);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90375);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110862);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 124224);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13855);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35548);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 63589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13856);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13857);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тко.
pub(crate) fn n3016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3006);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13858);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13859);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35552);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63591);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90376);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 90377);
									s.is_empty()
								}
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
			super::push(out, 13860);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35553);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63592);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90378);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уко.
pub(crate) fn n3017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3007);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13861);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35554);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63593);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90379);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110864);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13862);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 13863);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 13864);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35555);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 13865);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13866);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 35557);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63595);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90380);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110865);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13867);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 13868);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 13869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цко.
pub(crate) fn n3018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3008);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13870);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35558);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 13871);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35559);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63596);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90381);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110866);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чко.
pub(crate) fn n3019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3009);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13872);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35560);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63597);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90382);
									s.is_empty()
								}
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
			super::push(out, 13873);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35561);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63598);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90383);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110867);
											s.is_empty()
										}
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
					super::push(out, 35562);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63599);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90384);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110868);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 63600);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90385);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110869);
											s.is_empty()
										}
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
					super::push(out, 35563);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 35564);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63601);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 63602);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90387);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110870);
											s.is_empty()
										}
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
					super::push(out, 35565);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 35566);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 35567);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63604);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 90388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35568);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63605);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 63606);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90390);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110871);
											s.is_empty()
										}
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
					super::push(out, 35569);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63607);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 63608);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 35570);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63609);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90392);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110872);
											s.is_empty()
										}
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
					super::push(out, 35571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13874);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 35572);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 63610);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 35573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13875);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35574);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63611);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 90394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 63612);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 90395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35575);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63613);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90396);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110873);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 124225);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13876);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13877);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35577);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шко.
pub(crate) fn n3020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3010);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13878);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 35579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13879);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35580);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63617);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90397);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110874);
											s.is_empty()
										}
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
					super::push(out, 35581);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63618);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90398);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t37::n13954(s, out),
		['о', s @ ..] => {
			super::push(out, 13881);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35598);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63679);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 63680);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90477);
									s.is_empty()
								}
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
			super::push(out, 13882);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35599);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63681);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 90478);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 110946);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 90479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 35600);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63682);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90480);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110947);
											s.is_empty()
										}
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
					super::push(out, 35601);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63683);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90481);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110948);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124266);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35602);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63684);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90482);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 35603);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63685);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90483);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110949);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124267);
													match s {
														[] => true,
														['о', s @ ..] => super::t37::n133646(s, out),
														_ => false,
													}
												}
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
							super::push(out, 63686);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 90484);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 110950);
											s.is_empty()
										}
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
					super::push(out, 35604);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63687);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90485);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110951);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 124268);
													match s {
														[] => true,
														['с', s @ ..] => super::t37::n133647(s, out),
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
									super::push(out, 90486);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110952);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124269);
													match s {
														[] => true,
														['с', s @ ..] => super::t37::n133648(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13883);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 35605);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63688);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90487);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110953);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124270);
													match s {
														[] => true,
														['с', s @ ..] => super::t37::n133649(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35606);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63689);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90488);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 110954);
											s.is_empty()
										}
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
					super::push(out, 35607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => super::t37::n13958(s, out),
		_ => false,
	}
}

/// Words ending -ыко.
pub(crate) fn n3021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3011);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ько.
pub(crate) fn n3022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3012);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13886);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35612);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63695);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90494);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 110957);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124271);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t37::n133650(s, out),
														_ => false,
													}
												}
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
							super::push(out, 63696);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 35613);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63697);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90496);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110958);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124272);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13887);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35614);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63698);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 90497);
									s.is_empty()
								}
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
			super::push(out, 13888);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35615);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эко.
pub(crate) fn n3023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3013);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13889);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юко.
pub(crate) fn n3024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3014);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 13890);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35616);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63700);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90498);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110959);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word запевало.
pub(crate) fn n125559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124273);
	s.is_empty()
}

/// Words ending -икало.
pub(crate) fn n35972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35631);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 63712);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 90506);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 110965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еркало.
pub(crate) fn n64375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63713);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 90507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -скало.
pub(crate) fn n35974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35633);
	s.is_empty()
}

/// Words ending -шкало.
pub(crate) fn n35975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35634);
	s.is_empty()
}

/// Words ending -юкало.
pub(crate) fn n35976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35635);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 63714);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 90508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 110966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пало.
pub(crate) fn n13972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13898);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35636);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 63715);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 90509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35637);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 63716);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 90510);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 110967);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124274);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 35638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рало.
pub(crate) fn n13973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13899);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 35639);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 63717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 35640);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63718);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 90511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сало.
pub(crate) fn n13974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13900);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 35641);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 63719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тало.
pub(crate) fn n13975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13901);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 35642);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 63720);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 35643);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 63721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 35644);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 63722);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 90513);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 110968);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 124275);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фало.
pub(crate) fn n13976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13902);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 35645);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 63723);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 90514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 110969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хало.
pub(crate) fn n13977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13903);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 35646);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 63724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90515);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 90516);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 110970);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 124276);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цало.
pub(crate) fn n13978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13904);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 35647);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 63725);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чало.
pub(crate) fn n13979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 35648);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 63726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 35649);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 63727);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шало.
pub(crate) fn n13980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13906);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 35650);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 63728);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 90517);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бло.
pub(crate) fn n3026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3016);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13907);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35651);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 35652);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 35653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63729);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 63730);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90518);
									s.is_empty()
								}
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
			super::push(out, 13908);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35654);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63731);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 63732);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35655);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13909);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13910);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 13911);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35658);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13912);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35659);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вло.
pub(crate) fn n3027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3017);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13913);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гло.
pub(crate) fn n3028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3018);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 13914);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дло.
pub(crate) fn n3029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3019);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13915);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35663);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 35664);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63739);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90520);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110971);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13916);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35665);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35666);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63741);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90521);
									s.is_empty()
								}
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
			super::push(out, 13918);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ело.
pub(crate) fn n3030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3020);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13919);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35668);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 63742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13920);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35669);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63743);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 90522);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110972);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 124277);
													s.is_empty()
												}
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
							super::push(out, 63744);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90523);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110973);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 63745);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90524);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110974);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13921);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35670);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63746);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90525);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 110975);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 124278);
													match s {
														[] => true,
														['к', s @ ..] => super::t37::n133651(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13922);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 13923);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35672);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63747);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13924);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35673);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 63748);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90526);
									s.is_empty()
								}
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
			super::push(out, 13925);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 13926);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35675);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63749);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 90527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 13927);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35677);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 35678);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 63750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зло.
pub(crate) fn n3031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3021);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13928);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13929);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13930);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35681);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ило.
pub(crate) fn n3032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3022);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13931);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 13932);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35683);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63753);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 63754);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110976);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 110977);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 124279);
													match s {
														[] => true,
														['е', s @ ..] => super::t37::n133652(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35684);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 13933);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35685);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 35686);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35687);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 35688);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63759);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 63760);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 63761);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90530);
									s.is_empty()
								}
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
			super::push(out, 13934);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 35689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 13935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 13936);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35691);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 35692);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63762);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90531);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110978);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13937);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 13938);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 13939);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35694);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13940);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35695);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 63764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35697);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 63766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 35698);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90532);
									s.is_empty()
								}
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
			super::push(out, 13941);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35699);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35700);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 63768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 35701);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90533);
									s.is_empty()
								}
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
			super::push(out, 13942);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35702);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 35703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13943);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 35704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 13944);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 13945);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35706);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 35707);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63771);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 90534);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110979);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13946);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35708);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 35709);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63772);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90535);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110980);
											s.is_empty()
										}
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
					super::push(out, 35710);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йло.
pub(crate) fn n3033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3023);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13947);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35712);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63775);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90536);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 110981);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 124280);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35713);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13948);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35714);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63777);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90537);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110982);
											s.is_empty()
										}
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
					super::push(out, 35715);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 35716);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13949);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 35717);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кло.
pub(crate) fn n3034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3024);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13950);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13951);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35720);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13952);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 13953);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13954);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35722);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лло.
pub(crate) fn n3035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3025);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13955);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 35723);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13956);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35724);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35725);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63786);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90538);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 110983);
											s.is_empty()
										}
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
					super::push(out, 35726);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 35727);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63787);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90539);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110984);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 124281);
													match s {
														[] => true,
														['л', s @ ..] => super::t37::n133653(s, out),
														_ => false,
													}
												}
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
							super::push(out, 63788);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90540);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 110985);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13957);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 35728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13958);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 35729);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 63789);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90541);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110986);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мло.
pub(crate) fn n3036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3026);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13959);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 35730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63790);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нло.
pub(crate) fn n3037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3027);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 13960);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оло.
pub(crate) fn n3038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3028);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 13961);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35732);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63792);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90542);
									s.is_empty()
								}
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
			super::push(out, 13962);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35733);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 63793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 13963);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35734);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 63794);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 90543);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 110987);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 124282);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13964);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35735);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 63795);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90544);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 110988);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 124283);
													match s {
														[] => true,
														['т', s @ ..] => super::t37::n133654(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 35736);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 63796);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90545);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110989);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13965);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35737);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 35738);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63798);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 90546);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110990);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13966);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35739);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 63799);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90547);
									s.is_empty()
								}
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
			super::push(out, 13967);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35740);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63800);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90548);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110991);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 124284);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35741);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 63801);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90549);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 110992);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13968);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35742);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63802);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90550);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 110993);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13969);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 13970);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35743);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63803);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90551);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 110994);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пло.
pub(crate) fn n3039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3029);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 13971);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35744);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13972);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 35746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13973);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рло.
pub(crate) fn n3040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3030);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13974);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 35748);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 63805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 35749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13975);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35750);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 35751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13976);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13977);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 35753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13978);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13979);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35755);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63809);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 90553);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 110995);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 124285);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 35756);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сло.
pub(crate) fn n3041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3031);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13980);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35757);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 35758);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 35759);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 63811);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 90554);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 110996);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 124286);
													match s {
														[] => true,
														['в', s @ ..] => super::t37::n133655(s, out),
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
									super::push(out, 90555);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 110997);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 124287);
													s.is_empty()
												}
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
							super::push(out, 63812);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 90556);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 110998);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13981);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35760);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 35761);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63813);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 63814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35762);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63815);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 90558);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 110999);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 63816);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 90559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 35763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13982);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 35764);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13983);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13984);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35766);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63818);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 90560);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 111000);
											s.is_empty()
										}
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
					super::push(out, 35767);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 35768);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 63819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13985);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35769);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 63820);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90561);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 111001);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13986);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 35770);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 63821);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 90562);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 111002);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 124288);
													s.is_empty()
												}
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
							super::push(out, 63822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 35771);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тло.
pub(crate) fn n3042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3032);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 13987);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 35772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13988);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уло.
pub(crate) fn n3043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3033);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 13989);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 13990);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 13991);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35774);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фло.
pub(crate) fn n3044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3034);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 13992);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 35775);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63825);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 90563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 35776);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хло.
pub(crate) fn n3045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3035);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13993);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 35777);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 35778);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 63826);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 90564);
									s.is_empty()
								}
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
			super::push(out, 13994);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 35779);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13995);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 35780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 13996);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35781);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шло.
pub(crate) fn n3046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3036);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 13997);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыло.
pub(crate) fn n3047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3037);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13998);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 13999);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35783);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 63829);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 90565);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 111003);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 14000);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 63830);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 63831);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 90566);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 111004);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 124289);
													match s {
														[] => true,
														['а', s @ ..] => super::t37::n133656(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яло.
pub(crate) fn n3048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3038);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 14001);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 35785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 14002);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 35786);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уантанамо.
pub(crate) fn n125576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124290);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 132295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чентезимо.
pub(crate) fn n125577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124291);
	s.is_empty()
}

/// Words ending -ьермо.
pub(crate) fn n36147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35806);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 63845);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 90576);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 111010);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -смо.
pub(crate) fn n3057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3047);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 14020);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 35808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 14021);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 35809);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -умо.
pub(crate) fn n3058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3048);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 14022);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чмо.
pub(crate) fn n3059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3049);
	s.is_empty()
}

/// Words ending -ьмо.
pub(crate) fn n3060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3050);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 14023);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35810);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 63846);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 90577);
									s.is_empty()
								}
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
			super::push(out, 14024);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 35811);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 63847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 14025);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35812);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 14026);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 35813);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 63849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

