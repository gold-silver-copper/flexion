use flexion::Path;

/// Words ending -рик.
pub(crate) fn n2283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2277);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10969);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10970);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30804);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30805);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 58683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 30806);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 30807);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 58684);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 86740);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 108467);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 58685);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 86741);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 108468);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 122838);
													s.is_empty()
												}
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
							super::push(out, 58686);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 86742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30808);
					s.is_empty()
				}
				['з', 'и', s @ ..] => {
					super::push(out, 30809);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 58687);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86743);
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
						['с', s @ ..] => {
							super::push(out, 58688);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 58689);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86744);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108469);
											s.is_empty()
										}
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
					super::push(out, 30810);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 58690);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 86745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30811);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 58691);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 58692);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 86746);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 58693);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 86747);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 108470);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 122839);
													match s {
														[] => true,
														['а', s @ ..] => super::t35::n132868(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 30812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 58694);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 58695);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 58696);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 86748);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 108471);
											s.is_empty()
										}
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
					super::push(out, 30813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 58697);
							s.is_empty()
						}
						['у', 'г', '\u{1}', s @ ..] => {
							super::push(out, 58698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30814);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 58699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 30815);
					s.is_empty()
				}
				['х', 'у', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 58700);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 58701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 30816);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 58702);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 86751);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 108472);
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
								['л', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108473);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 108474);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 86752);
									s.is_empty()
								}
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
			super::push(out, 10971);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 58703);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 58704);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86754);
									match s {
										[] => true,
										['п', s @ ..] => super::t35::n109630(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t35::n31043(s, out),
				['м', s @ ..] => super::t35::n31044(s, out),
				['о', s @ ..] => super::t35::n31045(s, out),
				['у', s @ ..] => super::t35::n31046(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t35::n11005(s, out),
		['г', s @ ..] => super::t35::n11006(s, out),
		['д', s @ ..] => super::t35::n11007(s, out),
		['е', s @ ..] => super::t35::n11008(s, out),
		['и', s @ ..] => super::t35::n11009(s, out),
		['к', s @ ..] => super::t35::n11010(s, out),
		['н', s @ ..] => super::t35::n11011(s, out),
		['о', s @ ..] => super::t35::n11012(s, out),
		['р', s @ ..] => super::t35::n11013(s, out),
		['т', s @ ..] => super::t35::n11014(s, out),
		['у', s @ ..] => super::t35::n11015(s, out),
		['ф', s @ ..] => super::t35::n11016(s, out),
		['ы', s @ ..] => super::t35::n11017(s, out),
		['ю', s @ ..] => super::t35::n11018(s, out),
		['я', s @ ..] => super::t35::n11019(s, out),
		_ => false,
	}
}

/// Words ending -сик.
pub(crate) fn n2284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2278);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10987);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 58779);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 86800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 58780);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 86801);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 108504);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 122854);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30885);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 58781);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86802);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 108505);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 122855);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30886);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 58782);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 58783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', '\u{1}', s @ ..] => {
					super::push(out, 30887);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 30888);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 30889);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 30890);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 58784);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 86803);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 108506);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 122856);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10988);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30891);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30892);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 58785);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 86804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10989);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10990);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 58786);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 58787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 58788);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 58789);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 86805);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 108507);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 58790);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86806);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 108508);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 58791);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 86807);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 108509);
											s.is_empty()
										}
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
					super::push(out, 30895);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 58792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10991);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30896);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10992);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30897);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 58793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 58794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30898);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 58795);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 58796);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 86809);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 108510);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 122857);
													match s {
														[] => true,
														['д', s @ ..] => super::t35::n132874(s, out),
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
									super::push(out, 86810);
									s.is_empty()
								}
								['р', s @ ..] => super::t35::n87740(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t35::n31132(s, out),
				['с', 'е', 'у', s @ ..] => super::t35::n31133(s, out),
				['т', s @ ..] => super::t35::n31134(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t35::n11026(s, out),
		['р', s @ ..] => super::t35::n11027(s, out),
		['с', s @ ..] => super::t35::n11028(s, out),
		['у', s @ ..] => super::t35::n11029(s, out),
		['ы', s @ ..] => {
			super::push(out, 10996);
			match s {
				[] => true,
				['м', s @ ..] => super::t35::n31148(s, out),
				_ => false,
			}
		}
		['ь', s @ ..] => super::t35::n11031(s, out),
		['ю', s @ ..] => super::t35::n11032(s, out),
		_ => false,
	}
}

/// Words ending -тик.
pub(crate) fn n2285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2279);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10999);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30917);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 58818);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86824);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 108519);
											s.is_empty()
										}
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
					super::push(out, 30918);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 58819);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 86825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30919);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 58820);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86826);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 108520);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 122862);
													match s {
														[] => true,
														['п', s @ ..] => super::t35::n132875(s, out),
														_ => false,
													}
												}
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
							super::push(out, 58821);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 86827);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 108521);
											s.is_empty()
										}
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
					super::push(out, 30920);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 58822);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 86828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 58823);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 86829);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 58824);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 86830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 30921);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 58825);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 86831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 58826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30922);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 58827);
							match s {
								[] => true,
								['е', 'н', 'п', '\u{1}', s @ ..] => {
									super::push(out, 86832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 58828);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 86833);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 108522);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 122863);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t35::n132876(s, out),
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
									super::push(out, 86834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 58829);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 58830);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 86835);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108523);
											s.is_empty()
										}
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
					super::push(out, 30923);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 86836);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 86837);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108524);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 86838);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 108525);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 122864);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									super::push(out, 86839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 58831);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 58832);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 86840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t35::n31158(s, out),
				['с', s @ ..] => {
					super::push(out, 30924);
					match s {
						[] => true,
						['а', s @ ..] => super::t35::n59458(s, out),
						['о', s @ ..] => super::t35::n59459(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t35::n31160(s, out),
				['ф', s @ ..] => super::t35::n31161(s, out),
				['ч', s @ ..] => super::t35::n31162(s, out),
				['щ', s @ ..] => super::t35::n31163(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t35::n11034(s, out),
		['е', s @ ..] => super::t35::n11035(s, out),
		['и', s @ ..] => super::t35::n11036(s, out),
		['к', s @ ..] => super::t35::n11037(s, out),
		['л', s @ ..] => super::t35::n11038(s, out),
		['м', s @ ..] => super::t35::n11039(s, out),
		['н', s @ ..] => super::t35::n11040(s, out),
		['о', s @ ..] => super::t35::n11041(s, out),
		['п', s @ ..] => super::t35::n11042(s, out),
		['р', s @ ..] => super::t35::n11043(s, out),
		['с', s @ ..] => super::t35::n11044(s, out),
		['т', s @ ..] => super::t35::n11045(s, out),
		['у', s @ ..] => super::t35::n11046(s, out),
		['ф', s @ ..] => super::t35::n11047(s, out),
		['х', s @ ..] => super::t35::n11048(s, out),
		['ш', s @ ..] => super::t35::n11049(s, out),
		['ы', s @ ..] => super::t35::n11050(s, out),
		['ь', 'л', 'у', 'м', s @ ..] => super::t35::n11051(s, out),
		['э', s @ ..] => super::t35::n11052(s, out),
		['ю', s @ ..] => super::t35::n11053(s, out),
		['я', s @ ..] => super::t35::n11054(s, out),
		_ => false,
	}
}

/// Words ending -уик.
pub(crate) fn n2286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2280);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 11021);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 58964);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 86944);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 108609);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 122918);
													match s {
														[] => true,
														['ю', s @ ..] => super::t35::n132893(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фик.
pub(crate) fn n2287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2281);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11022);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 30994);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 58965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 58966);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 86945);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 108610);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 122919);
													match s {
														[] => true,
														['л', s @ ..] => super::t35::n132894(s, out),
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
									super::push(out, 86946);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 58967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11023);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30995);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 58968);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86947);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 108611);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 58969);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 86948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 58970);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 86949);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 108612);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11024);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30996);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 58971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11025);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30997);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 58972);
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
				['р', s @ ..] => {
					super::push(out, 30998);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 58973);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 86950);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 108613);
											s.is_empty()
										}
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
					super::push(out, 30999);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 58974);
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
					super::push(out, 31000);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 58975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31001);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 58976);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 86951);
									s.is_empty()
								}
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
			super::push(out, 11026);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 11027);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31003);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 58977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цик.
