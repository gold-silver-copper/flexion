use flexion::Path;

/// Words ending -пка.
pub(crate) fn n741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 740);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5691);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 21748);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 21749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 47630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 21750);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 47631);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 76247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 21751);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 21752);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 47632);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 76248);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100483);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5692);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 21753);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 47633);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 76249);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 100484);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117793);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21754);
					match s {
						[] => true,
						['к', 'ы', s @ ..] => {
							super::push(out, 47634);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 76250);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100485);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 47635);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 76251);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100486);
											s.is_empty()
										}
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
					super::push(out, 21755);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 47636);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 47637);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 76252);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 100487);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 117794);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t30::n130063(s, out),
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
									super::push(out, 76253);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 100488);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117795);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21756);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 21757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5693);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 21758);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 47638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 21759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5694);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 21760);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 47639);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 76254);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 100489);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 117796);
													match s {
														[] => true,
														['т', s @ ..] => super::t30::n130064(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 21761);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 47640);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 76255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 21762);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 47641);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 76256);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100490);
											s.is_empty()
										}
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
					super::push(out, 21763);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 47642);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 76257);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100491);
											s.is_empty()
										}
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
					super::push(out, 21764);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 47643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 21765);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 21766);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 47644);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 76258);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 100492);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 117797);
													s.is_empty()
												}
												['л', s @ ..] => {
													super::push(out, 117798);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21767);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 21768);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 47645);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 47646);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 76259);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100493);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t30::n5709(s, out),
		['у', s @ ..] => super::t30::n5710(s, out),
		['ы', s @ ..] => super::t30::n5711(s, out),
		['я', s @ ..] => super::t30::n5712(s, out),
		_ => false,
	}
}

/// Words ending -рка.
pub(crate) fn n742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 741);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5699);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21778);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 76268);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 100499);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 117801);
													match s {
														[] => true,
														['з', s @ ..] => super::t30::n130066(s, out),
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
									super::push(out, 76269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 47658);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 76270);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100500);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t30::n21899(s, out),
				['г', s @ ..] => super::t30::n21900(s, out),
				['д', s @ ..] => {
					super::push(out, 21781);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 47673);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 76288);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t30::n48151(s, out),
						['у', 'с', s @ ..] => super::t30::n48152(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => super::t30::n21902(s, out),
				['з', s @ ..] => super::t30::n21903(s, out),
				['к', s @ ..] => super::t30::n21904(s, out),
				['м', s @ ..] => super::t30::n21905(s, out),
				['н', s @ ..] => super::t30::n21906(s, out),
				['п', s @ ..] => super::t30::n21907(s, out),
				['р', s @ ..] => super::t30::n21908(s, out),
				['с', s @ ..] => super::t30::n21909(s, out),
				['т', s @ ..] => super::t30::n21910(s, out),
				['у', s @ ..] => super::t30::n21911(s, out),
				['х', s @ ..] => super::t30::n21912(s, out),
				['ц', s @ ..] => super::t30::n21913(s, out),
				['ч', s @ ..] => super::t30::n21914(s, out),
				['ш', s @ ..] => super::t30::n21915(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t30::n5714(s, out),
		['и', s @ ..] => super::t30::n5715(s, out),
		['о', s @ ..] => super::t30::n5716(s, out),
		['р', s @ ..] => super::t30::n5717(s, out),
		['у', s @ ..] => super::t30::n5718(s, out),
		['ы', s @ ..] => super::t30::n5719(s, out),
		['ю', s @ ..] => super::t30::n5720(s, out),
		['я', s @ ..] => super::t30::n5721(s, out),
		_ => false,
	}
}

/// Words ending -ска.
pub(crate) fn n743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 742);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 21875);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 21876);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 21877);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 47900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 21878);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 47901);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 76523);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 100681);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117894);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 21879);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 47902);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 76524);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100682);
											s.is_empty()
										}
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
					super::push(out, 21880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 47903);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 47904);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 76525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 47905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 21881);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 21882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 47906);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 47907);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 47908);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 76526);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 100683);
											s.is_empty()
										}
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
					super::push(out, 21883);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 47909);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 76527);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											super::push(out, 100684);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 47910);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 76528);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 76529);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 100685);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117895);
													s.is_empty()
												}
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
							super::push(out, 47911);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 76530);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 100686);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 117896);
													match s {
														[] => true,
														['г', s @ ..] => super::t30::n130109(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 21884);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 47912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 21885);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 47913);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 76531);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 100687);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117897);
													s.is_empty()
												}
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
							super::push(out, 47914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5709);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21886);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 47915);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76532);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 47916);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 76533);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 100688);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 117898);
													s.is_empty()
												}
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
							super::push(out, 47917);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 76534);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 100689);
											match s {
												[] => true,
												['р', s @ ..] => super::t30::n119161(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t30::n22007(s, out),
				['к', s @ ..] => super::t30::n22008(s, out),
				['л', s @ ..] => super::t30::n22009(s, out),
				['м', s @ ..] => super::t30::n22010(s, out),
				['р', s @ ..] => super::t30::n22011(s, out),
				['т', s @ ..] => super::t30::n22012(s, out),
				['ч', s @ ..] => super::t30::n22013(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t30::n5724(s, out),
		['й', s @ ..] => super::t30::n5725(s, out),
		['м', s @ ..] => super::t30::n5726(s, out),
		['о', s @ ..] => super::t30::n5727(s, out),
		['п', s @ ..] => super::t30::n5728(s, out),
		['с', s @ ..] => super::t30::n5729(s, out),
		['у', s @ ..] => super::t30::n5730(s, out),
		['ы', s @ ..] => super::t30::n5731(s, out),
		['э', s @ ..] => super::t30::n5732(s, out),
		['я', s @ ..] => super::t30::n5733(s, out),
		_ => false,
	}
}

/// Words ending -тка.
pub(crate) fn n744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 743);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5720);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 21932);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 47997);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 47998);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76598);
									match s {
										[] => true,
										['к', s @ ..] => super::t30::n101819(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t30::n48486(s, out),
						['у', s @ ..] => super::t30::n48487(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t30::n22054(s, out),
				['г', s @ ..] => super::t30::n22055(s, out),
				['д', s @ ..] => super::t30::n22056(s, out),
				['е', s @ ..] => super::t30::n22057(s, out),
				['ж', 'е', s @ ..] => super::t30::n22058(s, out),
				['и', s @ ..] => super::t30::n22059(s, out),
				['к', s @ ..] => super::t30::n22060(s, out),
				['л', s @ ..] => super::t30::n22061(s, out),
				['м', s @ ..] => super::t30::n22062(s, out),
				['н', s @ ..] => super::t30::n22063(s, out),
				['о', s @ ..] => super::t30::n22064(s, out),
				['п', s @ ..] => super::t30::n22065(s, out),
				['р', s @ ..] => super::t30::n22066(s, out),
				['с', s @ ..] => super::t30::n22067(s, out),
				['т', s @ ..] => super::t30::n22068(s, out),
				['х', s @ ..] => super::t30::n22069(s, out),
				['ц', s @ ..] => super::t30::n22070(s, out),
				['ч', s @ ..] => super::t30::n22071(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t30::n5735(s, out),
		['е', s @ ..] => super::t30::n5736(s, out),
		['и', s @ ..] => super::t30::n5737(s, out),
		['й', s @ ..] => super::t30::n5738(s, out),
		['л', s @ ..] => super::t30::n5739(s, out),
		['н', s @ ..] => super::t30::n5740(s, out),
		['о', s @ ..] => super::t30::n5741(s, out),
		['р', s @ ..] => super::t30::n5742(s, out),
		['с', s @ ..] => super::t30::n5743(s, out),
		['т', s @ ..] => super::t30::n5744(s, out),
		['у', s @ ..] => super::t30::n5745(s, out),
		['ш', s @ ..] => super::t30::n5746(s, out),
		['ы', s @ ..] => super::t30::n5747(s, out),
		['ь', s @ ..] => super::t30::n5748(s, out),
		['э', s @ ..] => super::t30::n5749(s, out),
		['ю', s @ ..] => super::t30::n5750(s, out),
		['я', s @ ..] => super::t30::n5751(s, out),
		_ => false,
	}
}

/// Words ending -ука.
pub(crate) fn n745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 744);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5737);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22043);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 48258);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 48259);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 76849);
									s.is_empty()
								}
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
			super::push(out, 5738);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22044);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 22045);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48260);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 76850);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22046);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48261);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 76851);
									s.is_empty()
								}
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
			super::push(out, 5739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22047);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 48262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5740);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22048);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 48263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22050);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22051);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 48264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 22052);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48265);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 48266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22053);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48267);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 76853);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 100918);
											s.is_empty()
										}
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
					super::push(out, 22054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 48268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5742);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22055);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22056);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 48269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 76854);
									s.is_empty()
								}
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
			super::push(out, 5743);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22057);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 48270);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 76855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 48271);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 48272);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76856);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 100919);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22059);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 22060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5745);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22061);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22062);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 48273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22063);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 48274);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5746);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 48275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5747);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 22066);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48276);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 76857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22067);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48277);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 76858);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 22068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 5748);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 5749);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22069);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 5750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22070);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22071);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 48279);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 76859);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 100920);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 48280);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 76860);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 100921);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фка.