pub(crate) fn n2288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2282);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11028);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31004);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 58978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11029);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11030);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 31006);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чик.
pub(crate) fn n2289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2283);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 31007);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 58979);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 86952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 31008);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 58980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31009);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 58981);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 86953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31010);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 58982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 31011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 11031);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 58983);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 86954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 58984);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 86955);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 108614);
											s.is_empty()
										}
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
					super::push(out, 31012);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 58985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 58986);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 58987);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 86956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 58988);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 58989);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86957);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 108615);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 58990);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 58991);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 58992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 31013);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 58993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', 'р', '\u{1}', s @ ..] => {
					super::push(out, 31014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11032);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31015);
					match s {
						[] => true,
						['д', 'у', s @ ..] => {
							super::push(out, 58994);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', 'р', 'е', 'м', '\u{1}', s @ ..] => {
							super::push(out, 58995);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 86959);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108616);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 86960);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 108617);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 58996);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 86961);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 108618);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 122920);
													s.is_empty()
												}
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
							super::push(out, 58997);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 86962);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 58998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 31016);
					match s {
						[] => true,
						['ж', '\u{1}', s @ ..] => {
							super::push(out, 58999);
							s.is_empty()
						}
						['л', 'т', s @ ..] => {
							super::push(out, 59000);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 86963);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 108619);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 122921);
													match s {
														[] => true,
														['с', s @ ..] => super::t35::n132895(s, out),
														_ => false,
													}
												}
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
							super::push(out, 59001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31017);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 59002);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 59003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 31018);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 59005);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 86964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 59006);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 86965);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 108620);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 122922);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t35::n11070(s, out),
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => super::t35::n59640(s, out),
						['п', s @ ..] => super::t35::n59641(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t35::n31270(s, out),
				['р', s @ ..] => super::t35::n31271(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t35::n11072(s, out),
		['з', s @ ..] => super::t35::n11073(s, out),
		['и', s @ ..] => super::t35::n11074(s, out),
		['й', s @ ..] => super::t35::n11075(s, out),
		['м', s @ ..] => super::t35::n11076(s, out),
		['н', s @ ..] => super::t35::n11077(s, out),
		['п', s @ ..] => super::t35::n11078(s, out),
		['р', s @ ..] => super::t35::n11079(s, out),
		['с', s @ ..] => super::t35::n11080(s, out),
		['т', s @ ..] => super::t35::n11081(s, out),
		['у', s @ ..] => super::t35::n11082(s, out),
		['ф', s @ ..] => super::t35::n11083(s, out),
		['ы', s @ ..] => super::t35::n11084(s, out),
		['ь', s @ ..] => super::t35::n11085(s, out),
		['ю', s @ ..] => super::t35::n11086(s, out),
		['я', s @ ..] => super::t35::n11087(s, out),
		_ => false,
	}
}

/// Words ending -шик.
pub(crate) fn n2290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2284);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 11049);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31076);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59147);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87101);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 108749);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123018);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11050);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31077);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 59148);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31078);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 31079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11051);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31080);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 59150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11052);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11053);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31082);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 31083);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59151);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87103);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 108750);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123019);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11054);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31085);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59152);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11055);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31086);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 59153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 11056);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31088);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щик.
pub(crate) fn n2291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2285);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11057);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31089);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 59155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87104);
									s.is_empty()
								}
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
			super::push(out, 11058);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31090);
					match s {
						[] => true,
						['д', 'а', s @ ..] => {
							super::push(out, 59156);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87105);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 108751);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123020);
													s.is_empty()
												}
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
							super::push(out, 59157);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 87106);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 108752);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123021);
													s.is_empty()
												}
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
							super::push(out, 59158);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 87107);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 108753);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123022);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t35::n132959(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31091);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59159);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87108);
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
						['л', s @ ..] => {
							super::push(out, 59160);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87109);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 108754);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123023);
													s.is_empty()
												}
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
							super::push(out, 59161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31092);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 59162);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87110);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 108755);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123024);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t35::n132960(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', 'ы', s @ ..] => {
							super::push(out, 59163);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87111);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108756);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t35::n11098(s, out),
		['г', s @ ..] => {
			super::push(out, 11060);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31100);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59193);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 87170);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 108844);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123101);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11061);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11062);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11063);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 11064);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31104);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31105);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59195);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87171);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 108845);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 123102);
													match s {
														[] => true,
														['т', s @ ..] => super::t35::n133008(s, out),
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
									super::push(out, 87172);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 108846);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123103);
													s.is_empty()
												}
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
							super::push(out, 59196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t35::n11104(s, out),
		['н', s @ ..] => super::t35::n11105(s, out),
		['п', s @ ..] => super::t35::n11106(s, out),
		['р', s @ ..] => super::t35::n11107(s, out),
		['т', s @ ..] => super::t35::n11108(s, out),
		['у', s @ ..] => {
			super::push(out, 11070);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11071);
			s.is_empty()
		}
		['ы', s @ ..] => super::t35::n11111(s, out),
		['ь', s @ ..] => super::t35::n11112(s, out),
		['ю', s @ ..] => super::t35::n11113(s, out),
		['я', s @ ..] => super::t35::n11114(s, out),
		_ => false,
	}
}

/// Words ending -юик.
pub(crate) fn n2292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2286);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 11076);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яик.
pub(crate) fn n2293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2287);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11077);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обер-шенк.
pub(crate) fn n124501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123218);
	s.is_empty()
}

/// The word мундшенк.
pub(crate) fn n124502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123219);
	s.is_empty()
}

/// Words ending -спрингбок.
pub(crate) fn n124503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123220);
	s.is_empty()
}

/// The word ястребок.
pub(crate) fn n124504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123221);
	s.is_empty()
}

/// Words ending -разжелобок.
pub(crate) fn n124505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123222);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 131721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -робок.
pub(crate) fn n31442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31188);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 59312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рбок.
pub(crate) fn n11154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11115);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31189);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 59313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87331);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 87332);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 109004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 87333);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 109005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убок.
pub(crate) fn n11155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11116);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 31190);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 31191);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 31192);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 59315);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 31193);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 59316);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 87335);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109006);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 123223);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131722);
													match s {
														[] => true,
														['м', s @ ..] => super::t35::n138122(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 59317);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 59318);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59319);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87337);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 109007);
									s.is_empty()
								}
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
			super::push(out, 31194);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 59320);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 59321);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 59322);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 59323);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87340);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 59324);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87341);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109009);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 59325);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87342);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109010);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 87343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 31195);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 59326);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87344);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109011);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 123224);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьбок.
pub(crate) fn n11156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11117);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31196);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 59327);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 87345);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109012);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123225);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 131723);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ябок.
pub(crate) fn n11157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11118);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 31197);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 59328);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вок.
pub(crate) fn n2308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2302);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11119);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11120);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31198);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 31199);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59329);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 87346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 59330);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 59331);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87348);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109013);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 59332);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87349);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 87350);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109014);
											s.is_empty()
										}
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
					super::push(out, 31200);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11121);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 31201);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 31202);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 31203);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59334);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 87351);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109015);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123226);
													match s {
														[] => true,
														['п', s @ ..] => super::t35::n133083(s, out),
														_ => false,
													}
												}
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
							super::push(out, 59335);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 59336);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 59337);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31204);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 59338);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 87352);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109016);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 59339);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87353);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109017);
											s.is_empty()
										}
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
					super::push(out, 31205);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59340);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 59341);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87354);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 59342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 31206);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59343);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 87355);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109018);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11122);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31207);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59344);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 31208);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 59345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31209);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 59346);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31210);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59347);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87359);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 109019);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11123);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11124);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31212);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 59348);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 31213);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 31214);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59349);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 87361);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109020);
											s.is_empty()
										}
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
					super::push(out, 31215);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59350);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31216);
					match s {
						[] => true,
						['к', s @ ..] => super::t35::n59980(s, out),
						['о', s @ ..] => super::t35::n59981(s, out),
						['т', s @ ..] => super::t35::n59982(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t35::n31471(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t35::n11164(s, out),
		_ => false,
	}
}

/// Words ending -гок.
pub(crate) fn n2309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2303);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11126);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -док.
pub(crate) fn n2310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2304);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11127);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11128);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31220);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59359);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 59360);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 87369);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109025);
											s.is_empty()
										}
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
					super::push(out, 31221);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59361);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 87370);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109026);
											s.is_empty()
										}
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
					super::push(out, 31222);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59362);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87371);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 59363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31223);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 59364);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 87372);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 31224);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59365);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 59366);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 87373);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 59367);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 59368);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 59369);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 59370);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87376);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 59371);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87377);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109027);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11129);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31225);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 59372);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87378);
									s.is_empty()
								}
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
			super::push(out, 11130);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31226);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 59373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31227);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 59374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31228);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 31229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59376);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 59377);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87379);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109028);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123228);
													match s {
														[] => true,
														['п', s @ ..] => super::t35::n133085(s, out),
														_ => false,
													}
												}
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
							super::push(out, 59378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31231);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 31232);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31233);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 59380);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87380);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 87381);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109029);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 59381);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87382);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109030);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123229);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 31235);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59383);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t35::n11171(s, out),
		['и', s @ ..] => {
			super::push(out, 11133);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31241);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 31242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 31243);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59393);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 87391);
									s.is_empty()
								}
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
			super::push(out, 11134);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31244);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59394);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87392);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 109037);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t35::n11174(s, out),
		['т', s @ ..] => super::t35::n11175(s, out),
		['у', s @ ..] => super::t35::n11176(s, out),
		['щ', s @ ..] => super::t35::n11177(s, out),
		['ь', s @ ..] => super::t35::n11178(s, out),
		['ю', s @ ..] => super::t35::n11179(s, out),
		['я', s @ ..] => super::t35::n11180(s, out),
		_ => false,
	}
}

/// Words ending -жок.
pub(crate) fn n2311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2305);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11142);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11143);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31263);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 31264);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59426);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87426);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109061);
											s.is_empty()
										}
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
					super::push(out, 31265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11144);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31266);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59427);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87427);
									s.is_empty()
								}
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
			super::push(out, 11145);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11146);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31267);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31268);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 59429);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 87428);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109062);
											s.is_empty()
										}
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
					super::push(out, 31269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11147);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11148);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 31271);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 31272);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 31273);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59432);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 87429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31274);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59433);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 59434);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87430);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 109063);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123244);
													match s {
														[] => true,
														['о', s @ ..] => super::t35::n133094(s, out),
														_ => false,
													}
												}
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
							super::push(out, 59435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11149);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11150);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31276);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 31277);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59436);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87431);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 59437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 11151);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 11152);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 11153);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31278);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зок.
pub(crate) fn n2312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2306);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11154);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31279);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59438);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31280);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59440);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 59441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11155);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31282);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59442);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87433);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 59443);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87434);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 59444);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87435);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109064);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11156);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31283);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 59445);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 87436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11157);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31285);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 59446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11158);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31286);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59447);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87437);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109065);
											s.is_empty()
										}
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
					super::push(out, 31287);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59448);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31288);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59449);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87439);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109066);
											s.is_empty()
										}
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
					super::push(out, 31289);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59450);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 87440);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109067);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 123245);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 109068);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123246);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t35::n133095(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 109069);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11159);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31290);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59451);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 87441);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109070);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11160);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31291);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 59452);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 31292);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31293);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59454);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87443);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109071);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11161);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31294);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59455);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31295);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59456);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87445);
									s.is_empty()
								}
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
			super::push(out, 11162);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31296);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 31297);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59457);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87446);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109072);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иок.