pub(crate) fn n746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 745);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5751);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22072);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 76861);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 100922);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 118043);
													match s {
														[] => true,
														['т', s @ ..] => super::t30::n130172(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 100923);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5752);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5753);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22074);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 48281);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 76862);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 76863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хка.
pub(crate) fn n747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 746);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5754);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22076);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 48282);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 76864);
									s.is_empty()
								}
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
			super::push(out, 5755);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22077);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 48283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цка.
pub(crate) fn n748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 747);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5756);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 22078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5757);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22079);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 48284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5758);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22080);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 48285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чка.
pub(crate) fn n749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 748);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5759);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 48286);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 76865);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 100924);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 76866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 48287);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 76867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 48288);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 48289);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 76868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 48290);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22082);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 48291);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 76870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 48292);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 76871);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 100925);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 48293);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 76872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 48294);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 76873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22083);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48295);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 76874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 76875);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 100926);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118044);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 76876);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 100927);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 48296);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 76877);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 48297);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 76878);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 76879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t30::n22214(s, out),
				['з', s @ ..] => super::t30::n22215(s, out),
				['к', s @ ..] => super::t30::n22216(s, out),
				['л', s @ ..] => super::t30::n22217(s, out),
				['м', s @ ..] => super::t30::n22218(s, out),
				['н', s @ ..] => super::t30::n22219(s, out),
				['п', s @ ..] => super::t30::n22220(s, out),
				['р', s @ ..] => super::t30::n22221(s, out),
				['с', s @ ..] => super::t30::n22222(s, out),
				['т', s @ ..] => super::t30::n22223(s, out),
				['х', s @ ..] => super::t30::n22224(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t30::n5775(s, out),
		['и', s @ ..] => super::t31::n5776(s, out),
		['л', s @ ..] => super::t31::n5777(s, out),
		['о', s @ ..] => super::t31::n5778(s, out),
		['у', s @ ..] => super::t31::n5779(s, out),
		['ы', s @ ..] => super::t31::n5780(s, out),
		['э', s @ ..] => super::t31::n5781(s, out),
		['ю', s @ ..] => super::t31::n5782(s, out),
		['я', s @ ..] => super::t31::n5783(s, out),
		_ => false,
	}
}

/// Words ending -шка.
pub(crate) fn n750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 749);
	match s {
		[] => true,
		['а', s @ ..] => super::t31::n5784(s, out),
		['е', s @ ..] => {
			super::push(out, 5770);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22204);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22205);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48724);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 77465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 48725);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 77466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22206);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48726);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 77467);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 101391);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118342);
													s.is_empty()
												}
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
							super::push(out, 48727);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 77468);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 101392);
											s.is_empty()
										}
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
					super::push(out, 22207);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48728);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 77469);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 101393);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 118343);
													match s {
														[] => true,
														['а', s @ ..] => super::t31::n130337(s, out),
														_ => false,
													}
												}
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
							super::push(out, 48729);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 48730);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 77470);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 101394);
											s.is_empty()
										}
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
					super::push(out, 22208);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 48731);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 48732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 22209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 48733);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 48734);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 77471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 48735);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 77472);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 101395);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 48736);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 48737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22210);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 77473);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 77474);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 101396);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 77475);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 101397);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 118344);
													match s {
														[] => true,
														['у', s @ ..] => super::t31::n130338(s, out),
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
									super::push(out, 77476);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22211);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22212);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 48738);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 77477);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 101398);
											s.is_empty()
										}
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
					super::push(out, 22213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 48739);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 48740);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 48741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22214);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 48742);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 48743);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 77478);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 101399);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118345);
													s.is_empty()
												}
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
							super::push(out, 48744);
							match s {
								[] => true,
								['д', s @ ..] => super::t31::n78304(s, out),
								_ => false,
							}
						}
						['ю', s @ ..] => super::t31::n49250(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t31::n22347(s, out),
				['ф', s @ ..] => super::t31::n22348(s, out),
				['ц', s @ ..] => super::t31::n22349(s, out),
				['ч', s @ ..] => super::t31::n22350(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t31::n5786(s, out),
		['о', s @ ..] => super::t31::n5787(s, out),
		['у', s @ ..] => super::t31::n5788(s, out),
		['ы', s @ ..] => super::t31::n5789(s, out),
		['э', s @ ..] => super::t31::n5790(s, out),
		['ю', s @ ..] => super::t31::n5791(s, out),
		['я', s @ ..] => super::t31::n5792(s, out),
		_ => false,
	}
}