pub(crate) fn n2313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2307);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11163);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31298);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 59458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кок.
pub(crate) fn n2314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2308);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11164);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 11165);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31299);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31300);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лок.
pub(crate) fn n2315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2309);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11166);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11167);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31301);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 31302);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59459);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87447);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109073);
											s.is_empty()
										}
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
					super::push(out, 31303);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59460);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87448);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109074);
											s.is_empty()
										}
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
					super::push(out, 31304);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59461);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 59462);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 87450);
									s.is_empty()
								}
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
			super::push(out, 11168);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 31305);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11169);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31307);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 59463);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87451);
									s.is_empty()
								}
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
			super::push(out, 11170);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31308);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59464);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87452);
									s.is_empty()
								}
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
			super::push(out, 11171);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31309);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59465);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 87453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 31310);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 59466);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 31311);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 59467);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87455);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109075);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 59468);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87456);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109076);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123247);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31312);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59469);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 59470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 31313);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59471);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 87458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31314);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59472);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87459);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109077);
											s.is_empty()
										}
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
					super::push(out, 31315);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 59473);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31316);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59474);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87461);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 87462);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 109078);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123248);
													match s {
														[] => true,
														['л', s @ ..] => super::t35::n133096(s, out),
														_ => false,
													}
												}
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
							super::push(out, 59475);
							match s {
								[] => true,
								['ж', s @ ..] => super::t35::n88409(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t35::n60105(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t35::n31572(s, out),
				['т', s @ ..] => super::t35::n31573(s, out),
				['щ', s @ ..] => super::t35::n31574(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t35::n11211(s, out),
		['й', s @ ..] => super::t35::n11212(s, out),
		['к', s @ ..] => super::t35::n11213(s, out),
		['л', s @ ..] => super::t35::n11214(s, out),
		['о', s @ ..] => super::t35::n11215(s, out),
		['п', s @ ..] => super::t35::n11216(s, out),
		['р', s @ ..] => super::t35::n11217(s, out),
		['у', s @ ..] => super::t35::n11218(s, out),
		['ы', s @ ..] => super::t35::n11219(s, out),
		_ => false,
	}
}

/// Words ending -мок.
pub(crate) fn n2316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2310);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11181);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31349);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 31350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59536);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 59537);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87505);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 87506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31351);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59538);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87507);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109100);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11182);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31352);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59539);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 59540);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31353);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31354);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59542);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87510);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109101);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 59543);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 87511);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109102);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 87512);
									s.is_empty()
								}
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
			super::push(out, 11183);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31355);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 31356);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 31357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11184);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31358);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 59545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11185);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31359);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 31360);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 31361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59546);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 59547);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87513);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 59548);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 59549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31362);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59550);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87515);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109103);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 123255);
													match s {
														[] => true,
														['п', s @ ..] => super::t35::n133099(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31363);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59551);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87516);
									s.is_empty()
								}
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
			super::push(out, 11186);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31364);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 59552);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 87517);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109104);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 87518);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109105);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123256);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11187);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11188);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31366);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 59553);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87519);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109106);
											s.is_empty()
										}
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
					super::push(out, 31367);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59554);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87520);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109107);
											s.is_empty()
										}
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
					super::push(out, 31368);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59555);
							match s {
								[] => true,
								['е', s @ ..] => super::t35::n88467(s, out),
								['ж', s @ ..] => super::t35::n88468(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t35::n31624(s, out),
				['с', s @ ..] => super::t35::n31625(s, out),
				['ш', s @ ..] => super::t35::n31626(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t35::n11228(s, out),
		['ы', s @ ..] => super::t35::n11229(s, out),
		['я', s @ ..] => super::t35::n11230(s, out),
		_ => false,
	}
}

/// Words ending -нок.
pub(crate) fn n2317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2311);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11192);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11193);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31376);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59563);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87528);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109112);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 59564);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 31377);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59565);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 87530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 59566);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 87531);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109113);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 123258);
													match s {
														[] => true,
														['т', s @ ..] => super::t35::n133100(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 109114);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 123259);
													match s {
														[] => true,
														['о', s @ ..] => super::t35::n133101(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31378);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59567);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 31379);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59568);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87533);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 87534);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109115);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 59569);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31380);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59570);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 87536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31382);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59572);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 87537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 59573);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87538);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109116);
											s.is_empty()
										}
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
					super::push(out, 31383);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 59574);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 87539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31384);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59575);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87540);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109117);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 59576);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 87541);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109118);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 87542);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 109119);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 87543);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109120);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123260);
													match s {
														[] => true,
														['п', s @ ..] => super::t35::n133102(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t35::n11233(s, out),
		['и', s @ ..] => super::t35::n11234(s, out),
		['л', s @ ..] => {
			super::push(out, 11196);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31412);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 59642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t35::n11236(s, out),
		['с', s @ ..] => {
			super::push(out, 11198);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31424);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59661);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87619);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109173);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 59662);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 59663);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t35::n11238(s, out),
		['ы', s @ ..] => super::t35::n11239(s, out),
		['ю', s @ ..] => super::t35::n11240(s, out),
		_ => false,
	}
}

/// Words ending -пок.
pub(crate) fn n2318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2312);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11202);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31442);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 31443);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 59694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59695);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 59696);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87645);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109187);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11203);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31445);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59697);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 59698);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 59699);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87647);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 87648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31446);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 59700);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 87649);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', '\u{1}', s @ ..] => {
					super::push(out, 31447);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 31448);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59701);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87650);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109188);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11204);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31449);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', 'о', s @ ..] => {
					super::push(out, 31450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', 'у', s @ ..] => {
					super::push(out, 31451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11205);
			match s {
				[] => true,
				['л', 'х', '\u{1}', s @ ..] => {
					super::push(out, 31452);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 31453);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31454);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59706);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87651);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109189);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11206);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31455);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11207);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31456);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 59708);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 87652);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109190);
											s.is_empty()
										}
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
					super::push(out, 31457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31458);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59710);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 87653);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109191);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 59711);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рок.
pub(crate) fn n2319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2313);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11208);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 11209);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31459);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 31460);
					s.is_empty()
				}
				['ф', 'р', 'е', 'с', '\u{1}', s @ ..] => {
					super::push(out, 31461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 11210);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59712);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 59713);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87655);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109192);
											s.is_empty()
										}
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
					super::push(out, 31463);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59714);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87656);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 31464);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59715);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87657);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 59716);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 31465);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 31466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31467);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 31468);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59718);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87659);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109193);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123289);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 87660);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109194);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 123290);
													match s {
														[] => true,
														['п', s @ ..] => super::t35::n133117(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31469);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59719);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87661);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109195);
											s.is_empty()
										}
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
					super::push(out, 31470);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59720);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 87662);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109196);
											s.is_empty()
										}
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
					super::push(out, 31471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 11211);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31472);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 59721);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87663);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 109197);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 123291);
													match s {
														[] => true,
														['г', s @ ..] => super::t35::n133118(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 11212);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11213);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31475);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11214);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31476);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 59724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 31477);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 59725);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87665);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 59726);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 87666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 59727);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87667);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109198);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123292);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => super::t35::n31733(s, out),
				['м', s @ ..] => super::t35::n31734(s, out),
				['т', s @ ..] => {
					super::push(out, 31480);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59733);
							s.is_empty()
						}
						['с', s @ ..] => super::t35::n60364(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => super::t35::n31736(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t35::n11254(s, out),
		['о', s @ ..] => super::t35::n11255(s, out),
		['п', s @ ..] => super::t35::n11256(s, out),
		['с', s @ ..] => super::t35::n11257(s, out),
		['т', s @ ..] => super::t35::n11258(s, out),
		['у', s @ ..] => super::t35::n11259(s, out),
		['ф', s @ ..] => super::t35::n11260(s, out),
		['ы', s @ ..] => super::t35::n11261(s, out),
		['ю', s @ ..] => super::t35::n11262(s, out),
		['я', s @ ..] => super::t35::n11263(s, out),
		_ => false,
	}
}