/// Words ending -ыка.
pub(crate) fn n751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 750);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5778);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22309);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22310);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22311);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49073);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 77931);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 101783);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 118583);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5780);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22312);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49074);
							match s {
								[] => true,
								['-', 'п', s @ ..] => {
									super::push(out, 77932);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 101784);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 118584);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5781);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22313);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 49075);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 77933);
									s.is_empty()
								}
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
			super::push(out, 5782);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22314);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49076);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 77934);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 101785);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5783);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22315);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 49077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22316);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 49078);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 77935);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 101786);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 49079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 5784);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22317);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 49080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ька.
pub(crate) fn n752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 751);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5785);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22318);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 49081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22319);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49082);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 49083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22320);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49084);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 77936);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 101787);
											s.is_empty()
										}
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
					super::push(out, 22321);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 49085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5786);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22322);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5787);
			match s {
				[] => true,
				['а', s @ ..] => super::t31::n22455(s, out),
				['е', s @ ..] => super::t31::n22456(s, out),
				['и', s @ ..] => super::t31::n22457(s, out),
				['о', s @ ..] => super::t31::n22458(s, out),
				['у', s @ ..] => super::t31::n22459(s, out),
				['ю', s @ ..] => super::t31::n22460(s, out),
				['я', s @ ..] => {
					super::push(out, 22329);
					match s {
						[] => true,
						['л', s @ ..] => super::t31::n49634(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t31::n5803(s, out),
		['р', s @ ..] => super::t31::n5804(s, out),
		['с', s @ ..] => super::t31::n5805(s, out),
		['т', s @ ..] => super::t31::n5806(s, out),
		_ => false,
	}
}

/// Words ending -эка.
pub(crate) fn n753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 752);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 5792);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22350);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юка.
pub(crate) fn n754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 753);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5793);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22351);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49191);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 78079);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 101923);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 118654);
													match s {
														[] => true,
														['ы', s @ ..] => super::t31::n130489(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5794);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 22352);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 49192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5795);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22353);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 22354);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49193);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 78080);
									s.is_empty()
								}
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
			super::push(out, 5796);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22355);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22356);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49195);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 78081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22357);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49196);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 49197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 22358);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 22360);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 49200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5797);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22361);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49201);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78083);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 101924);
											s.is_empty()
										}
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
					super::push(out, 22362);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49202);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 78084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22363);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49203);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78085);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 101925);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5798);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22364);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49204);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 78086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22365);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49205);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78087);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 101926);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яка.
pub(crate) fn n755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 754);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5799);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22366);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49206);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78088);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 101927);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', 'о', 'р', 'к', '\u{1}', s @ ..] => {
			super::push(out, 5800);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5801);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22367);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49207);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78089);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 101928);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5802);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22368);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49208);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 78090);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 101929);
											s.is_empty()
										}
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
					super::push(out, 22369);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22370);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49210);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78091);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 101930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 49211);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5803);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22371);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49212);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 78092);
									s.is_empty()
								}
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
			super::push(out, 5804);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22372);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49213);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 49214);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 78093);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 101931);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 118655);
													match s {
														[] => true,
														['я', s @ ..] => super::t31::n130490(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22373);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49215);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78094);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 101932);
											s.is_empty()
										}
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
					super::push(out, 22374);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5805);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22375);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 49217);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78095);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 101933);
											s.is_empty()
										}
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
					super::push(out, 22376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5806);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22377);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 49218);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 78096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22378);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49219);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 49220);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22379);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5807);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22380);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 22381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5808);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22382);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49222);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 78098);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 101934);
											s.is_empty()
										}
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
					super::push(out, 22383);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49223);
							match s {
								[] => true,
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 78099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22384);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5809);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22385);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5810);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22386);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49226);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 78100);
									s.is_empty()
								}
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
			super::push(out, 5811);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22387);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бала.
pub(crate) fn n5828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5813);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22388);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 49228);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 49229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78101);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 78102);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 101935);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118656);
											s.is_empty()
										}
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
					super::push(out, 49230);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 49231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 22389);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49232);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22390);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49233);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 49234);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 78105);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 101936);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118657);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 129139);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22391);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 49235);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 78106);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101937);
									s.is_empty()
								}
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
			super::push(out, 22392);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49236);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78107);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 78108);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вала.
pub(crate) fn n5829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5814);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22393);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49237);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78109);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 49238);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78110);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101940);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118658);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 129140);
													s.is_empty()
												}
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
							super::push(out, 78111);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 101941);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 118659);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 101942);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 49239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22394);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 49240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22395);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 49241);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78112);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 49242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 22397);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 22398);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49245);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 49246);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 78113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 22399);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49247);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 49248);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78114);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -агала.
pub(crate) fn n22533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22401);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 49249);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 49250);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78116);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -егала.
pub(crate) fn n22534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22402);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 49251);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78117);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -игала.
pub(crate) fn n22535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22403);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 49252);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 78118);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нгала.
pub(crate) fn n22536(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22404);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 49253);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 78119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огала.
pub(crate) fn n22537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22405);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 49254);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78120);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дала.
pub(crate) fn n5831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5816);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22406);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49255);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 49256);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22407);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22408);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49258);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еала.
pub(crate) fn n5832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5817);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 22409);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49259);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 78124);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жала.
pub(crate) fn n5833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5818);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22410);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49260);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78125);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зала.
pub(crate) fn n5834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5819);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 22411);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49261);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78126);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101949);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иала.
pub(crate) fn n5835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5820);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 22412);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 22413);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кала.
pub(crate) fn n5836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5821);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22414);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49263);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78127);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101950);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 118660);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22415);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49264);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78128);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 101951);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 118661);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 129141);
													match s {
														[] => true,
														['п', s @ ..] => super::t31::n136751(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 22416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49265);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 49266);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78129);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 101952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 49267);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 78130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 22417);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49268);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 78131);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 101953);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118662);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22418);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мала.
pub(crate) fn n5837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5822);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22419);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22420);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49270);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78133);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 101954);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 118663);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22421);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 49271);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78134);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101955);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118664);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22422);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 49272);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22423);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49273);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 49274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нала.
pub(crate) fn n5838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5823);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22424);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49275);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78138);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оала.
pub(crate) fn n5839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5824);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 22425);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пала.
pub(crate) fn n5840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5825);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 22426);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 49276);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78139);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101957);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118665);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 78140);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 101958);
									s.is_empty()
								}
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
			super::push(out, 22427);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22428);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 49278);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78141);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 22429);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рала.
pub(crate) fn n5841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5826);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22430);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 22431);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49279);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сала.
pub(crate) fn n5842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5827);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22433);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 22434);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 49281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22435);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49282);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78144);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101959);
									s.is_empty()
								}
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
			super::push(out, 22436);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тала.
pub(crate) fn n5843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5828);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22437);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22438);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49285);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 78146);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101960);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 118666);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22439);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49286);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 78147);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 101961);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уала.
pub(crate) fn n5844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5829);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 22440);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цала.
pub(crate) fn n5846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5831);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 22441);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49287);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78148);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 101962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шала.
pub(crate) fn n5847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5832);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22442);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 22443);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49288);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бла.
pub(crate) fn n757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 756);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5833);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22444);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 22445);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5834);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22447);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5835);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 22449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 5836);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22450);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 49292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вла.
pub(crate) fn n758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 757);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5837);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22451);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гла.
pub(crate) fn n759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 758);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5838);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22452);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5839);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22453);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22454);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49294);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78149);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 101963);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5840);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22455);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 49295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дла.
pub(crate) fn n760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 759);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5841);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5842);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22457);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5843);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5844);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 22459);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ела.
pub(crate) fn n761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 760);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5845);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22460);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49298);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 49299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78150);
									s.is_empty()
								}
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
			super::push(out, 5846);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22461);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 49300);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78151);
									s.is_empty()
								}
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
			super::push(out, 5847);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22462);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 22463);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49301);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78152);
									s.is_empty()
								}
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
			super::push(out, 5848);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 22465);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49303);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 78153);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 101964);
											s.is_empty()
										}
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
					super::push(out, 22466);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49304);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22467);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49305);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 78155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22468);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49306);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 78156);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 101965);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 118667);
													match s {
														[] => true,
														['п', s @ ..] => super::t31::n130494(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5849);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22469);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49307);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 78157);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 101966);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 118668);
													s.is_empty()
												}
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
							super::push(out, 49308);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78158);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22470);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5850);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22471);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49310);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78159);
									s.is_empty()
								}
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
			super::push(out, 5851);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5852);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22473);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 49311);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5853);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22475);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22476);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49313);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 78161);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 101967);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 118669);
													match s {
														[] => true,
														['р', s @ ..] => super::t31::n130495(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5854);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 5855);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22478);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 49314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22479);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 49315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 5856);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 22480);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5857);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22482);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 49318);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зла.
pub(crate) fn n762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 761);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 5858);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 22483);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49319);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78163);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 101968);
											s.is_empty()
										}
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
					super::push(out, 22484);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ила.
pub(crate) fn n763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 762);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5859);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 22485);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49320);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78164);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 22486);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49321);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 78165);
									s.is_empty()
								}
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
			super::push(out, 5860);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 22487);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49322);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 78166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22488);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49323);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 49324);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 78167);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 101969);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118670);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5861);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22489);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49325);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 78168);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 101970);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 101971);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 118671);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22490);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49326);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78169);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 101972);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 118672);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5862);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22491);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5863);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22492);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49328);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49329);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 49330);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 49331);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 49332);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 49333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5864);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5865);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22495);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22496);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22497);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49336);
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
				['р', s @ ..] => {
					super::push(out, 22498);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49337);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78171);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 101973);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5867);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22499);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22500);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22501);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5868);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22502);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 49340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22503);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5869);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22504);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 49342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22505);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49343);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 49344);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 78173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22506);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49345);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 78174);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 101974);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 49346);
							match s {
								[] => true,
								['б', s @ ..] => super::t31::n79002(s, out),
								['г', s @ ..] => {
									super::push(out, 78176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t31::n22639(s, out),
				['т', s @ ..] => super::t31::n22640(s, out),
				['у', s @ ..] => super::t31::n22641(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t31::n5885(s, out),
		['п', s @ ..] => super::t31::n5886(s, out),
		['р', s @ ..] => super::t31::n5887(s, out),
		['с', s @ ..] => super::t31::n5888(s, out),
		['т', s @ ..] => super::t31::n5889(s, out),
		['у', s @ ..] => super::t31::n5890(s, out),
		['ф', s @ ..] => super::t31::n5891(s, out),
		['х', s @ ..] => super::t31::n5892(s, out),
		['ч', s @ ..] => super::t31::n5893(s, out),
		['ш', s @ ..] => super::t31::n5894(s, out),
		_ => false,
	}
}

/// Words ending -йла.
pub(crate) fn n764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 763);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5880);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22552);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 22553);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49389);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 78208);
									s.is_empty()
								}
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
			super::push(out, 5881);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22554);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 22555);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5882);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22556);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кла.
pub(crate) fn n765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 764);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5883);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22557);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 22558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5884);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22559);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 49393);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 78209);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 101989);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 118679);
													match s {
														[] => true,
														['с', s @ ..] => super::t31::n130497(s, out),
														_ => false,
													}
												}
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
							super::push(out, 49394);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 78210);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 101990);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 118680);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лла.
pub(crate) fn n766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 765);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22560);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22561);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 49395);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 78211);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 101991);
											s.is_empty()
										}
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
					super::push(out, 22562);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49396);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 78212);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 101992);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 118681);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49397);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 49398);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78213);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 101993);
											s.is_empty()
										}
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
					super::push(out, 22564);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49399);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78214);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 101994);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 118682);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22565);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 49400);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 78215);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 101995);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 118683);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5886);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49401);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', 'и', '\u{1}', s @ ..] => {
									super::push(out, 78216);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 78217);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 78218);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 101996);
											s.is_empty()
										}
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
					super::push(out, 22567);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 22568);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49403);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 78219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22569);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							super::push(out, 49405);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 49406);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22570);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49407);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78221);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 101997);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 118684);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22571);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49408);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 49409);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 78222);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 101998);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 118685);
													match s {
														[] => true,
														['с', s @ ..] => super::t31::n130498(s, out),
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
									super::push(out, 78223);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 78224);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 101999);
											match s {
												[] => true,
												['р', s @ ..] => super::t31::n119948(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t31::n49920(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t31::n22705(s, out),
				['р', s @ ..] => super::t31::n22706(s, out),
				['т', s @ ..] => super::t31::n22707(s, out),
				['ц', s @ ..] => super::t31::n22708(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t31::n5902(s, out),
		['о', s @ ..] => super::t31::n5903(s, out),
		['у', s @ ..] => super::t31::n5904(s, out),
		['э', s @ ..] => super::t31::n5905(s, out),
		_ => false,
	}
}

/// Words ending -ола.
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 766);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5891);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22593);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49435);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78248);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102012);
											s.is_empty()
										}
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
					super::push(out, 22594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5892);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22595);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49437);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78249);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 49438);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78250);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102013);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 49439);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 78251);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 102014);
											s.is_empty()
										}
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
					super::push(out, 22596);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 49440);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22597);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 49441);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 78253);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102015);
											s.is_empty()
										}
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
					super::push(out, 22598);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49442);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 78254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22599);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49443);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78255);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102016);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 118691);
													s.is_empty()
												}
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
							super::push(out, 49444);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 78256);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102017);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 118692);
													match s {
														[] => true,
														['к', s @ ..] => super::t31::n130502(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22600);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49445);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 78257);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102018);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 118693);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5893);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22602);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 22603);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49446);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 78258);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102019);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5894);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22604);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5895);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5896);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22606);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 22607);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49448);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 78259);
									s.is_empty()
								}
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
			super::push(out, 5897);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22608);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22609);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 49449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22610);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 49450);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78260);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 102020);
											s.is_empty()
										}
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
					super::push(out, 22611);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5898);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22612);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22613);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5899);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 22614);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22615);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49453);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => super::t31::n49964(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t31::n22750(s, out),
				['у', s @ ..] => super::t31::n22751(s, out),
				['ш', s @ ..] => super::t31::n22752(s, out),
				['ы', s @ ..] => super::t31::n22753(s, out),
				['ю', s @ ..] => super::t31::n22754(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t31::n5915(s, out),
		['м', s @ ..] => super::t31::n5916(s, out),
		['н', s @ ..] => super::t31::n5917(s, out),
		['п', s @ ..] => super::t31::n5918(s, out),
		['р', s @ ..] => super::t31::n5919(s, out),
		['т', s @ ..] => super::t31::n5920(s, out),
		['ь', s @ ..] => super::t31::n5921(s, out),
		_ => false,
	}
}