/// Words ending -сок.
pub(crate) fn n2320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2314);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11225);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11226);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31528);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 31529);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 59791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31530);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31531);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59793);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87714);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109220);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123302);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31532);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59794);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87715);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109221);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123303);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11227);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59795);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 59796);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87716);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109222);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 59797);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31534);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59798);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 59799);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87718);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109223);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 59800);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87719);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109224);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 59801);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 59802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31535);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59803);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 59804);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87720);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109225);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 123304);
													match s {
														[] => true,
														['а', s @ ..] => super::t35::n133123(s, out),
														_ => false,
													}
												}
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
							super::push(out, 59805);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87721);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109226);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123305);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 87722);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109227);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123306);
													s.is_empty()
												}
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
							super::push(out, 59806);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 87723);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109228);
											s.is_empty()
										}
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
					super::push(out, 31536);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59807);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 59808);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31537);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 59809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11228);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31539);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 59811);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87726);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109229);
											s.is_empty()
										}
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
					super::push(out, 31540);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t35::n11268(s, out),
		['р', s @ ..] => {
			super::push(out, 11230);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31546);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 59825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11231);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31547);
					match s {
						[] => true,
						['и', s @ ..] => super::t35::n60457(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t35::n11271(s, out),
		['ы', s @ ..] => super::t35::n11272(s, out),
		['я', s @ ..] => super::t35::n11273(s, out),
		_ => false,
	}
}

/// Words ending -ток.
pub(crate) fn n2321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2315);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11235);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 11236);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31552);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 59831);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87744);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109238);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123310);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31553);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 59832);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87745);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109239);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11237);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31554);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 31555);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59834);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 87747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 59835);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 31556);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 31557);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31558);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 59838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31559);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 59839);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87749);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109240);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 59840);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87750);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31560);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 59841);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31561);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 59842);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31562);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 59843);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 31563);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 31564);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11238);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31565);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 59845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11239);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31566);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 59846);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 31567);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 59847);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 87755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59848);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 59849);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87756);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109241);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 59850);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 87757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 59851);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 59852);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 59853);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87758);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109242);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123311);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t35::n133125(s, out),
														_ => false,
													}
												}
												['ы', s @ ..] => {
													super::push(out, 123312);
													match s {
														[] => true,
														['т', s @ ..] => super::t35::n133126(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t35::n31824(s, out),
				['н', s @ ..] => super::t35::n31825(s, out),
				['р', s @ ..] => super::t35::n31826(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t36::n11279(s, out),
		['к', s @ ..] => super::t36::n11280(s, out),
		['л', s @ ..] => super::t36::n11281(s, out),
		['о', s @ ..] => super::t36::n11282(s, out),
		['р', s @ ..] => super::t36::n11283(s, out),
		['с', s @ ..] => super::t36::n11284(s, out),
		['т', s @ ..] => {
			super::push(out, 11246);
			match s {
				[] => true,
				['о', s @ ..] => super::t36::n31861(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t36::n11286(s, out),
		['ш', s @ ..] => super::t36::n11287(s, out),
		['ы', s @ ..] => super::t36::n11288(s, out),
		['э', s @ ..] => super::t36::n11289(s, out),
		['я', s @ ..] => super::t36::n11290(s, out),
		_ => false,
	}
}

/// Words ending -уок.
pub(crate) fn n2322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2316);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 11252);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31621);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 59934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фок.
pub(crate) fn n2323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2317);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11253);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11254);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31622);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 59935);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87842);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109292);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123343);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хок.
pub(crate) fn n2324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2318);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 11255);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31623);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 59936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чок.
pub(crate) fn n2325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2319);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11256);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11257);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31624);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59937);
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 87843);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 31625);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 59938);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87844);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 109293);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123344);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31626);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 59939);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 87845);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109294);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59940);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 59941);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 87847);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 87848);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 87849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 31627);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 31628);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 59942);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 87850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 59943);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 87851);
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
						['\u{1}', s @ ..] => {
							super::push(out, 59944);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 59945);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 59946);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 87852);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109295);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123345);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59947);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 59948);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 87853);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109296);
											s.is_empty()
										}
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
					super::push(out, 31630);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 59949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31631);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 59950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31632);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59951);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 59952);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 59953);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 87854);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109297);
											s.is_empty()
										}
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
					super::push(out, 31633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 59954);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 59955);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 59956);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 87856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 59957);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 87857);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 87858);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 59958);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'с', 'о', s @ ..] => {
					super::push(out, 31634);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 59959);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87860);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109298);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							super::push(out, 59960);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 59961);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 87861);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109299);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11258);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 31635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t36::n11298(s, out),
		['л', s @ ..] => {
			super::push(out, 11260);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31643);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 59968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t36::n31903(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11261);
			match s {
				[] => true,
				['а', s @ ..] => super::t36::n31904(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t36::n11301(s, out),
		['р', s @ ..] => super::t36::n11302(s, out),
		['у', s @ ..] => super::t36::n11303(s, out),
		['ы', s @ ..] => super::t36::n11304(s, out),
		['ю', s @ ..] => super::t36::n11305(s, out),
		['я', s @ ..] => super::t36::n11306(s, out),
		_ => false,
	}
}

/// Words ending -шок.
pub(crate) fn n2326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2320);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11268);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11269);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31670);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 31671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11270);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31672);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60013);
							match s {
								[] => true,
								['р', 'г', '\u{1}', s @ ..] => {
									super::push(out, 87912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31673);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31674);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60015);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87913);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109332);
											s.is_empty()
										}
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
					super::push(out, 31675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11271);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31676);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60016);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 87914);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109333);
											s.is_empty()
										}
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
					super::push(out, 31677);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31678);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60018);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 60019);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87916);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 60020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11272);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31680);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60022);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87917);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 109334);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123363);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31681);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60023);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87918);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 60024);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 87919);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 109335);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 123364);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133149(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31682);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11273);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31683);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60026);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 60027);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 87920);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109336);
											s.is_empty()
										}
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
					super::push(out, 31684);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31685);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60029);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 87921);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 109337);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11274);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31686);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 31687);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 31688);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 31689);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 31690);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60031);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87922);
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
						['а', s @ ..] => {
							super::push(out, 60032);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87923);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109338);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 60033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31691);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 31692);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60034);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 87924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 60035);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 87925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t36::n60673(s, out),
						_ => false,
					}
				}
				['ч', '\u{1}', s @ ..] => super::t36::n31960(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t36::n11314(s, out),
		['я', s @ ..] => super::t36::n11315(s, out),
		_ => false,
	}
}

/// Words ending -щок.
pub(crate) fn n2327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2321);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11277);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31697);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word фольварк.
pub(crate) fn n124649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123366);
	s.is_empty()
}

/// Words ending -ерверк.
pub(crate) fn n60687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60050);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 87937);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 109344);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 123367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сверк.
pub(crate) fn n31983(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 60051);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 87938);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 109345);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 123368);
									s.is_empty()
								}
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
			super::push(out, 60052);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 87939);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 109346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 60053);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 60054);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 87941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тверк.