/// Words ending -пла.
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 767);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5907);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22634);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рла.
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 768);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5908);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5909);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 22636);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49472);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 78275);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 102027);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 118696);
													match s {
														[] => true,
														['щ', s @ ..] => super::t31::n130504(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5910);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22637);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 49473);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78276);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 102028);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5911);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5912);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22639);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 49474);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 78277);
									s.is_empty()
								}
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
			super::push(out, 5913);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22640);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сла.
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 769);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5914);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 22641);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5915);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22642);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 49476);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5916);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22644);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тла.
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 770);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22645);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 22646);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49477);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78279);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 102029);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5918);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 22647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5919);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22648);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ула.
pub(crate) fn n772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 771);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5920);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22649);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22650);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 22651);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49479);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 78280);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 78281);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 102030);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 118697);
													match s {
														[] => true,
														['с', s @ ..] => super::t31::n130505(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22652);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49480);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78282);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102031);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5921);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22653);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22654);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 49481);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5922);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22656);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49482);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 78284);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102032);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118698);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5923);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49483);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 49484);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 49485);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 78285);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102033);
											s.is_empty()
										}
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
					super::push(out, 22658);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49486);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22659);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22660);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49488);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22661);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49489);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22662);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22663);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49490);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78288);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 102034);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118699);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5924);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22664);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 22665);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 22666);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 49492);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 78289);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102035);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5925);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5926);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5927);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5928);
			match s {
				[] => true,
				['е', 'ф', s @ ..] => {
					super::push(out, 22669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5929);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22670);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22671);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 49495);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78290);
									s.is_empty()
								}
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
			super::push(out, 5930);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22672);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49496);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 78291);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102036);
											s.is_empty()
										}
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
					super::push(out, 22673);
					match s {
						[] => true,
						['а', s @ ..] => super::t31::n50007(s, out),
						['и', s @ ..] => {
							super::push(out, 49498);
							match s {
								[] => true,
								['ф', s @ ..] => super::t31::n79120(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => super::t31::n5946(s, out),
		['х', s @ ..] => super::t31::n5947(s, out),
		_ => false,
	}
}

/// Words ending -ыла.
pub(crate) fn n773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5933);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22677);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5934);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22678);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эла.
pub(crate) fn n774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 773);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5935);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22679);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49503);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 78295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22680);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49504);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78296);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 102038);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5936);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22681);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22682);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юла.
pub(crate) fn n775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 774);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5937);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5938);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22683);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5939);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22684);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 49506);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 78297);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 102039);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 118702);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22685);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 49507);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78298);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 102040);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5940);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22686);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яла.
pub(crate) fn n776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 775);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5941);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5942);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22688);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22689);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49509);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 78299);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102041);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 118703);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 78300);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102042);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5943);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5944);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22690);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 49510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 5945);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 22691);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49511);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78301);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 102043);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5946);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22692);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49512);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 78302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -алай-лама.
pub(crate) fn n119966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118704);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 129155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нчен-лама.
pub(crate) fn n119967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118705);
	s.is_empty()
}

/// Words ending -тармалама.
pub(crate) fn n119968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118706);
	s.is_empty()
}

/// Words ending -термалама.
pub(crate) fn n119969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118707);
	s.is_empty()
}

/// Words ending -эпиталама.
pub(crate) fn n119970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118708);
	s.is_empty()
}

/// Words ending -реклама.
pub(crate) fn n79137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78310);
	match s {
		[] => true,
		['о', 'м', s @ ..] => {
			super::push(out, 102049);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 118709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слама.
pub(crate) fn n22840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22706);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 49526);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 78311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мама.
pub(crate) fn n5969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5954);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22708);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нама.
pub(crate) fn n5970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5955);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22709);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22710);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 49527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 22711);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 49528);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78312);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рама.
pub(crate) fn n5971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5956);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22713);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 22714);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 49530);
					match s {
						[] => true,
						['а', 'п', '-', s @ ..] => {
							super::push(out, 78313);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 102052);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 118710);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 129156);
													match s {
														[] => true,
														['о', s @ ..] => super::t31::n136756(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сама.
pub(crate) fn n5972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5957);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 22715);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49531);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вма.
pub(crate) fn n778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 777);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5958);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гма.
pub(crate) fn n779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 778);
	s.is_empty()
}

/// Words ending -ема.
pub(crate) fn n780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 779);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 5959);
			match s {
				[] => true,
				['о', 'б', s @ ..] => {
					super::push(out, 22716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5960);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5961);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22717);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22718);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 49532);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78314);
									s.is_empty()
								}
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
			super::push(out, 5962);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22719);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22720);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49533);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 49534);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 49535);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78316);
									s.is_empty()
								}
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
			super::push(out, 5963);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22721);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22722);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49537);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 78318);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102053);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 118711);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n130509(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5964);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22723);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 49538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22724);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22725);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22726);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49539);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78320);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 49540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22728);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22729);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49543);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 49544);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 78321);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102054);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 118712);
													s.is_empty()
												}
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
							super::push(out, 49545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22730);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5965);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5966);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 5967);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22733);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 49547);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78322);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102055);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 49548);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 78323);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 5968);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жма.
pub(crate) fn n781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 780);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5969);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зма.
pub(crate) fn n782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 781);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5971);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5972);
			match s {
				[] => true,
				['р', 'а', s @ ..] => {
					super::push(out, 22736);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 49550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -има.
pub(crate) fn n783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 782);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5973);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22737);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22738);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49551);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 78324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22739);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49552);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78325);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 78326);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 102056);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118713);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5974);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22740);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22741);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49553);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22742);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49554);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78328);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 102057);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 118714);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5975);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5976);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5977);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22744);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22745);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49555);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 49556);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 49557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 22746);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49558);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22747);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22748);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 49560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5978);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22749);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49561);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 78331);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102058);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5979);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22750);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5980);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5981);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22753);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22755);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22756);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49565);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 78332);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102059);
											s.is_empty()
										}
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
					super::push(out, 22757);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 49566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 49567);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 78334);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 102060);
											s.is_empty()
										}
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
					super::push(out, 22758);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 49568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5983);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22759);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49569);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 49570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 22760);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 49571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22761);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22762);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49573);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 78337);
									s.is_empty()
								}
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
			super::push(out, 5984);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22763);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49574);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 78338);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102061);
											s.is_empty()
										}
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
					super::push(out, 22764);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 5985);
			match s {
				[] => true,
				['н', s @ ..] => super::t31::n22899(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 5986);
			match s {
				[] => true,
				['е', s @ ..] => super::t31::n22900(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йма.
pub(crate) fn n784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 783);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5987);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22768);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49579);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78340);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102063);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5988);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5989);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кма.
pub(crate) fn n785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 784);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 5990);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 22769);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 49580);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 78341);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102064);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лма.
pub(crate) fn n786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 785);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5991);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 22770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 22771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5992);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22772);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мма.
pub(crate) fn n787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 786);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5993);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22773);
					s.is_empty()
				}
				['р', 'г', 'о', 'х', s @ ..] => {
					super::push(out, 22774);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49582);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78342);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102065);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118716);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5994);
			match s {
				[] => true,
				['л', 'а', s @ ..] => {
					super::push(out, 22775);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 49583);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78343);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 102066);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118717);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5995);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22776);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49584);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78344);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102067);
											s.is_empty()
										}
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
					super::push(out, 22777);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 5996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22778);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нма.
pub(crate) fn n788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 787);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 5997);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 22779);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ома.
pub(crate) fn n789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 788);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5998);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22780);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22781);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 49588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5999);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 6000);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22782);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49589);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 78345);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 102068);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 118718);
													match s {
														[] => true,
														['р', s @ ..] => super::t31::n130510(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22783);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 22784);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49590);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 78346);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102069);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 118719);
													match s {
														[] => true,
														['д', s @ ..] => super::t31::n130511(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6001);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22785);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 22786);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49591);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 78347);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102070);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6002);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22787);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22788);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49592);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78348);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 102071);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118720);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22789);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49593);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 78349);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102072);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 118721);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6003);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22790);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49594);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 78350);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102073);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 118722);
													match s {
														[] => true,
														['п', s @ ..] => super::t31::n130512(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6004);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22791);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 22792);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22793);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22794);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 22795);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49596);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78351);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 78352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22796);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49597);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78353);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102074);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118723);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 118724);
													match s {
														[] => true,
														['а', s @ ..] => super::t31::n130513(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22797);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49598);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 22798);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49599);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 78355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 49600);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t31::n6020(s, out),
		['т', s @ ..] => super::t31::n6021(s, out),
		['ф', s @ ..] => super::t31::n6022(s, out),
		['ц', s @ ..] => super::t31::n6023(s, out),
		_ => false,
	}
}

/// Words ending -рма.
pub(crate) fn n790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 789);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6009);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22809);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 22810);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22811);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22812);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 49614);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78366);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102084);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118731);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22813);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 49615);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 78367);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 102085);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 118732);
													match s {
														[] => true,
														['о', s @ ..] => super::t31::n130517(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6010);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22814);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49616);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 78368);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102086);
											s.is_empty()
										}
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
					super::push(out, 22815);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22816);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 22817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22818);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49618);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78369);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 102087);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 118733);
													match s {
														[] => true,
														['а', s @ ..] => super::t31::n130518(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22819);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 22820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6011);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 22821);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49619);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78370);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 102088);
											match s {
												[] => true,
												['а', '-', s @ ..] => {
													super::push(out, 118734);
													match s {
														[] => true,
														['з', s @ ..] => super::t31::n130519(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6012);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 22823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6013);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22824);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 49621);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22825);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49622);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78372);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102089);
											s.is_empty()
										}
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
					super::push(out, 22826);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49623);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 78373);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102090);
											s.is_empty()
										}
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
					super::push(out, 22827);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 22828);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49625);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 49626);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78374);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102091);
											s.is_empty()
										}
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
					super::push(out, 22829);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 22830);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сма.
pub(crate) fn n791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 790);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6014);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22831);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 22832);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 22833);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22834);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49628);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 78375);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102092);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 118735);
													match s {
														[] => true,
														['а', s @ ..] => super::t31::n130520(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6015);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тма.
pub(crate) fn n792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 791);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6016);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22836);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49629);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 78376);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102093);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6017);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22837);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49630);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78377);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102094);
											s.is_empty()
										}
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
					super::push(out, 22838);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 49631);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 78378);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 102095);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6018);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22839);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ума.
pub(crate) fn n793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 792);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6019);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6020);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22841);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6021);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 22842);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 49633);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78379);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 22843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22844);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 22845);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 49634);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78380);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102096);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6023);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 6024);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22846);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 22847);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 49635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6025);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6026);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 6027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22849);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фма.
pub(crate) fn n794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 793);
	s.is_empty()
}

/// Words ending -хма.
pub(crate) fn n795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 794);
	s.is_empty()
}

/// Words ending -чма.
pub(crate) fn n796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 795);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 6028);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22850);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49636);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78381);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шма.
pub(crate) fn n797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 796);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6029);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 22851);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 49637);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78382);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102097);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6030);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6031);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 22853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6032);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьма.
pub(crate) fn n798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 797);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 6033);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22854);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6034);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22855);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49640);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 22856);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6035);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49642);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 49643);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 49644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22858);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49645);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 49646);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 49647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22859);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 22860);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49648);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78385);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102098);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 118736);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6036);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22861);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49649);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78386);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102099);
											s.is_empty()
										}
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
					super::push(out, 22862);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49650);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 22863);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6037);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22864);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49652);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6038);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22865);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 49653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22866);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49654);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78388);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102100);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эма.
pub(crate) fn n799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 798);
	s.is_empty()
}

/// Words ending -яма.
pub(crate) fn n800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 799);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 6039);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6040);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22867);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 49655);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 78389);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кебана.
pub(crate) fn n50166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49656);
	s.is_empty()
}

/// Words ending -ибана.
pub(crate) fn n23004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22870);
	s.is_empty()
}