pub(crate) fn n31984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31716);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 60055);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87942);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 109347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хверк.
pub(crate) fn n31985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31717);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 60056);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 87943);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьверк.
pub(crate) fn n31986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31718);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 60057);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 87944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узерк.
pub(crate) fn n31987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31719);
	s.is_empty()
}

/// Words ending -керк.
pub(crate) fn n11328(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 31720);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 60058);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 87945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 31721);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 60059);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лерк.
pub(crate) fn n11329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11289);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 31722);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -серк.
pub(crate) fn n11330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11290);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 31723);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 60060);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -черк.
pub(crate) fn n11331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11291);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31724);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 31725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирк.
pub(crate) fn n2330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2324);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11292);
			s.is_empty()
		}
		['ц', s @ ..] => {
			super::push(out, 11293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -крк.
pub(crate) fn n2331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2325);
	s.is_empty()
}

/// Words ending -орк.
pub(crate) fn n2332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2326);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11294);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 11295);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31726);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 60061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31727);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 31728);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60063);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87947);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109348);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123369);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11296);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 31729);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60064);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 87948);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109349);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 123370);
													s.is_empty()
												}
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
							super::push(out, 60065);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 87949);
									s.is_empty()
								}
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
			super::push(out, 11297);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31730);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60066);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 87950);
									s.is_empty()
								}
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
			super::push(out, 11298);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31731);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урк.
pub(crate) fn n2333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2327);
	s.is_empty()
}

/// Words ending -ырк.
pub(crate) fn n2334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2328);
	s.is_empty()
}

/// Words ending -юрк.
pub(crate) fn n2335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2329);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 11300);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11301);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31733);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -евск.
pub(crate) fn n11349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11308);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31741);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 60074);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 87956);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 109353);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 123371);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 131792);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 60075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87957);
							match s {
								[] => true,
								['к', 'и', 'н', 'о', s @ ..] => {
									super::push(out, 109354);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123372);
											s.is_empty()
										}
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
					super::push(out, 60076);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 60077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 31742);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 60078);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 87959);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 109355);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123373);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 131793);
													match s {
														[] => true,
														['м', s @ ..] => super::t36::n138143(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 60079);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 87960);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109356);
									s.is_empty()
								}
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
			super::push(out, 31743);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 60080);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31744);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 60081);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87962);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109357);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123374);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 131794);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31745);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 60082);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87963);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 60083);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 87964);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 109359);
									s.is_empty()
								}
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
			super::push(out, 31746);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 31747);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 60084);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 87965);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 109360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 60085);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 31748);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 60086);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87967);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 109361);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 123375);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 131795);
													match s {
														[] => true,
														['о', s @ ..] => super::t36::n138144(s, out),
														_ => false,
													}
												}
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
							super::push(out, 87968);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 109362);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 123376);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 131796);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31749);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 60087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овск.
pub(crate) fn n11350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11309);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 31750);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 60088);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 87969);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109363);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123377);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'а', s @ ..] => {
			super::push(out, 31751);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 60089);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87970);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109364);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 123378);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31752);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 60090);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 87971);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109365);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 123379);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 131797);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 60091);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 87972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 60092);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87973);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 109366);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 123380);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 131798);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
				['в', s @ ..] => {
					super::push(out, 60093);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 87974);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 60094);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87975);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 109368);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 123381);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 131799);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 31753);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 60095);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 87976);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 109369);
									s.is_empty()
								}
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
			super::push(out, 31754);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 60096);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 87977);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 109370);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 123382);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 31755);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 60097);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 87978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 60098);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 87979);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 109371);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 123383);
											s.is_empty()
										}
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
					super::push(out, 60099);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 87980);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 109372);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 123384);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 131800);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', s @ ..] => {
			super::push(out, 31756);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 60100);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 87981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еск.
pub(crate) fn n2340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2333);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11311);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31758);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60101);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87982);
									s.is_empty()
								}
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
			super::push(out, 11312);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11313);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60102);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 60103);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87983);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109373);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 60104);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 60105);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87985);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109374);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 60106);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 31761);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60107);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 87987);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 60108);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 60109);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87988);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 87989);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109375);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123385);
													s.is_empty()
												}
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
							super::push(out, 60110);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 87990);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 60111);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 87991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31762);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60112);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 87992);
									s.is_empty()
								}
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
			super::push(out, 11314);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31763);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31764);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 31765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11315);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31766);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 31767);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60114);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87993);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 109376);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11316);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31768);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60115);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 87994);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 109377);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 123386);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133160(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11317);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31769);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60116);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 87995);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 109378);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ложск.
pub(crate) fn n32040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31771);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 60118);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 87996);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 109379);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 123387);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 131802);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иск.
pub(crate) fn n2343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2336);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11320);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 31772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11321);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31773);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11322);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31774);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31775);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60120);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 87997);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 109380);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123388);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11323);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11324);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31777);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11325);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 11326);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31778);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31779);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 87998);
									s.is_empty()
								}
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
			super::push(out, 11327);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31780);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31781);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 11328);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31782);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60125);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 87999);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 109381);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йск.
pub(crate) fn n2344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2337);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11329);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31783);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60126);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88000);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109382);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11330);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11331);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мск.
pub(crate) fn n2346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2339);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нск.
pub(crate) fn n2347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2340);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11333);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 31784);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 31785);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60127);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88001);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109383);
											s.is_empty()
										}
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
					super::push(out, 31786);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60128);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 31787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11334);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31788);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60129);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88003);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109384);
											s.is_empty()
										}
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
					super::push(out, 31789);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 31790);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60131);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88004);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109385);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123389);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31791);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60132);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 31792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11335);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31793);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 60133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 31794);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60134);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88006);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 88007);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109386);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123390);
													s.is_empty()
												}
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
							super::push(out, 60135);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88008);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 109387);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123391);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t36::n133162(s, out),
														_ => false,
													}
												}
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
							super::push(out, 60136);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 88009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 60137);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88010);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 109388);
											s.is_empty()
										}
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
					super::push(out, 31795);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60138);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 31796);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60139);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 31797);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60140);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88013);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 109389);
											s.is_empty()
										}
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
					super::push(out, 31798);
					match s {
						[] => true,
						['а', s @ ..] => super::t36::n60778(s, out),
						['т', s @ ..] => super::t36::n60779(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t36::n32068(s, out),
				['м', s @ ..] => super::t36::n32069(s, out),
				['н', s @ ..] => super::t36::n32070(s, out),
				['п', s @ ..] => super::t36::n32071(s, out),
				['р', s @ ..] => super::t36::n32072(s, out),
				['с', s @ ..] => super::t36::n32073(s, out),
				['т', s @ ..] => super::t36::n32074(s, out),
				['у', s @ ..] => super::t36::n32075(s, out),
				['ч', s @ ..] => super::t36::n32076(s, out),
				['ь', s @ ..] => super::t36::n32077(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t36::n11377(s, out),
		['я', s @ ..] => super::t36::n11378(s, out),
		_ => false,
	}
}

/// Words ending -оск.
pub(crate) fn n2348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2341);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11338);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11339);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11340);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рск.
pub(crate) fn n2349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2342);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11341);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11342);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31813);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60164);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88033);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109405);
											s.is_empty()
										}
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
					super::push(out, 31814);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 31815);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60166);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88034);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 109406);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123398);
													match s {
														[] => true,
														['с', s @ ..] => super::t36::n133164(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11343);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 11344);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 31816);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60167);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88035);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 109407);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 123399);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'н', 'д', s @ ..] => {
							super::push(out, 60168);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88036);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109408);
											s.is_empty()
										}
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
					super::push(out, 31817);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60169);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 60170);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88038);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109409);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сск.