/// Words ending -эбана.
pub(crate) fn n23005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22871);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 49657);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78390);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вана.
pub(crate) fn n6058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6043);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22872);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 49658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22873);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22874);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49660);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78391);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 78392);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гана.
pub(crate) fn n6059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6044);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22875);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49661);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78393);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 78394);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 102103);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 118737);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22876);
			match s {
				[] => true,
				['ъ', s @ ..] => {
					super::push(out, 49662);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 78395);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102104);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 118738);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22877);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49663);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 78396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22878);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49664);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78397);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102105);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 118739);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 102106);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 118740);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22879);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49665);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 78398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дана.
pub(crate) fn n6060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6045);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22880);
			s.is_empty()
		}
		['а', s @ ..] => {
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 49666);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 49667);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 22881);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49668);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22882);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49669);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 78402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жана.
pub(crate) fn n6061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6046);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22884);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 49670);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зана.
pub(crate) fn n6062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6047);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22885);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49671);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иана.
pub(crate) fn n6063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6048);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 22886);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 22887);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49673);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 49674);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78405);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 78406);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 102108);
									s.is_empty()
								}
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
			super::push(out, 22888);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49675);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 49676);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 49677);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22889);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49678);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78409);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 102109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 78410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 22890);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 49680);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78412);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 49681);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 78413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 49682);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 78414);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102111);
									s.is_empty()
								}
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
			super::push(out, 22891);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49683);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 78415);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102112);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 118741);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йана.
pub(crate) fn n6064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6049);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22892);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 49684);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 49685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кана.
pub(crate) fn n6065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6050);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22893);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22894);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49686);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 78417);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 102113);
									s.is_empty()
								}
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
			super::push(out, 22895);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 49687);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78418);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102114);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 118742);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 22896);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 22897);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22898);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 22899);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49690);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лана.
pub(crate) fn n6066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6051);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22900);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22901);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22902);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 22903);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49693);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 78420);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 102115);
									s.is_empty()
								}
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
			super::push(out, 22904);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 49694);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78421);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 102116);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мана.
pub(crate) fn n6067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6052);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22905);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 49695);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оана.
pub(crate) fn n6068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6053);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 22906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пана.
pub(crate) fn n6069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6054);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22907);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 49696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 22908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 49697);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рана.
pub(crate) fn n6070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6055);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 22909);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 49698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22910);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 22911);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 49700);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 78424);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 102117);
									s.is_empty()
								}
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
			super::push(out, 22912);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49701);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сана.
pub(crate) fn n6071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6056);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22913);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22914);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 49702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78425);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 78426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22915);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22916);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22917);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49705);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тана.
pub(crate) fn n6072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6057);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22918);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 49706);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 49707);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 49708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22919);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 49709);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 78430);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 102118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 49710);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 78431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 22920);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 49711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 22921);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 49712);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 22922);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 22923);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49714);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78434);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 22924);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 49715);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 78435);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 49716);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уана.
pub(crate) fn n6073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6058);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 22925);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 22926);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 22927);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49718);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 78436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фана.
pub(crate) fn n6074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6059);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 22928);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49719);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 78437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хана.
pub(crate) fn n6075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6060);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 22929);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 49720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 22930);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49721);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 78438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 49722);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 78439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 22931);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 49723);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 78440);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 102120);
									s.is_empty()
								}
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
			super::push(out, 22932);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 49724);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 78441);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 102121);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 118743);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цана.
pub(crate) fn n6076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6061);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22933);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шана.
pub(crate) fn n6077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6062);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 22934);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 49725);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 78442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бна.
pub(crate) fn n802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 801);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 6063);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22935);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 22936);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вна.
pub(crate) fn n803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 802);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6064);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22937);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49727);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78443);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 102122);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 118744);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6065);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22938);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49728);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 78444);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102123);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 118745);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n130521(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22939);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49729);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78445);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102124);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 118746);
													match s {
														[] => true,
														['г', s @ ..] => super::t31::n130522(s, out),
														_ => false,
													}
												}
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
							super::push(out, 49730);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 78446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 49731);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 78447);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102125);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 118747);
													match s {
														[] => true,
														['д', s @ ..] => super::t31::n130523(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 22940);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49732);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78448);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102126);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 118748);
													s.is_empty()
												}
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
							super::push(out, 49733);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78449);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102127);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 118749);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22941);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 22942);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49734);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 78450);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102128);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 78451);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 49735);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 78452);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102129);
											s.is_empty()
										}
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
					super::push(out, 22943);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49736);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 49737);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 49738);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 78453);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102130);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 118750);
													match s {
														[] => true,
														['р', s @ ..] => super::t31::n130524(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 6066);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22944);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t31::n6082(s, out),
		_ => false,
	}
}

/// Words ending -гна.
pub(crate) fn n804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 803);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6068);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 22958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 22959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6069);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 22960);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дна.
pub(crate) fn n805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 804);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6070);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 22961);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49766);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78485);
									s.is_empty()
								}
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
			super::push(out, 6071);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22962);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6072);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 22963);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ена.
pub(crate) fn n806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 805);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6073);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 22964);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78486);
									s.is_empty()
								}
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
			super::push(out, 6074);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 6075);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6076);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22966);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 22967);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 6077);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6078);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 22968);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49771);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 49772);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 78487);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102159);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 118771);
													s.is_empty()
												}
												['х', s @ ..] => {
													super::push(out, 118772);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22969);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 22970);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 49773);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78488);
									s.is_empty()
								}
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
			super::push(out, 6079);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6080);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 22971);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22972);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 22973);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49774);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 78489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22974);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49775);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 78490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 22975);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 49776);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 49777);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 49778);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 49779);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78493);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 102160);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 118773);
													match s {
														[] => true,
														['п', s @ ..] => super::t31::n130536(s, out),
														_ => false,
													}
												}
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
							super::push(out, 49780);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22976);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49781);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 49782);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78495);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102161);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 49783);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 49784);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 78496);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 102162);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 118774);
													s.is_empty()
												}
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
							super::push(out, 49785);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 78497);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 102163);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 118775);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22977);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49786);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 78498);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22978);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22979);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22980);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 49789);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 78499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 22981);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 49790);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 49791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 22982);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 49792);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 78500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t31::n6096(s, out),
		['п', s @ ..] => {
			super::push(out, 6082);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 22993);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 49803);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78507);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 102169);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t31::n6098(s, out),
		['с', s @ ..] => super::t31::n6099(s, out),
		['т', s @ ..] => super::t31::n6100(s, out),
		['ф', s @ ..] => super::t31::n6101(s, out),
		['ц', s @ ..] => super::t31::n6102(s, out),
		['ч', s @ ..] => {
			super::push(out, 6088);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жна.
pub(crate) fn n807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 806);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 6089);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 23012);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 49827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зна.
pub(crate) fn n808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 807);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6090);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6091);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 23014);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 49828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78522);
									s.is_empty()
								}
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
			super::push(out, 6092);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23015);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 23016);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49829);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 78523);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 78524);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 102177);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 118781);
													s.is_empty()
												}
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
							super::push(out, 49830);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 78525);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 49831);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 78526);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102179);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 118782);
													match s {
														[] => true,
														['о', s @ ..] => super::t31::n130541(s, out),
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
									super::push(out, 78527);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 102180);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 118783);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 78528);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23017);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 23018);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49832);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 78529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23019);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 23020);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 49833);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 78530);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102181);
											s.is_empty()
										}
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
					super::push(out, 23021);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 49834);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 78531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 49835);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 78532);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 102182);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 49836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23022);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49837);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 49838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 23023);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 49839);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 23024);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 49840);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 78534);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 102183);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 118784);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ина.