pub(crate) fn n2350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2343);
	s.is_empty()
}

/// Words ending -тск.
pub(crate) fn n2351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2344);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11345);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уск.
pub(crate) fn n2352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2345);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 11347);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31818);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 60171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 31819);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 31820);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60172);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31821);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60173);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88040);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109410);
											s.is_empty()
										}
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
					super::push(out, 31822);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60174);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 88041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 31823);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60175);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31824);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60176);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 60177);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88043);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109411);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123400);
													s.is_empty()
												}
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
							super::push(out, 60178);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 60179);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88044);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109412);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 109413);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 123401);
													match s {
														[] => true,
														['е', s @ ..] => super::t36::n133165(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 109414);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123402);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 31825);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60180);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 60181);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88045);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 88046);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109415);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 88047);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31826);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 31827);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11348);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31828);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60185);
							s.is_empty()
						}
						['э', s @ ..] => {
							super::push(out, 60186);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыск.
pub(crate) fn n2353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2346);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11349);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11350);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 11351);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31832);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31833);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31834);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11352);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31835);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 31836);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60189);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 60190);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 60191);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88048);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109416);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 60192);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88049);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109417);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11353);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьск.
pub(crate) fn n2354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2347);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11354);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31837);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 31838);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60193);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 88050);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109418);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 123403);
													match s {
														[] => true,
														['р', s @ ..] => super::t36::n133166(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31839);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60194);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88051);
									match s {
										[] => true,
										['к', '-', s @ ..] => {
											super::push(out, 109419);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 123404);
													match s {
														[] => true,
														['н', s @ ..] => super::t36::n133167(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31840);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эск.
pub(crate) fn n2355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2348);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 11355);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31841);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88052);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юск.
pub(crate) fn n2356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2349);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11356);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31842);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60197);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88053);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яск.
pub(crate) fn n2357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2350);
	s.is_empty()
}

/// Words ending -джук.
pub(crate) fn n11417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11376);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31863);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 31864);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 60216);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 31865);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 60217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 31866);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 60218);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 88064);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 109423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужук.
pub(crate) fn n11418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11377);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 31867);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 60219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зук.
pub(crate) fn n2365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2358);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11378);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31868);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 60220);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88065);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109424);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11379);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 31869);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60221);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 88066);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109425);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кук.
pub(crate) fn n2366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2359);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 11380);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31870);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 11381);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31871);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60223);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лук.
pub(crate) fn n2367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2360);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 11382);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31872);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 11383);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 31873);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60225);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31874);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60226);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 60227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 31875);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60228);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88069);
									s.is_empty()
								}
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
			super::push(out, 11384);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31876);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 31877);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60230);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 88070);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 109426);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11385);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31878);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11386);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 31879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11387);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11388);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31881);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мук.
pub(crate) fn n2368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2361);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11389);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11390);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31882);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60232);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 88071);
									s.is_empty()
								}
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
			super::push(out, 11391);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31883);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нук.
pub(crate) fn n2369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2362);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11392);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31884);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60233);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88072);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 109427);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11393);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 31885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11394);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31886);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60234);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88073);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 109428);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пук.
pub(crate) fn n2370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2363);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11395);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 11396);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 31887);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рук.
pub(crate) fn n2371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2364);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11397);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31888);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 11398);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31889);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 31890);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60238);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88074);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109429);
											s.is_empty()
										}
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
					super::push(out, 31891);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60239);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88075);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109430);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 123405);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11399);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31892);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 31893);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11400);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31894);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11401);
			match s {
				[] => true,
				['н', 'х', 'е', 'т', '\u{1}', s @ ..] => {
					super::push(out, 31895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11402);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31896);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60243);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 88076);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109431);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11403);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 31897);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60244);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88077);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 109432);
											s.is_empty()
										}
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
					super::push(out, 31898);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сук.
pub(crate) fn n2372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2365);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11404);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 11405);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 31899);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60245);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88078);
									s.is_empty()
								}
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
			super::push(out, 11406);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31900);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60246);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88079);
									s.is_empty()
								}
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
			super::push(out, 11407);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31901);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31902);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тук.
pub(crate) fn n2373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2366);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 11408);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31903);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60249);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88080);
									s.is_empty()
								}
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
			super::push(out, 11409);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31904);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 31905);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11410);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31906);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60251);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88081);
									s.is_empty()
								}
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
			super::push(out, 11411);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 31907);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60252);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 60253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31908);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 60254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 31909);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31910);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 31911);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60257);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 88083);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 109433);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123406);
													match s {
														[] => true,
														['ж', s @ ..] => super::t36::n133168(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11413);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 31912);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60258);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88084);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 109434);
											s.is_empty()
										}
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
					super::push(out, 31913);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60259);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 31914);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60260);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 88086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31915);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 31916);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11414);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 31917);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60263);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 88087);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 109435);
											s.is_empty()
										}
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
					super::push(out, 31918);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60264);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 31919);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60265);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88089);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109436);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11415);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31920);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60266);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88090);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уук.
pub(crate) fn n2374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2367);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11416);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хук.
pub(crate) fn n2376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2369);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 11417);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31921);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60267);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109437);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цук.
pub(crate) fn n2377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2370);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 11418);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31922);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 60268);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 88092);
									s.is_empty()
								}
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
			super::push(out, 11419);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31923);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 60269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чук.
pub(crate) fn n2378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2371);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11420);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31924);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 11421);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31925);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60271);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 60272);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88093);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109438);
											s.is_empty()
										}
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
					super::push(out, 31926);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60273);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 31927);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31928);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11422);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31929);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60276);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 60277);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88095);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 109439);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11423);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31930);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60278);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88096);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109440);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 123407);
													match s {
														[] => true,
														['п', s @ ..] => super::t36::n133169(s, out),
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
									super::push(out, 88097);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 109441);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 123408);
													match s {
														[] => true,
														['у', s @ ..] => super::t36::n133170(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 31931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 11424);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31932);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60280);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88098);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шук.
pub(crate) fn n2379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2372);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11425);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11426);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31934);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щук.
pub(crate) fn n2380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2373);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 11427);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 31935);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60282);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 31936);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60283);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88100);
									s.is_empty()
								}
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
			super::push(out, 11428);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11429);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зылык.
pub(crate) fn n32240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31970);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 60306);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 88115);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йылык.
pub(crate) fn n32241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31971);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 60307);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 88116);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мык.
pub(crate) fn n2391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2384);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11459);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 11460);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31973);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11461);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 11462);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 31974);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хнык.
pub(crate) fn n11504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11463);
	s.is_empty()
}

/// Words ending -рык.
pub(crate) fn n2393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2386);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11464);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 11465);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 31975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11466);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 11467);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31976);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60309);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88117);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109445);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тык.
pub(crate) fn n2394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2387);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11468);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 11469);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 11470);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 11471);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60310);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 60311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11472);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 31977);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11473);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31978);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 60313);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 60314);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88120);
									s.is_empty()
								}
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
			super::push(out, 11474);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31979);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 31980);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60315);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 88121);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109446);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цык.
pub(crate) fn n2395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2388);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11475);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 31981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чык.
pub(crate) fn n2396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2389);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 11476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аннингбэк.
pub(crate) fn n124692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123409);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 131812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -джонсонюк.
pub(crate) fn n124693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123410);
	s.is_empty()
}

/// Words ending -трюк.
pub(crate) fn n11554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11513);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 32014);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 32015);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 60337);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 88132);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 109450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хрюк.
pub(crate) fn n11556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11515);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32017);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 60338);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юрюк.
pub(crate) fn n11557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11516);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 32018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сюк.
pub(crate) fn n2415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2408);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11517);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11518);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32020);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тюк.
pub(crate) fn n2416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2409);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11519);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 32021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11520);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 32022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11521);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 32023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11522);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32024);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', 'н', 'у', 'к', '\u{1}', s @ ..] => {
			super::push(out, 11523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьюк.
pub(crate) fn n2417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2410);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11524);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word здоровяк.
pub(crate) fn n124694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123411);
	s.is_empty()
}

/// Words ending -ляк.
pub(crate) fn n2425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2418);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 11542);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32045);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60353);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88143);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 109455);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123412);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11543);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32046);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60354);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 32047);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60355);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88145);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 60356);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88146);
									s.is_empty()
								}
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
			super::push(out, 11544);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32048);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 32049);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11545);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 32051);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 60360);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88147);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32052);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60361);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88148);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109456);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11546);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32053);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 32054);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60363);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88149);
									s.is_empty()
								}
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
			super::push(out, 11547);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 32055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11548);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32056);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60364);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88150);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 109457);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123413);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 32057);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 60365);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88151);
									s.is_empty()
								}
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
			super::push(out, 11549);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 60366);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 60367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88152);
									s.is_empty()
								}
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
			super::push(out, 11550);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32058);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60368);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 32059);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 32060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11551);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32062);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 60371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60372);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 60373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11552);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11553);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11554);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32065);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60374);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88154);
									s.is_empty()
								}
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
			super::push(out, 11555);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 11556);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32067);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 32068);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60375);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 88155);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 109458);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 123414);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11557);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32069);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 32070);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60377);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 60378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 32071);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11558);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32072);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омяк.
pub(crate) fn n11601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11559);
	match s {
		[] => true,
		['к', '\u{1}', s @ ..] => {
			super::push(out, 32073);
			s.is_empty()
		}
		['х', '\u{1}', s @ ..] => {
			super::push(out, 32074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рмяк.
pub(crate) fn n11602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11560);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32075);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 32076);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 60381);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хмяк.
pub(crate) fn n11603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11561);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32077);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 60382);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word шмяк.
pub(crate) fn n32351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32078);
	s.is_empty()
}

/// Words ending -няк.
pub(crate) fn n2427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2420);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11563);
			match s {
				[] => true,
				['о', 'с', s @ ..] => {
					super::push(out, 32079);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 32080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 11564);
			match s {
				[] => true,
				['и', 'а', s @ ..] => {
					super::push(out, 32081);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60383);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88156);
									s.is_empty()
								}
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
			super::push(out, 11565);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32082);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60384);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 60385);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88157);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 109459);
											s.is_empty()
										}
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
					super::push(out, 32083);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 60386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 11566);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32084);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60387);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88158);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109460);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11567);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32085);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60388);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32086);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60389);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 88160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 60390);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 88161);
									s.is_empty()
								}
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
			super::push(out, 11568);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					super::push(out, 32087);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 32088);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60392);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11569);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32090);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32091);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 11570);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32092);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60394);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88163);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109461);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 60395);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 60396);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88164);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 109462);
											s.is_empty()
										}
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
					super::push(out, 32093);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 60397);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 88165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 60398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11571);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32094);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32095);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 32096);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 32097);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60400);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 60401);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 60402);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 60403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11572);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32098);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32099);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60404);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 60405);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 32100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11573);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 32101);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 60406);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									super::push(out, 88170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 32102);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32103);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60407);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88171);
									match s {
										[] => true,
										['з', 'и', '\u{1}', s @ ..] => {
											super::push(out, 109463);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 11574);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 11575);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32104);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60408);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 32105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 11576);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ояк.
pub(crate) fn n2428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2421);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11577);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 32107);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 11578);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 32109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11579);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 32110);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ряк.
pub(crate) fn n2429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2422);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 11580);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32111);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 32112);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 60410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'о', 'б', '\u{1}', s @ ..] => {
			super::push(out, 11581);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 11582);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32113);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60411);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32114);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 32115);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60412);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 88174);
									s.is_empty()
								}
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
			super::push(out, 11583);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32116);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 60413);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 88175);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 109464);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 123415);
													match s {
														[] => true,
														['и', s @ ..] => super::t36::n133172(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 32117);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 32118);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 11584);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32119);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 32120);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 60415);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 88176);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109465);
											s.is_empty()
										}
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
					super::push(out, 32121);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 60416);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 88177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 60417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11585);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 32122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 32123);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 32124);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 11586);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32125);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 60420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 32126);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 60421);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 60422);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 88179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 60423);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 88180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 32127);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60424);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 88181);
									s.is_empty()
								}
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
			super::push(out, 11587);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32128);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 32129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 11588);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32130);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 60426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сяк.
pub(crate) fn n2430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2423);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 11589);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32131);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 60427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 11590);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11591);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 60428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 32134);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 60429);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 88182);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 109466);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 123416);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 11592);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 32135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тяк.
pub(crate) fn n2431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2424);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 11593);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 32136);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 60430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 11594);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 11595);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 32138);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 60431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 32139);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 60432);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 88183);
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
						['\u{1}', s @ ..] => {
							super::push(out, 60433);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 60434);
							s.is_empty()
						}
						['л', 'о', 'х', '\u{1}', s @ ..] => {
							super::push(out, 60435);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 60436);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 88184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 32140);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 60437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 11596);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 32141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 11597);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 32142);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 60438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уяк.
pub(crate) fn n2432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2425);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 11598);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фяк.
pub(crate) fn n2433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2426);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 11599);
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					super::push(out, 32143);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