pub(crate) fn n809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 808);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6093);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 23025);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 49841);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 23026);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 23027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49843);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 49844);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78536);
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									super::push(out, 78537);
									s.is_empty()
								}
								['ю', s @ ..] => {
									super::push(out, 78538);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 102184);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 118785);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t31::n130542(s, out),
														_ => false,
													}
												}
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
							super::push(out, 49845);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 23028);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 49846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6094);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 23029);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 49847);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 78540);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 102185);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 118786);
													match s {
														[] => true,
														['п', s @ ..] => super::t31::n130543(s, out),
														_ => false,
													}
												}
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
							super::push(out, 49848);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 78541);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 49849);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 78542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 49850);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 78543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 49851);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 78544);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 102186);
											s.is_empty()
										}
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
					super::push(out, 23030);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 49852);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 78545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 49853);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 78546);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102187);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 118787);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', s @ ..] => super::t31::n79374(s, out),
								['х', s @ ..] => super::t31::n79375(s, out),
								_ => false,
							}
						}
						['ы', s @ ..] => super::t31::n50365(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => super::t31::n23166(s, out),
				['и', s @ ..] => super::t31::n23167(s, out),
				['л', s @ ..] => {
					super::push(out, 23033);
					s.is_empty()
				}
				['м', s @ ..] => super::t31::n23169(s, out),
				['о', s @ ..] => super::t31::n23170(s, out),
				['р', s @ ..] => super::t31::n23171(s, out),
				['у', s @ ..] => super::t31::n23172(s, out),
				['ы', s @ ..] => super::t31::n23173(s, out),
				['ь', s @ ..] => super::t31::n23174(s, out),
				['я', s @ ..] => super::t31::n23175(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t31::n6110(s, out),
		['г', s @ ..] => super::t31::n6111(s, out),
		['д', s @ ..] => super::t31::n6112(s, out),
		['е', s @ ..] => super::t31::n6113(s, out),
		['ж', s @ ..] => super::t31::n6114(s, out),
		['з', s @ ..] => super::t31::n6115(s, out),
		['и', s @ ..] => super::t31::n6116(s, out),
		['к', s @ ..] => super::t31::n6117(s, out),
		['л', s @ ..] => super::t31::n6118(s, out),
		['м', s @ ..] => super::t31::n6119(s, out),
		['н', s @ ..] => super::t31::n6120(s, out),
		['о', s @ ..] => super::t31::n6121(s, out),
		['п', s @ ..] => super::t31::n6122(s, out),
		['р', s @ ..] => super::t31::n6123(s, out),
		['с', s @ ..] => super::t31::n6124(s, out),
		['т', s @ ..] => super::t31::n6125(s, out),
		['у', s @ ..] => super::t31::n6126(s, out),
		['ф', s @ ..] => super::t31::n6127(s, out),
		['х', s @ ..] => super::t31::n6128(s, out),
		['ц', s @ ..] => super::t31::n6129(s, out),
		['ч', s @ ..] => super::t32::n6130(s, out),
		['ш', s @ ..] => super::t32::n6131(s, out),
		['щ', s @ ..] => super::t32::n6132(s, out),
		_ => false,
	}
}

/// Words ending -йна.
pub(crate) fn n810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 809);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6118);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 23246);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 23247);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6119);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 23249);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 50399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 23250);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 50400);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 79223);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 102719);
											s.is_empty()
										}
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
					super::push(out, 23251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6120);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кна.
pub(crate) fn n811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 810);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6121);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23252);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50401);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 79224);
									s.is_empty()
								}
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
			super::push(out, 6122);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 23253);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50402);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 79225);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 102720);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лна.
pub(crate) fn n812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 811);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6123);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 23254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6124);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 23255);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 50404);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 50405);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 79226);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 102721);
											match s {
												[] => true,
												['а', 'р', '\u{1}', s @ ..] => {
													super::push(out, 119109);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 79227);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 102722);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 50406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мна.
pub(crate) fn n813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 812);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6125);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 23256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 23257);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50408);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 79228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нна.
pub(crate) fn n814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 813);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6126);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23258);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 23259);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50409);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 50410);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79230);
									s.is_empty()
								}
								['р', 'а', 'м', '\u{1}', s @ ..] => {
									super::push(out, 79231);
									s.is_empty()
								}
								['ь', s @ ..] => {
									super::push(out, 79232);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 102723);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119110);
													match s {
														[] => true,
														['м', s @ ..] => super::t32::n130701(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 23260);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50411);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 79233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 23261);
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50412);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 50413);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23262);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 50414);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 50415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 23263);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 23264);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23265);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 23266);
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 50418);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 50419);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 79236);
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
							super::push(out, 50420);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 50421);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 79237);
									s.is_empty()
								}
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
			super::push(out, 6127);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 23267);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 23268);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 50422);
							match s {
								[] => true,
								['а', '-', s @ ..] => {
									super::push(out, 79238);
									match s {
										[] => true,
										['а', 'т', 'ч', s @ ..] => {
											super::push(out, 102724);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 119111);
													match s {
														[] => true,
														['м', s @ ..] => super::t32::n130702(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
					super::push(out, 23269);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 23270);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 23271);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6128);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 50423);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 50424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 23272);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50425);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 79239);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23273);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 50426);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 79240);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 102725);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 119112);
													match s {
														[] => true,
														['б', s @ ..] => super::t32::n130703(s, out),
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
									super::push(out, 79241);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 102726);
											s.is_empty()
										}
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
					super::push(out, 23274);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 50427);
							match s {
								[] => true,
								['в', s @ ..] => super::t32::n80074(s, out),
								_ => false,
							}
						}
						['л', s @ ..] => super::t32::n50943(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t32::n23414(s, out),
				['н', s @ ..] => super::t32::n23415(s, out),
				['т', s @ ..] => super::t32::n23416(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t32::n6145(s, out),
		_ => false,
	}
}

