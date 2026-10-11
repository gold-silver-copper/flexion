use flexion::Path;

/// Words ending -che.
pub(crate) fn n1137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1135);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4901);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13266);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 13267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13268);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23874);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 32713);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 23875);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13269);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 23876);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 23877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13270);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23878);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 23879);
							match s {
								[] => true,
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 32715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13271);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23880);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 32716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13272);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23881);
							s.is_empty()
						}
						['p', s @ ..] => super::t12::n24691(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13273);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23883);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 32717);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13274);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 23884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13275);
					match s {
						[] => true,
						['n', 'e', 't', 's', s @ ..] => {
							super::push(out, 23885);
							match s {
								[] => true,
								['ü', 'k', '-', s @ ..] => {
									super::push(out, 32718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4902);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13276);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4903);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13277);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13278);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 23886);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32719);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 32720);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 23887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13279);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 13280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t12::n4932(s, out),
		['l', s @ ..] => super::t12::n4933(s, out),
		['n', s @ ..] => super::t12::n4934(s, out),
		['o', s @ ..] => super::t12::n4935(s, out),
		['r', s @ ..] => super::t12::n4936(s, out),
		['s', s @ ..] => super::t12::n4937(s, out),
		['u', s @ ..] => super::t12::n4938(s, out),
		['w', s @ ..] => {
			super::push(out, 4911);
			match s {
				[] => true,
				['i', s @ ..] => super::t12::n13518(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t12::n4940(s, out),
		['ä', s @ ..] => super::t12::n4941(s, out),
		['ü', s @ ..] => super::t12::n4942(s, out),
		_ => false,
	}
}

/// Words ending -eie.
pub(crate) fn n1151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1149);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 4952);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4953);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13355);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13356);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fie.
pub(crate) fn n1152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1150);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4954);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 23983);
							s.is_empty()
						}
						['i', 't', 'a', s @ ..] => {
							super::push(out, 23984);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 42148);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 42149);
													s.is_empty()
												}
												['r', 'u', s @ ..] => {
													super::push(out, 42150);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'a', s @ ..] => {
											super::push(out, 38692);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'o', s @ ..] => {
									super::push(out, 32777);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 32778);
									match s {
										[] => true,
										['t', 'i', 'l', 'o', s @ ..] => {
											super::push(out, 38693);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 32779);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 38694);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 32780);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', 'h', s @ ..] => {
											super::push(out, 38695);
											s.is_empty()
										}
										['e', 'p', s @ ..] => {
											super::push(out, 38696);
											s.is_empty()
										}
										['l', 'a', 't', 'e', s @ ..] => {
											super::push(out, 38697);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'o', 't', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', 'n', 'o', 'y', s @ ..] => {
											super::push(out, 38698);
											s.is_empty()
										}
										['z', s @ ..] => {
											super::push(out, 38699);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38700);
											s.is_empty()
										}
										['e', 'l', s @ ..] => {
											super::push(out, 38701);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 38702);
											s.is_empty()
										}
										['r', 'o', 'p', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 42151);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 42152);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 32781);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 32782);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', 'm', 'o', 'r', 'h', 'c', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 42153);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 42154);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'f', s @ ..] => {
											match s {
												[] => true,
												['-', 't', s @ ..] => {
													super::push(out, 42155);
													s.is_empty()
												}
												['g', s @ ..] => {
													super::push(out, 42156);
													s.is_empty()
												}
												['k', s @ ..] => {
													super::push(out, 42157);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 42158);
													s.is_empty()
												}
												['s', 'n', s @ ..] => {
													super::push(out, 42159);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 38703);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ä', s @ ..] => {
									super::push(out, 32783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4955);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13357);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gie.
pub(crate) fn n1153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1151);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4956);
			match s {
				[] => true,
				['t', 'a', 'r', 't', 's', 's', 'g', 'n', 'u', 'f', s @ ..] => {
					super::push(out, 13358);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 23985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4957);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 13359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4958);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13360);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4959);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4960);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13361);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 23986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13362);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 23987);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 32784);
									s.is_empty()
								}
								['n', 'a', 'h', s @ ..] => {
									super::push(out, 32785);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									super::push(out, 32786);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 32787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 23988);
							s.is_empty()
						}
						['o', s @ ..] => super::t12::n24804(s, out),
						['s', s @ ..] => {
							super::push(out, 23990);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 23991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4961);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13363);
					s.is_empty()
				}
				['e', s @ ..] => super::t12::n13576(s, out),
				['o', s @ ..] => {
					super::push(out, 13365);
					s.is_empty()
				}
				['u', s @ ..] => super::t12::n13578(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hie.
pub(crate) fn n1154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1152);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4962);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4963);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13368);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 32806);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 32807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', 'n', 'o', 'm', 'r', s @ ..] => {
					super::push(out, 13369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4964);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 23998);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 23999);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 38746);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 38747);
											s.is_empty()
										}
										['o', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['g', 'o', s @ ..] => super::t12::n46785(s, out),
														['r', 'o', s @ ..] => super::t12::n46786(s, out),
														_ => false,
													}
												}
												['h', 't', s @ ..] => {
													match s {
														[] => true,
														['i', 'l', 'n', s @ ..] => super::t12::n46787(s, out),
														['r', s @ ..] => super::t12::n46788(s, out),
														_ => false,
													}
												}
												['i', s @ ..] => {
													super::push(out, 42186);
													match s {
														[] => true,
														['b', s @ ..] => super::t12::n46789(s, out),
														['d', 'a', 'r', 'o', 'r', s @ ..] => super::t12::n46790(s, out),
														['g', 'n', 'a', 'n', s @ ..] => super::t12::n46791(s, out),
														_ => false,
													}
												}
												['k', s @ ..] => {
													super::push(out, 42187);
													s.is_empty()
												}
												['l', s @ ..] => super::t12::n44921(s, out),
												['m', 'y', s @ ..] => {
													super::push(out, 42188);
													s.is_empty()
												}
												['n', s @ ..] => super::t12::n44923(s, out),
												['p', 'y', s @ ..] => {
													super::push(out, 42189);
													s.is_empty()
												}
												['r', 'e', s @ ..] => {
													super::push(out, 42190);
													s.is_empty()
												}
												['t', s @ ..] => super::t12::n44926(s, out),
												['ä', s @ ..] => super::t12::n44927(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t12::n13583(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t12::n4996(s, out),
		['t', s @ ..] => super::t12::n4997(s, out),
		_ => false,
	}
}

/// Words ending -kie.
pub(crate) fn n1155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1153);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4967);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4968);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 13376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4969);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4970);
			match s {
				[] => true,
				['a', 't', '-', 'e', 'i', 'k', 'l', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 13378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4971);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4972);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13379);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 24005);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4973);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13381);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 4974);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lie.
pub(crate) fn n1156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1154);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', 'p', 'e', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24007);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 32810);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'o', 't', s @ ..] => {
							super::push(out, 24008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13383);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 13384);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13385);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24009);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 24010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13386);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13387);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13388);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 24011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4975);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13389);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13390);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 24012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24013);
							s.is_empty()
						}
						['o', 'r', s @ ..] => {
							super::push(out, 24014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13391);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24015);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 24016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4976);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13392);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 32811);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['i', 'x', s @ ..] => {
											super::push(out, 38760);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 38761);
											s.is_empty()
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['a', 'b', s @ ..] => {
													super::push(out, 42198);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 42199);
													s.is_empty()
												}
												['i', 's', s @ ..] => {
													super::push(out, 42200);
													s.is_empty()
												}
												['m', s @ ..] => {
													super::push(out, 42201);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['k', s @ ..] => {
													super::push(out, 42202);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 42203);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 38762);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['a', 'c', s @ ..] => {
													super::push(out, 42204);
													s.is_empty()
												}
												['k', s @ ..] => {
													match s {
														[] => true,
														['a', s @ ..] => super::t12::n46803(s, out),
														['e', s @ ..] => super::t12::n46804(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13393);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13394);
					match s {
						[] => true,
						['a', 'f', 'm', s @ ..] => {
							super::push(out, 24017);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 32812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'r', 'e', 't', 'e', 'p', 'l', s @ ..] => {
					super::push(out, 13395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4977);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13396);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13397);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 24018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4978);
			match s {
				[] => true,
				['b', 'a', s @ ..] => {
					super::push(out, 13398);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 13399);
					s.is_empty()
				}
				['h', 'c', 'a', s @ ..] => {
					super::push(out, 13400);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 13401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4979);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4980);
			s.is_empty()
		}
		['y', 't', 'k', 'a', 'd', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13403);
					s.is_empty()
				}
				['o', 'r', s @ ..] => {
					super::push(out, 13404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mie.
pub(crate) fn n1157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1155);
	match s {
		[] => true,
		['a', 'g', s @ ..] => {
			super::push(out, 4981);
			match s {
				[] => true,
				['i', 'b', s @ ..] => {
					super::push(out, 13405);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 24020);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 24021);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24022);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 13406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4982);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13407);
					match s {
						[] => true,
						['a', 'k', 'a', 'i', s @ ..] => {
							super::push(out, 24024);
							match s {
								[] => true,
								['e', 'z', 'i', 'l', 'o', 'p', 's', s @ ..] => {
									super::push(out, 32813);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 13408);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 32814);
									s.is_empty()
								}
								['l', 'l', s @ ..] => {
									super::push(out, 32815);
									s.is_empty()
								}
								['n', 'e', 'r', s @ ..] => {
									super::push(out, 32816);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 38763);
											s.is_empty()
										}
										['r', 'd', s @ ..] => {
											super::push(out, 38764);
											s.is_empty()
										}
										['t', 'o', s @ ..] => {
											super::push(out, 38765);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 24025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 't', 'y', s @ ..] => {
			match s {
				[] => true,
				['h', 'r', 'u', s @ ..] => {
					super::push(out, 13410);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4983);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4984);
			match s {
				[] => true,
				['u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 13413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4985);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13414);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13416);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 24027);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32817);
									s.is_empty()
								}
								['k', 'ö', 'l', s @ ..] => {
									super::push(out, 32818);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 32819);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 38766);
											s.is_empty()
										}
										['t', 's', 'a', 'g', 's', s @ ..] => {
											super::push(out, 38767);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13417);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24028);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 38768);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 38769);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 24029);
							match s {
								[] => true,
								['e', 'i', s @ ..] => {
									super::push(out, 32820);
									match s {
										[] => true,
										['r', 'e', s @ ..] => {
											super::push(out, 38770);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'o', s @ ..] => {
							super::push(out, 24030);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 32821);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'o', 'k', 'a', s @ ..] => {
							super::push(out, 24031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4986);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'o', s @ ..] => {
							super::push(out, 24032);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 24033);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['o', 'p', s @ ..] => {
											super::push(out, 38771);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 38772);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 's', 'o', s @ ..] => {
							super::push(out, 24034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'o', 'n', s @ ..] => {
			super::push(out, 4987);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4988);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4989);
			match s {
				[] => true,
				['h', 't', 'o', s @ ..] => {
					super::push(out, 13418);
					s.is_empty()
				}
				['n', 'o', s @ ..] => {
					match s {
						[] => true,
						['p', 'o', 'r', s @ ..] => {
							super::push(out, 24035);
							s.is_empty()
						}
						['r', 'd', s @ ..] => {
							super::push(out, 24036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 4990);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13419);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24037);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13420);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13421);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13422);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nie.
pub(crate) fn n1158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1156);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4991);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13423);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 24039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 13424);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13425);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 38773);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 38774);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', s @ ..] => {
									super::push(out, 32822);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 32823);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13426);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13427);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13428);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4992);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13430);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 24040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 13431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24041);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 24042);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 24043);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 24044);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 24045);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 24046);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 32824);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 32825);
									s.is_empty()
								}
								['r', 'o', s @ ..] => {
									super::push(out, 32826);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 32827);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 32828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 't', 'u', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 32829);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 32830);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'n', 'a', 'n', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 24047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 13432);
					match s {
						[] => true,
						['t', 's', 'a', 'g', s @ ..] => {
							super::push(out, 24048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13433);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 24049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13434);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13435);
					match s {
						[] => true,
						['h', 'p', 'a', s @ ..] => {
							super::push(out, 24050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4993);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 24051);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 32831);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 32832);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32833);
									match s {
										[] => true,
										['t', 't', 'a', s @ ..] => {
											super::push(out, 38775);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 32834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 24052);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 24053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 4994);
			s.is_empty()
		}
		['m', 'o', 's', 'n', s @ ..] => {
			super::push(out, 4995);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4996);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13436);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24054);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24055);
							s.is_empty()
						}
						['o', 'k', s @ ..] => {
							super::push(out, 24056);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 24057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24058);
							s.is_empty()
						}
						['o', 'm', 'a', s @ ..] => {
							super::push(out, 24059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 13437);
					match s {
						[] => true,
						['p', 'o', s @ ..] => {
							super::push(out, 24060);
							match s {
								[] => true,
								['k', 'a', s @ ..] => {
									super::push(out, 32835);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 32836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13438);
					match s {
						[] => true,
						['o', 'k', 'p', s @ ..] => {
							super::push(out, 24061);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 32837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13439);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24062);
							s.is_empty()
						}
						['r', 'a', 'h', s @ ..] => {
							match s {
								[] => true,
								['l', 'i', 'h', 'p', 'b', s @ ..] => {
									super::push(out, 32838);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 32839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 24063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13440);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13441);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 24064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24065);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 4997);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 4998);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13444);
					match s {
						[] => true,
						['o', 'h', s @ ..] => {
							super::push(out, 24066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 13445);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 4999);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pie.
pub(crate) fn n1159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1157);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5000);
			match s {
				[] => true,
				['r', 'e', 'h', 't', 'o', 'g', s @ ..] => {
					super::push(out, 13446);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5001);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24068);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24069);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'e', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 42205);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 42206);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 38776);
											s.is_empty()
										}
										['h', 'c', 'n', 'o', 'r', 'b', s @ ..] => {
											match s {
												[] => true,
												['o', 'e', 'h', 'c', 'a', 'r', 't', 'o', s @ ..] => {
													super::push(out, 42207);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 42208);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 42209);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 38777);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 38778);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 38779);
											match s {
												[] => true,
												['k', 'i', 'm', 'l', s @ ..] => {
													super::push(out, 42210);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 38780);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 38781);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13447);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13448);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13449);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 24070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13450);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'm', 'm', s @ ..] => {
									super::push(out, 32840);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 32841);
									s.is_empty()
								}
								['o', 's', 'i', 'n', s @ ..] => {
									super::push(out, 32842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13451);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 24071);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32843);
									s.is_empty()
								}
								['n', 'e', 'i', s @ ..] => {
									super::push(out, 32844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'b', s @ ..] => {
					super::push(out, 13452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5002);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5003);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5004);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 13454);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5005);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13455);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 24072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rie.
pub(crate) fn n1160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1158);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5006);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13456);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13457);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 24073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5007);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5008);
			match s {
				[] => true,
				['n', 'o', s @ ..] => {
					super::push(out, 13459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5009);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13460);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13461);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24075);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32845);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 38782);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 42211);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 38783);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 32846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'a', s @ ..] => {
					super::push(out, 13462);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13463);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13464);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13465);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24076);
							match s {
								[] => true,
								['m', 'i', s @ ..] => {
									super::push(out, 32847);
									s.is_empty()
								}
								['r', 'i', 'p', 't', s @ ..] => {
									super::push(out, 32848);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 24077);
							s.is_empty()
						}
						['s', 'y', s @ ..] => {
							super::push(out, 24078);
							match s {
								[] => true,
								['h', '-', s @ ..] => {
									super::push(out, 32849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 24079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13466);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5010);
			match s {
				[] => true,
				['e', 'h', 't', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 24080);
							s.is_empty()
						}
						['d', 'l', 'e', 'f', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 32850);
									s.is_empty()
								}
								['n', 'e', 'd', s @ ..] => {
									super::push(out, 32851);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 32852);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 24081);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 24082);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 24083);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 24084);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 24085);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32853);
									s.is_empty()
								}
								['ö', s @ ..] => {
									super::push(out, 32854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 32855);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 32856);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 38784);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 38785);
											s.is_empty()
										}
										['g', 'n', 'e', s @ ..] => {
											super::push(out, 38786);
											s.is_empty()
										}
										['h', s @ ..] => {
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 42212);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 42213);
													s.is_empty()
												}
												_ => false,
											}
										}
										['k', s @ ..] => {
											super::push(out, 38787);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 38788);
											s.is_empty()
										}
										['p', 'p', s @ ..] => {
											super::push(out, 38789);
											s.is_empty()
										}
										['t', 'a', s @ ..] => {
											super::push(out, 38790);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 32857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 24086);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24087);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32858);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 32859);
									s.is_empty()
								}
								['n', 'o', 'i', 't', s @ ..] => super::t12::n34544(s, out),
								['t', 'ä', 't', 'i', 't', s @ ..] => super::t12::n34545(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => super::t12::n24919(s, out),
						['z', 'n', s @ ..] => super::t12::n24920(s, out),
						_ => false,
					}
				}
				['g', s @ ..] => super::t12::n13689(s, out),
				['h', s @ ..] => super::t12::n13690(s, out),
				['m', s @ ..] => super::t12::n13691(s, out),
				['p', s @ ..] => super::t12::n13692(s, out),
				['t', 's', 'i', 'h', s @ ..] => super::t12::n13693(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t12::n5046(s, out),
		['t', s @ ..] => super::t12::n5047(s, out),
		['u', s @ ..] => super::t12::n5048(s, out),
		['y', s @ ..] => super::t12::n5049(s, out),
		['ä', s @ ..] => super::t12::n5050(s, out),
		_ => false,
	}
}

/// Words ending -sie.
pub(crate) fn n1161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1159);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5016);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5017);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13479);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5018);
			match s {
				[] => true,
				['h', 't', 's', 'ä', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24097);
							s.is_empty()
						}
						['n', 'i', s @ ..] => {
							super::push(out, 24098);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 24099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'm', 'o', s @ ..] => {
					super::push(out, 13481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5019);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 13482);
					match s {
						[] => true,
						['i', 'd', 'e', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 24100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5020);
			match s {
				[] => true,
				['n', 'g', 'a', 'p', s @ ..] => {
					super::push(out, 13483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5021);
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 24101);
							s.is_empty()
						}
						['i', 'p', 'e', 'n', s @ ..] => {
							super::push(out, 24102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'd', 'y', s @ ..] => {
					super::push(out, 13484);
					s.is_empty()
				}
				['o', 't', 'o', s @ ..] => {
					super::push(out, 13485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5022);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13486);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13487);
					match s {
						[] => true,
						['l', 'g', 'a', s @ ..] => {
							super::push(out, 24103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5023);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5024);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13490);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tie.
pub(crate) fn n1162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5025);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13491);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24104);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 24105);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 32878);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 32879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'k', 'o', s @ ..] => {
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							super::push(out, 24106);
							s.is_empty()
						}
						['m', 'e', 'd', 't', s @ ..] => {
							super::push(out, 24107);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 32880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'i', 'r', 'a', 's', s @ ..] => {
							super::push(out, 24108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5026);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 5027);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5028);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13492);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 24109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5029);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5030);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 24110);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 38802);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 38803);
											s.is_empty()
										}
										['r', 'i', 'e', s @ ..] => {
											super::push(out, 38804);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 24111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13494);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13495);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5031);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13496);
					s.is_empty()
				}
				['i', 'd', 'i', 'e', s @ ..] => {
					super::push(out, 13497);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13498);
					match s {
						[] => true,
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 24113);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5032);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5033);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24114);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 32881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 24115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5034);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13499);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 24116);
							s.is_empty()
						}
						['n', 'y', 'd', '-', s @ ..] => {
							super::push(out, 24117);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 13500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5035);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13501);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uie.
pub(crate) fn n1163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1161);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5036);
			match s {
				[] => true,
				['p', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vie.
pub(crate) fn n1164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1162);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5037);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5038);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24119);
							s.is_empty()
						}
						['d', 'a', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 24120);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 5039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wie.
pub(crate) fn n1165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1163);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5040);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xie.
pub(crate) fn n1166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1164);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'a', 'g', s @ ..] => {
					match s {
						[] => true,
						['-', 'a', s @ ..] => {
							super::push(out, 24122);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 24123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13503);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24124);
							s.is_empty()
						}
						['p', 'o', 't', s @ ..] => {
							super::push(out, 24125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5041);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5042);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13504);
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					super::push(out, 13505);
					s.is_empty()
				}
				['m', 'a', s @ ..] => {
					super::push(out, 13506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5043);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zie.
pub(crate) fn n1167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1165);
	match s {
		[] => true,
		['a', 'm', s @ ..] => {
			super::push(out, 5044);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 5045);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -anschnecke.
pub(crate) fn n24973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24139);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 32885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -recke.
pub(crate) fn n13774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13542);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -icke.
pub(crate) fn n5109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5073);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 13543);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 13544);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 13545);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ocke.
pub(crate) fn n5110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5074);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 13546);
			s.is_empty()
		}
		['l', 'g', 'e', s @ ..] => {
			super::push(out, 13547);
			match s {
				[] => true,
				['z', 't', s @ ..] => {
					super::push(out, 24142);
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 32886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucke.
pub(crate) fn n5111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5075);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 13548);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 24143);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äcke.
pub(crate) fn n5112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5076);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 13549);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ücke.
pub(crate) fn n5113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5077);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 24144);
					s.is_empty()
				}
				['n', 'h', s @ ..] => {
					super::push(out, 24145);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 32887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 13550);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 24146);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 32888);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 38805);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'b', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 24147);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['a', s @ ..] => {
					super::push(out, 24148);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 32890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 24149);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 24150);
					match s {
						[] => true,
						['t', 'l', s @ ..] => {
							super::push(out, 32891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'z', s @ ..] => {
					super::push(out, 24151);
					s.is_empty()
				}
				['s', 'l', 'r', s @ ..] => {
					super::push(out, 24152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 13551);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eke.
pub(crate) fn n1176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1174);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5078);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5079);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5080);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5081);
			match s {
				[] => true,
				['a', 'p', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 13552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5082);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13553);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13554);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fke.
pub(crate) fn n1177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1175);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5083);
			s.is_empty()
		}
		['o', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ike.
pub(crate) fn n1179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1177);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5085);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5086);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5087);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5088);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5089);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13556);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13558);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5091);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5092);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24153);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5093);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13561);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13562);
					match s {
						[] => true,
						['e', 'm', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 24155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5094);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kke.
pub(crate) fn n1180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1178);
	s.is_empty()
}

/// Words ending -lke.
pub(crate) fn n1181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1179);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5095);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13564);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13565);
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 24156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5096);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5097);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5098);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13567);
					match s {
						[] => true,
						['c', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5099);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13569);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nke.
pub(crate) fn n1182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5100);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13570);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 13571);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24158);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 38806);
											match s {
												[] => true,
												['c', 'ü', 'r', '\u{1}', s @ ..] => {
													super::push(out, 42221);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'g', s @ ..] => {
											super::push(out, 38807);
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['a', 'w', s @ ..] => super::t12::n46808(s, out),
														['i', s @ ..] => super::t12::n46809(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', 'h', s @ ..] => {
											super::push(out, 38808);
											match s {
												[] => true,
												['c', 'u', 'l', 'f', '\u{1}', s @ ..] => {
													super::push(out, 42222);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13572);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13573);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13574);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 24159);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 24160);
							match s {
								[] => true,
								['c', 's', '-', s @ ..] => {
									super::push(out, 32892);
									match s {
										[] => true,
										['n', 'r', s @ ..] => {
											super::push(out, 38809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 24161);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 24162);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24163);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 32893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 13575);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5101);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13576);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13577);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 13578);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 24165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5102);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24166);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 24167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13580);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 24168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13581);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13582);
					match s {
						[] => true,
						['t', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 24169);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5103);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13583);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13584);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13585);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24170);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 24171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oke.
pub(crate) fn n1183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1181);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5105);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13587);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 24172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5106);
			s.is_empty()
		}
		['j', '\u{1}', s @ ..] => {
			super::push(out, 5107);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 5108);
			match s {
				[] => true,
				['s', 'u', '\u{1}', s @ ..] => {
					super::push(out, 13588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5109);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13589);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pke.
pub(crate) fn n1184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 5110);
			match s {
				[] => true,
				['e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 13590);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rke.
pub(crate) fn n1185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5111);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 13591);
					match s {
						[] => true,
						['s', 'b', 'l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 24173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5112);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 13592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5113);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5114);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13593);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13594);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 24174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5115);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 24175);
							s.is_empty()
						}
						['l', 'l', 'a', s @ ..] => {
							super::push(out, 24176);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24177);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									super::push(out, 32894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									super::push(out, 32895);
									s.is_empty()
								}
								['n', 'e', 'b', s @ ..] => {
									super::push(out, 32896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', 't', '\u{1}', s @ ..] => {
			super::push(out, 5116);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 5117);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 13595);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ske.
pub(crate) fn n1186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1184);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13596);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13597);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24178);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32897);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'r', 'o', s @ ..] => {
			super::push(out, 5118);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 13598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5119);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 13599);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tke.
pub(crate) fn n1187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1185);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 5120);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uke.
pub(crate) fn n1188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1186);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5121);
			s.is_empty()
		}
		['a', 'b', s @ ..] => {
			super::push(out, 5122);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5123);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5124);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5125);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 13602);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 24179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yke.
pub(crate) fn n1189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1187);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5126);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zke.
pub(crate) fn n1190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1188);
	match s {
		[] => true,
		['t', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öke.
pub(crate) fn n1191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1189);
	match s {
		[] => true,
		['i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5128);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5129);
			s.is_empty()
		}
		['t', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uviale.
pub(crate) fn n25027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24192);
	s.is_empty()
}

/// Words ending -ziale.
pub(crate) fn n13859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13623);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 24193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kale.
pub(crate) fn n5176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5139);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 13624);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 24194);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 24195);
					match s {
						[] => true,
						['i', 't', 'n', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 32904);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 13625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -male.
pub(crate) fn n5177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5140);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13626);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 24196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'z', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 13627);
			s.is_empty()
		}
		['r', 'o', 'n', 'b', s @ ..] => {
			super::push(out, 13628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nale.
pub(crate) fn n5178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5141);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 13629);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 24197);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 32905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 24198);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 32906);
							s.is_empty()
						}
						['g', 'a', 'l', 'h', 'c', 's', 'n', s @ ..] => {
							super::push(out, 32907);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 32908);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 32909);
							match s {
								[] => true,
								['u', 'c', '\u{1}', s @ ..] => {
									super::push(out, 38812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 24199);
					match s {
						[] => true,
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 32910);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 24200);
					match s {
						[] => true,
						['o', 'n', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 32911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13630);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13631);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 24202);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 24203);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 24204);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 24205);
					match s {
						[] => true,
						['t', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 38813);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 38814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 24206);
					match s {
						[] => true,
						['r', 'e', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							super::push(out, 32912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 13632);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 24207);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rale.
pub(crate) fn n5179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5142);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 13633);
			match s {
				[] => true,
				['e', 'h', 't', 'a', 'k', 's', s @ ..] => {
					super::push(out, 24208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13634);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 24209);
					match s {
						[] => true,
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 32913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 24210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13635);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13636);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 24211);
					match s {
						[] => true,
						['m', 'e', 't', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 32914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 24212);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 32915);
							s.is_empty()
						}
						['k', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 32916);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 32917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13637);
			match s {
				[] => true,
				['n', 'e', 'z', 's', 'g', 'u', s @ ..] => {
					super::push(out, 24213);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 24214);
					match s {
						[] => true,
						['i', 'g', 'a', 'm', '-', s @ ..] => {
							super::push(out, 32918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sale.
pub(crate) fn n5180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5143);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 13638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tale.
pub(crate) fn n5181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5144);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 13639);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 24215);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 24216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13640);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24217);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 32919);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 32920);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 24218);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uale.
pub(crate) fn n5182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5145);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 13641);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 13642);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vale.
pub(crate) fn n5183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5146);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 13643);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 24219);
					match s {
						[] => true,
						['c', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 32921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 24220);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xale.
pub(crate) fn n5184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5147);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 13644);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zale.
pub(crate) fn n5185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5148);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 13645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ble.
pub(crate) fn n1193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1191);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5149);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13646);
					match s {
						[] => true,
						['r', 'a', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32922);
									s.is_empty()
								}
								['n', 'e', 's', 's', 'a', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 32923);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'b', 'm', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 38815);
											s.is_empty()
										}
										['o', 't', 'a', s @ ..] => {
											match s {
												[] => true,
												['i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
													super::push(out, 42223);
													s.is_empty()
												}
												['r', 'e', 'd', 'o', 'm', '\u{1}', s @ ..] => {
													super::push(out, 42224);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 32924);
									s.is_empty()
								}
								['t', 't', 'i', 'r', 'd', '\u{1}', s @ ..] => {
									super::push(out, 32925);
									s.is_empty()
								}
								['z', 'n', 'a', 't', 's', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 32926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13647);
					match s {
						[] => true,
						['a', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 24221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13648);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 24222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5150);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13649);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5151);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5152);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13651);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13652);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 24223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5153);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13653);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 13654);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 24225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cle.
pub(crate) fn n1194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1192);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5154);
			match s {
				[] => true,
				['s', 'p', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13655);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dle.
pub(crate) fn n1195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1193);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5155);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5156);
			s.is_empty()
		}
		['n', 'ä', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5157);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5158);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 5159);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 13656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 5160);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ele.
pub(crate) fn n1196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1194);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5161);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13657);
					match s {
						[] => true,
						['d', 'n', '-', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 32927);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 32928);
									match s {
										[] => true,
										['a', 'a', 'v', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 38816);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13658);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 13659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5162);
			match s {
				[] => true,
				['u', 'h', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 13660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5163);
			match s {
				[] => true,
				['s', 'n', s @ ..] => {
					super::push(out, 13661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5164);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13662);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13663);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 32929);
									s.is_empty()
								}
								['r', 'e', 'm', s @ ..] => {
									super::push(out, 32930);
									s.is_empty()
								}
								['t', 's', s @ ..] => {
									super::push(out, 32931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5165);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5166);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5167);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5168);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13665);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5169);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fle.
pub(crate) fn n1197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1195);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 5171);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5172);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5173);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gle.
pub(crate) fn n1198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1196);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5174);
			match s {
				[] => true,
				['e', 'b', s @ ..] => {
					super::push(out, 13668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5175);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5176);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13669);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 24227);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13670);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13671);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hle.
pub(crate) fn n1199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1197);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 5177);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 13672);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 13673);
					match s {
						[] => true,
						['n', 'e', 'h', 'c', 'o', s @ ..] => {
							super::push(out, 24229);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5178);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13675);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 5179);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 5180);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 13676);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13677);
					match s {
						[] => true,
						['r', 'e', 'b', s @ ..] => {
							super::push(out, 24232);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 32933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ile.
pub(crate) fn n1200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1198);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5181);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 24233);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 32934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 24234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5182);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5183);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13679);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13680);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 13681);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13682);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13683);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13684);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 24235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13685);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 13686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5184);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13687);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5185);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5186);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 13690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5187);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13691);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 24237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13692);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5188);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5189);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5190);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5191);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13696);
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32935);
									s.is_empty()
								}
								['-', 'e', 's', 'i', 'u', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 32936);
									s.is_empty()
								}
								['e', 's', 'i', 'u', 'r', 'c', '\u{1}', s @ ..] => {
									super::push(out, 32937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kle.
pub(crate) fn n1201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1199);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5192);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13697);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 13698);
					match s {
						[] => true,
						['m', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 24238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lle.
pub(crate) fn n1202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1200);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5193);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13699);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13700);
					match s {
						[] => true,
						['e', 'l', 'l', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24240);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5194);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13701);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24241);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 24242);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 32938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 13702);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 24243);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13703);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 24245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13704);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24246);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 24247);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 38817);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 38818);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 24248);
							match s {
								[] => true,
								['h', 'c', 's', 't', '\u{1}', s @ ..] => {
									super::push(out, 32939);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 32940);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 32941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 24249);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['s', 's', 'e', 'f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 38819);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 38820);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 32942);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 24250);
							match s {
								[] => true,
								['e', 'b', 'i', 'b', '-', s @ ..] => {
									super::push(out, 32943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 24251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13705);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13706);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24252);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 32944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13707);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24253);
							match s {
								[] => true,
								['g', 's', 's', 'e', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 32945);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 24254);
							match s {
								[] => true,
								['o', 'm', 'e', 'd', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 32946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13708);
					match s {
						[] => true,
						['s', 'f', s @ ..] => {
							super::push(out, 24255);
							match s {
								[] => true,
								['ü', s @ ..] => {
									super::push(out, 32947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13709);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 24256);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 32948);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 38821);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 32949);
									match s {
										[] => true,
										['a', 'n', 'o', s @ ..] => {
											super::push(out, 38822);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 24257);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 32950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 24258);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 32951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13710);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24259);
							match s {
								[] => true,
								['e', 'k', 'n', s @ ..] => {
									super::push(out, 32952);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', s @ ..] => {
							super::push(out, 24260);
							match s {
								[] => true,
								['n', 'u', 'g', s @ ..] => {
									super::push(out, 32953);
									match s {
										[] => true,
										['i', 'd', 'n', 'ü', 'k', '\u{1}', s @ ..] => {
											super::push(out, 38823);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 13711);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t12::n5232(s, out),
		['o', s @ ..] => {
			super::push(out, 5196);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13724);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13725);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13726);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24281);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 24282);
							match s {
								[] => true,
								['n', 'o', 'k', s @ ..] => super::t12::n34664(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 13727);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24283);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 24284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t12::n5234(s, out),
		['ä', s @ ..] => super::t12::n5235(s, out),
		['ö', s @ ..] => super::t12::n5236(s, out),
		['ü', s @ ..] => super::t12::n5237(s, out),
		_ => false,
	}
}

/// Words ending -mle.
pub(crate) fn n1203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1201);
	s.is_empty()
}

/// Words ending -ole.
pub(crate) fn n1204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1202);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5201);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 13741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5202);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5203);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13743);
					s.is_empty()
				}
				['t', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 13744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5204);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 13745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5205);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5206);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13746);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13747);
					match s {
						[] => true,
						['c', 's', 't', 'u', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 24293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13748);
					match s {
						[] => true,
						['r', 't', 'e', 'm', s @ ..] => {
							match s {
								[] => true,
								['-', 'e', s @ ..] => {
									super::push(out, 32975);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 32976);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 32977);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 32978);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 32979);
									s.is_empty()
								}
								['r', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 5207);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24294);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 24295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 13749);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 24296);
							match s {
								[] => true,
								['i', 'f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 32981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5208);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ple.
pub(crate) fn n1205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1203);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5210);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5211);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5212);
			match s {
				[] => true,
				['e', 'p', 't', 'a', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 13750);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rle.
pub(crate) fn n1206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1204);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5213);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13751);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 13752);
					match s {
						[] => true,
						['n', 'i', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 24298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 13753);
					match s {
						[] => true,
						['c', 'u', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 24299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 13754);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13755);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 24301);
							match s {
								[] => true,
								['e', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 32982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 13756);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13757);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5214);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24303);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24304);
							s.is_empty()
						}
						['t', 'f', 'a', 's', 'r', s @ ..] => {
							super::push(out, 24305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sle.
pub(crate) fn n1207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1205);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5215);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5216);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 13758);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 13759);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 24306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 24307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tle.
pub(crate) fn n1208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1206);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5217);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5218);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5219);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 13760);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 13761);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 13762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 5220);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ule.
pub(crate) fn n1209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1207);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5221);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5222);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5223);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13765);
					match s {
						[] => true,
						['n', 'o', 'k', 'i', '\u{1}', s @ ..] => {
							super::push(out, 24308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5224);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13766);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 13767);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13768);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 24310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5225);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5226);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13769);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 13770);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 13771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5227);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13772);
					match s {
						[] => true,
						['l', 'e', 'd', s @ ..] => {
							super::push(out, 24312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13773);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 24313);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 32984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 24314);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 32985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wle.
pub(crate) fn n1210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1208);
	s.is_empty()
}

/// Words ending -yle.
pub(crate) fn n1211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1209);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5228);
			match s {
				[] => true,
				['a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 13774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5229);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5230);
			match s {
				[] => true,
				['a', 'g', 'r', 'o', 'k', s @ ..] => {
					super::push(out, 13775);
					match s {
						[] => true,
						['n', 'i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 24315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5231);
			match s {
				[] => true,
				['g', 'r', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 13776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5232);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13777);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24316);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 24317);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 24318);
							match s {
								[] => true,
								['m', 'u', 'j', '\u{1}', s @ ..] => {
									super::push(out, 32986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zle.
pub(crate) fn n1212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1210);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 5233);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5234);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13778);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 24319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 13779);
					match s {
						[] => true,
						['p', 's', 't', s @ ..] => {
							super::push(out, 24320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5235);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 13780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öle.
pub(crate) fn n1213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1211);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 5236);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üle.
pub(crate) fn n1214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1212);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 5238);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 5239);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13781);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ahme.
pub(crate) fn n5291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5254);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 13812);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13813);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24331);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 24332);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 32993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'u', 'a', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 32994);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 38835);
									match s {
										[] => true,
										['g', 'r', 'e', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 42230);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'g', s @ ..] => {
							super::push(out, 32995);
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 38836);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 32996);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 38837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', 'e', 'w', 'n', s @ ..] => {
					super::push(out, 24333);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 32997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 24334);
					match s {
						[] => true,
						['i', 'e', 't', 'n', s @ ..] => {
							super::push(out, 32998);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'i', 'h', s @ ..] => {
					super::push(out, 24335);
					s.is_empty()
				}
				['r', 'e', 'b', 'ü', 's', s @ ..] => {
					super::push(out, 24336);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 32999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', s @ ..] => {
					super::push(out, 24337);
					s.is_empty()
				}
				['t', 'i', s @ ..] => {
					super::push(out, 24338);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 24339);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 24340);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ime.
pub(crate) fn n1219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1217);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5256);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5257);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 24341);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 33000);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 38839);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 't', s @ ..] => {
									super::push(out, 33001);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 24342);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24343);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 24344);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 33002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 24345);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5258);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5259);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5260);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5261);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13818);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 13819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5262);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 13820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 5263);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13821);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word femme.
pub(crate) fn n25191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24347);
	s.is_empty()
}

/// Words ending -imme.
pub(crate) fn n5303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5266);
	match s {
		[] => true,
		['t', 's', 's', s @ ..] => {
			super::push(out, 13823);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24348);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 33003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -omme.
pub(crate) fn n5304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5267);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 13824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -umme.
pub(crate) fn n5305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5268);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 13825);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13826);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 13827);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 24349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ome.
pub(crate) fn n1222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1220);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5269);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5270);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5271);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5272);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rme.
pub(crate) fn n1223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1221);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5274);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13829);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 13830);
					s.is_empty()
				}
				['s', 'u', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 13831);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5275);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13832);
					match s {
						[] => true,
						['y', 'h', 'c', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 13833);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 13834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5276);
			match s {
				[] => true,
				['w', 's', s @ ..] => {
					super::push(out, 13835);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ume.
pub(crate) fn n1224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1222);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13836);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13837);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äme.
pub(crate) fn n1225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1223);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5277);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ème.
pub(crate) fn n1226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1224);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5278);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5279);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24352);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hiene.
pub(crate) fn n14103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13860);
	s.is_empty()
}

/// Words ending -miene.
pub(crate) fn n14104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13861);
	s.is_empty()
}

/// Words ending -kene.
pub(crate) fn n5337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5297);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 13862);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13863);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lene.
pub(crate) fn n5338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5298);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 13864);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13865);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 24369);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 24370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 13866);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 24371);
					match s {
						[] => true,
						['f', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 33012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 24372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mene.
pub(crate) fn n5339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5299);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 13867);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'e', 't', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 24373);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 24374);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rene.
pub(crate) fn n5340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5300);
	match s {
		[] => true,
		['h', 'p', 'o', 'z', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 13868);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13869);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13870);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sene.
pub(crate) fn n5341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5301);
	s.is_empty()
}

/// Words ending -vene.
pub(crate) fn n5342(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5302);
	s.is_empty()
}

/// Words ending -wene.
pub(crate) fn n5343(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5303);
	s.is_empty()
}

/// Words ending -zene.
pub(crate) fn n5344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5304);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13871);
			match s {
				[] => true,
				['r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 24375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 24376);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 33013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 24377);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßene.
pub(crate) fn n5345(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5305);
	match s {
		[] => true,
		['o', 't', 's', 'e', 'g', 's', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 13872);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gne.
pub(crate) fn n1229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1227);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5306);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5307);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13873);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hne.
pub(crate) fn n1230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1228);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5308);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13874);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5309);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5310);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 5311);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 13876);
					match s {
						[] => true,
						['e', 'd', 'i', 'e', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 24378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 5312);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13877);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ine.
pub(crate) fn n1231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1229);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5313);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13878);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5314);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5315);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13879);
					match s {
						[] => true,
						['u', 't', 't', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 24379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 13880);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5316);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13881);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13882);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13883);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 24380);
							match s {
								[] => true,
								['a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 33014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5317);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13884);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 24381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13885);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24382);
							match s {
								[] => true,
								['d', 'a', s @ ..] => {
									super::push(out, 33015);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 38842);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 24383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5319);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13888);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13889);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5320);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13891);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 24384);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 13892);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13893);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 24385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5321);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5322);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13896);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 13897);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13898);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 24386);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24387);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 24388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 13899);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24389);
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 33016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'c', s @ ..] => {
					super::push(out, 13900);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13901);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13902);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24390);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 24391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13903);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13904);
					s.is_empty()
				}
				['y', 'k', s @ ..] => {
					super::push(out, 13905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5323);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13906);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 24392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13907);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5324);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5325);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 13909);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5326);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13910);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13911);
					match s {
						[] => true,
						['e', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 24395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5327);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13912);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 33018);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 38843);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 38844);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5328);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13913);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 33019);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 33020);
									match s {
										[] => true,
										['s', 'a', '\u{1}', s @ ..] => {
											super::push(out, 38845);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 24396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24397);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5329);
			match s {
				[] => true,
				['a', 'l', 'e', s @ ..] => {
					super::push(out, 13914);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13915);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 24399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13916);
					s.is_empty()
				}
				['u', 'o', 'r', 's', s @ ..] => {
					super::push(out, 13917);
					match s {
						[] => true,
						['g', 'n', 'u', 't', s @ ..] => {
							super::push(out, 24400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5330);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13918);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 24401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5331);
			match s {
				[] => true,
				['a', 'l', 'k', s @ ..] => {
					super::push(out, 13920);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 24402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 13921);
					match s {
						[] => true,
						['r', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 24403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5332);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 13922);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 24404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 13923);
					match s {
						[] => true,
						['h', 't', 'i', 'p', 'o', 'l', 'a', 'r', 't', 's', 'u', 'a', '\u{1}', s @ ..] => super::t12::n25251(s, out),
						_ => false,
					}
				}
				['k', s @ ..] => super::t12::n14170(s, out),
				['n', s @ ..] => super::t12::n14171(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kne.
pub(crate) fn n1232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1230);
	s.is_empty()
}

/// Words ending -lne.
pub(crate) fn n1233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1231);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mne.
pub(crate) fn n1234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1232);
	match s {
		[] => true,
		['y', 'h', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13926);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13927);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nne.
pub(crate) fn n1235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1233);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5334);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 13928);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13929);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24407);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24408);
							match s {
								[] => true,
								['o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 33021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13930);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 13931);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 13932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5335);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13933);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13934);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 24410);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 24411);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 33022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 24412);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 24413);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 24414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13935);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13936);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13937);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13938);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 13939);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24417);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 33023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5336);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13940);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13941);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13942);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13943);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24418);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 33024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5337);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13944);
					match s {
						[] => true,
						['a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 24419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13945);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 13946);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13947);
					match s {
						[] => true,
						['l', 'e', 'p', s @ ..] => {
							super::push(out, 24420);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 33025);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 33026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33027);
									s.is_empty()
								}
								['e', 'b', 'o', s @ ..] => {
									super::push(out, 33028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38846);
											s.is_empty()
										}
										['n', 'i', 'l', 'l', s @ ..] => {
											super::push(out, 38847);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 33029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13948);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24421);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 33030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ç', s @ ..] => {
					super::push(out, 13949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5338);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 13950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5339);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13951);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -one.
pub(crate) fn n1236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1234);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 5340);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5341);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 13953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5342);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5343);
			match s {
				[] => true,
				['e', 'l', 'y', 't', 'o', 'k', 'o', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 13955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5344);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5345);
			match s {
				[] => true,
				['c', 's', 'e', 'h', 'c', 's', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 13956);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13957);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24422);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 24423);
							match s {
								[] => true,
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 33031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 24424);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 24425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13958);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13959);
					match s {
						[] => true,
						['h', 'c', 'o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 24427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5346);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13960);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 13961);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 13962);
					match s {
						[] => true,
						['g', 'e', 'p', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 24428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5347);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5348);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13964);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 13965);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 24429);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 13966);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 13967);
					match s {
						[] => true,
						['g', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 24430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13968);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 24431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5349);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13969);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5350);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13970);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13971);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24433);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5351);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 13972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5352);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13973);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13974);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 24434);
							match s {
								[] => true,
								['i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 33032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 13975);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 24435);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 33033);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13976);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 24436);
							match s {
								[] => true,
								['e', 'n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 33034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5353);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5354);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 13978);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 24437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'e', s @ ..] => {
					super::push(out, 13979);
					match s {
						[] => true,
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5355);
			match s {
				[] => true,
				['t', 'k', 'i', 'h', 'p', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 13980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5356);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13981);
					s.is_empty()
				}
				['d', 'n', 'e', 'i', s @ ..] => {
					super::push(out, 13982);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 24439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13983);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 13984);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13985);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 13986);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13987);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13988);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24441);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24442);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 33035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rne.
pub(crate) fn n1237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1235);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5357);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13989);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24443);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 24444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 13990);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 24445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 13991);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 13992);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24446);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 33036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5358);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5359);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13994);
					match s {
						[] => true,
						['e', 'g', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 24447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5360);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 13995);
					match s {
						[] => true,
						['c', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 24448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -une.
pub(crate) fn n1238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1236);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5361);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13996);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13997);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 24449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13998);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5362);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5363);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5364);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14001);
					match s {
						[] => true,
						['r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 24450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5365);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5366);
			match s {
				[] => true,
				['h', 'c', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 14002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5367);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14003);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yne.
pub(crate) fn n1239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1237);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 5368);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äne.
pub(crate) fn n1240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1238);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5369);
			s.is_empty()
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14004);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14005);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5370);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 14006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5371);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5372);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öne.
pub(crate) fn n1241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1239);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5373);
			match s {
				[] => true,
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 14007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üne.
pub(crate) fn n1242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1240);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5374);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5375);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 5376);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5377);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uppe.
pub(crate) fn n5451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5409);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 14028);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 14029);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 33041);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 38848);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 33042);
							match s {
								[] => true,
								['f', 'o', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 42231);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 42232);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 33043);
							match s {
								[] => true,
								['c', 'a', 'r', 'p', 's', '-', s @ ..] => {
									super::push(out, 38849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'n', s @ ..] => {
							super::push(out, 33044);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'k', s @ ..] => {
									super::push(out, 38850);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 42233);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'a', s @ ..] => {
									super::push(out, 38851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 33045);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42234);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 42235);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38852);
									s.is_empty()
								}
								['e', 's', 'i', s @ ..] => {
									super::push(out, 38853);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'f', 'p', s @ ..] => {
									super::push(out, 38854);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 42236);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'b', s @ ..] => {
									super::push(out, 38855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 33046);
							s.is_empty()
						}
						['t', 'l', s @ ..] => {
							super::push(out, 33047);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 38856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 24465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 14030);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 24466);
					match s {
						[] => true,
						['m', 'è', s @ ..] => {
							super::push(out, 33048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 24467);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33049);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 33050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hupe.
pub(crate) fn n5453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5411);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 14032);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 24469);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jupe.
pub(crate) fn n5454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5412);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14033);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kupe.
pub(crate) fn n5455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5413);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 14034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nupe.
pub(crate) fn n5457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5415);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 14035);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oupe.
pub(crate) fn n5458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5416);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 14036);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24470);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pupe.
pub(crate) fn n5459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5417);
	s.is_empty()
}

/// Words ending -ype.
pub(crate) fn n1257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1255);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5418);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5419);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14037);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zpe.
pub(crate) fn n1258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1256);
	s.is_empty()
}

/// Words ending -êpe.
pub(crate) fn n1259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1257);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 5420);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 14038);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdre.
pub(crate) fn n5476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5434);
	s.is_empty()
}

/// Words ending -ere.
pub(crate) fn n1265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1263);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5435);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14067);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14068);
					match s {
						[] => true,
						['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24498);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 24499);
							match s {
								[] => true,
								['l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 33056);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14069);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 24501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5436);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14070);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14071);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5437);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5438);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5439);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14073);
					match s {
						[] => true,
						['e', 'g', 'm', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 24503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14074);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24504);
							s.is_empty()
						}
						['g', 'a', 's', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 24505);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 24506);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 33057);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 33058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14075);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 24507);
							match s {
								[] => true,
								['c', 's', 'e', s @ ..] => {
									super::push(out, 33059);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 38858);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14076);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 14077);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14078);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14079);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 24509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5440);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5441);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5442);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14082);
					match s {
						[] => true,
						['a', 'o', s @ ..] => {
							super::push(out, 24510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5443);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5444);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 14084);
					match s {
						[] => true,
						['t', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 24511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5445);
			match s {
				[] => true,
				['q', s @ ..] => {
					super::push(out, 14085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5446);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 5447);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 14086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fre.
pub(crate) fn n1266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1264);
	s.is_empty()
}

/// Words ending -hre.
pub(crate) fn n1267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1265);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5448);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14087);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 14088);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 14089);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24512);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24513);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 38859);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 38860);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5449);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 14090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5450);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14091);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24514);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 24515);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 24516);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33060);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 38861);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 38862);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 38863);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'r', s @ ..] => {
							super::push(out, 24517);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24518);
							match s {
								[] => true,
								['g', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['m', 'ö', s @ ..] => {
											super::push(out, 38864);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 38865);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 33061);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 33062);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 33063);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 33064);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 33065);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 38866);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 33066);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 33067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5451);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 5452);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14093);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 5453);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ire.
pub(crate) fn n1268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1266);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5454);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5455);
			match s {
				[] => true,
				['d', 'i', 'g', 'i', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 14095);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 14096);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 24519);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 24520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14097);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14098);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14099);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 14100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5456);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5457);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 14102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5458);
			match s {
				[] => true,
				['a', 'k', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 14103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5459);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 5460);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 14104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5461);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14105);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5462);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5463);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5464);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nre.
pub(crate) fn n1269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1267);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5466);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ore.
pub(crate) fn n1270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1268);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5467);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14109);
					match s {
						[] => true,
						['a', 'e', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 24521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14110);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5468);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5470);
			match s {
				[] => true,
				['c', 'o', s @ ..] => {
					super::push(out, 14114);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 24522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 14115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5471);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5472);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14117);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14118);
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 24523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5473);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 14119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5474);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5475);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5476);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14121);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5477);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14123);
					match s {
						[] => true,
						['d', 'e', 'v', 'v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 5478);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14125);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24525);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 33068);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 33069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 24526);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rre.
pub(crate) fn n1271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1269);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5479);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14127);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14128);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24528);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5480);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 14129);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14130);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 14131);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5481);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14132);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 14133);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 24532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5482);
			match s {
				[] => true,
				['n', 'k', s @ ..] => {
					super::push(out, 14135);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 24533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5483);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14136);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tre.
pub(crate) fn n1272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1270);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 5484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ure.
pub(crate) fn n1273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1271);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5485);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5486);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5487);
			match s {
				[] => true,
				['l', 'p', 'o', 't', s @ ..] => {
					super::push(out, 14139);
					match s {
						[] => true,
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 24534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5488);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5489);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5490);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5491);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 5492);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14141);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5493);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5494);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14143);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 24535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5495);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5496);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5497);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14145);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14146);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14147);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 24536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5498);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 5499);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							super::push(out, 24537);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 24538);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['e', 'f', s @ ..] => {
									super::push(out, 33070);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 33071);
									s.is_empty()
								}
								['o', 'z', 'n', 'e', 'b', 'o', s @ ..] => {
									super::push(out, 33072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'f', 'o', 't', 's', 'r', 'e', s @ ..] => {
							match s {
								[] => true,
								['s', 's', 'a', 'w', 'n', s @ ..] => {
									super::push(out, 33073);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 33074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', 's', 's', 'e', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 38867);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 38868);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 33075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 24539);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 24540);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 24541);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 33076);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 38869);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 38870);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', 'r', s @ ..] => {
									super::push(out, 33077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 24542);
							s.is_empty()
						}
						['n', s @ ..] => super::t12::n25399(s, out),
						['o', s @ ..] => {
							super::push(out, 24543);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 'i', s @ ..] => {
									super::push(out, 33079);
									s.is_empty()
								}
								['e', s @ ..] => super::t12::n34787(s, out),
								['o', 'b', s @ ..] => {
									super::push(out, 33081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t12::n25402(s, out),
						['y', s @ ..] => {
							super::push(out, 24545);
							s.is_empty()
						}
						['z', s @ ..] => super::t12::n25404(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vre.
pub(crate) fn n1274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1272);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5500);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5501);
			match s {
				[] => true,
				['e', 'o', '\u{1}', s @ ..] => {
					super::push(out, 14148);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14149);
					s.is_empty()
				}
				['œ', s @ ..] => {
					super::push(out, 14150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äre.
pub(crate) fn n1275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1273);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 5502);
			match s {
				[] => true,
				['f', 'a', '-', s @ ..] => {
					super::push(out, 14151);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5503);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14152);
					s.is_empty()
				}
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['i', 'm', 'e', 'h', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 33085);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 33086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 33087);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 33088);
									s.is_empty()
								}
								['i', 'b', s @ ..] => {
									super::push(out, 33089);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 38892);
											s.is_empty()
										}
										['t', 'a', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 42247);
													s.is_empty()
												}
												['f', s @ ..] => {
													super::push(out, 42248);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 42249);
													s.is_empty()
												}
												['n', 'r', s @ ..] => {
													super::push(out, 42250);
													s.is_empty()
												}
												['s', 'a', s @ ..] => {
													super::push(out, 42251);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 38893);
											s.is_empty()
										}
										['o', 'i', s @ ..] => {
											super::push(out, 38894);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 33090);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 33091);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 33092);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38895);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 38896);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 38897);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 24547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5504);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5505);
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 14153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5506);
			match s {
				[] => true,
				['h', 'c', 's', 't', 'u', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14154);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ère.
pub(crate) fn n1276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1274);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5507);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['n', 'o', 's', 'n', s @ ..] => {
							super::push(out, 24548);
							match s {
								[] => true,
								['a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 33093);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 24549);
							match s {
								[] => true,
								['s', 'n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 33094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14155);
					match s {
						[] => true,
						['u', 'o', 'c', 's', 'u', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 24550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', s @ ..] => {
					super::push(out, 14156);
					match s {
						[] => true,
						['o', 'j', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 24551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5508);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5509);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 14157);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öre.
pub(crate) fn n1277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1275);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5510);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5511);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üre.
pub(crate) fn n1278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1276);
	s.is_empty()
}

/// Words ending -hese.
pub(crate) fn n5568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5526);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 14182);
			match s {
				[] => true,
				['e', 't', 'a', 'k', 'n', s @ ..] => {
					super::push(out, 24559);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 24560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 14183);
			match s {
				[] => true,
				['n', 'y', 's', 'o', s @ ..] => {
					super::push(out, 24561);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 33099);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 33100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 33101);
							match s {
								[] => true,
								['y', 'h', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 42252);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 42253);
											s.is_empty()
										}
										['s', 'n', s @ ..] => {
											super::push(out, 42254);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 42255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 33102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					super::push(out, 24562);
					s.is_empty()
				}
				['s', 'g', 'e', s @ ..] => {
					super::push(out, 24563);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iese.
pub(crate) fn n5569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5527);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 14184);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 14185);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 14186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24564);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 24565);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 14187);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 14188);
			match s {
				[] => true,
				['z', 'l', s @ ..] => {
					super::push(out, 24566);
					match s {
						[] => true,
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 33103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 14189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kese.
pub(crate) fn n5570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5528);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 14190);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 14191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lese.
pub(crate) fn n5571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5529);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14192);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 14193);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 14194);
			match s {
				[] => true,
				['e', 'b', 'a', s @ ..] => {
					super::push(out, 24567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 14195);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 24568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 14196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mese.
pub(crate) fn n5572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5530);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14197);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 14198);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nese.
pub(crate) fn n5573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5531);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14199);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 24569);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 33104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 14200);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 33105);
							s.is_empty()
						}
						['o', 'i', s @ ..] => {
							super::push(out, 33106);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 42256);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 42257);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 14201);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 14202);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 24570);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 24571);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 33107);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 38900);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 42258);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 42259);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 24572);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 24573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 14203);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 14204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oese.
pub(crate) fn n5574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5532);
	match s {
		[] => true,
		['p', 'o', 't', 'a', 'm', 'ä', s @ ..] => {
			super::push(out, 14205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rese.
pub(crate) fn n5575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5533);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 14206);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 14207);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 24574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', 'p', 'o', s @ ..] => {
					super::push(out, 24575);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 24576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 14208);
			match s {
				[] => true,
				['h', 'p', 'o', s @ ..] => {
					super::push(out, 24577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 14209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tese.
pub(crate) fn n5576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5534);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 14210);
			s.is_empty()
		}
		['n', 'o', 'm', 'e', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 14211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uese.
pub(crate) fn n5577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5535);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 14212);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vese.
pub(crate) fn n5578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5536);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 14213);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wese.
pub(crate) fn n5579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5537);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 14214);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xese.
pub(crate) fn n5580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5538);
	s.is_empty()
}

/// Words ending -zese.
pub(crate) fn n5581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5539);
	s.is_empty()
}

/// Words ending -gse.
pub(crate) fn n1283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1281);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5540);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hse.
pub(crate) fn n1284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1282);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 24578);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 33108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 24579);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24580);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24581);
							match s {
								[] => true,
								['g', 'n', 'ä', 'l', '\u{1}', s @ ..] => {
									super::push(out, 33109);
									s.is_empty()
								}
								['l', 'e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 38901);
											s.is_empty()
										}
										['m', 'm', 'i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 38902);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'i', 't', 'a', 't', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 33110);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 33111);
									s.is_empty()
								}
								['s', 'o', 'r', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 33112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 14215);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 14216);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14217);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 14218);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ise.
pub(crate) fn n1285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1283);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5541);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14219);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14220);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14221);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14222);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 14223);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 24583);
							match s {
								[] => true,
								['g', 'n', s @ ..] => {
									super::push(out, 33113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5542);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14224);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14225);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 24585);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 33114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14226);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24586);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 33115);
									match s {
										[] => true,
										['ä', 't', 'i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 38903);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 14227);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 33116);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 33117);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 38904);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 33118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5543);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5544);
			match s {
				[] => true,
				['c', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 14229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5545);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 14230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5546);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['-', 'o', s @ ..] => {
							super::push(out, 24587);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							super::push(out, 24588);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', s @ ..] => {
							super::push(out, 24589);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 24590);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 33120);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'i', s @ ..] => {
							super::push(out, 24591);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 33121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 24592);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 24593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5547);
			match s {
				[] => true,
				['q', 'k', 'a', 'n', s @ ..] => {
					super::push(out, 14231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kse.
pub(crate) fn n1286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1284);
	s.is_empty()
}

/// Words ending -lse.
pub(crate) fn n1287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1285);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5548);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14232);
					match s {
						[] => true,
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 24594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mse.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1286);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14233);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14234);
					match s {
						[] => true,
						['b', 's', 's', s @ ..] => {
							super::push(out, 24595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5549);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14235);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nse.
pub(crate) fn n1289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1287);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5550);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14237);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 14238);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 14239);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5551);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14241);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14242);
					match s {
						[] => true,
						['n', 'e', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 24598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14243);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ose.
pub(crate) fn n1290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1288);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5553);
			match s {
				[] => true,
				['u', 'l', 'g', 'y', s @ ..] => {
					super::push(out, 14244);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5554);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14245);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24600);
							s.is_empty()
						}
						['z', 's', s @ ..] => {
							super::push(out, 24601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5555);
			match s {
				[] => true,
				['r', 'y', 'h', 't', 'o', s @ ..] => {
					super::push(out, 14246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5556);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5557);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24602);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 24603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 14247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5558);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 14248);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5559);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14249);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14250);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14251);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14252);
					match s {
						[] => true,
						['e', 't', 'a', 'a', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 24605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14253);
					match s {
						[] => true,
						['h', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 24606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14254);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14255);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14256);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24607);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 24608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 14257);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24609);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 24610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14258);
					s.is_empty()
				}
				['s', 'o', 'r', s @ ..] => {
					super::push(out, 14259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5560);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14260);
					s.is_empty()
				}
				['p', 'y', 'h', 'o', s @ ..] => {
					super::push(out, 14261);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14262);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 24611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 14263);
					match s {
						[] => true,
						['z', 'o', s @ ..] => {
							match s {
								[] => true,
								['i', 'b', 'o', s @ ..] => {
									super::push(out, 33122);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 33123);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5561);
			match s {
				[] => true,
				['a', 'h', 'c', 'c', s @ ..] => {
					super::push(out, 14264);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 14265);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 38905);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 38906);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 33124);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 33125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 24612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'k', 's', 'o', 'e', s @ ..] => {
							super::push(out, 24613);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 24614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14266);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14267);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14268);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 24615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14269);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14270);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14271);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24616);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 33126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 24617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 14272);
					match s {
						[] => true,
						['e', 'n', 't', s @ ..] => {
							super::push(out, 24618);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 33127);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5562);
			match s {
				[] => true,
				['a', 'm', 'o', 'x', s @ ..] => {
					super::push(out, 14273);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 14274);
					s.is_empty()
				}
				['k', 'a', s @ ..] => {
					super::push(out, 14275);
					s.is_empty()
				}
				['y', 'z', 'o', s @ ..] => {
					match s {
						[] => true,
						['g', 'a', 'h', 'p', 'o', s @ ..] => {
							super::push(out, 24619);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 24620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5563);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24621);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 5564);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pse.
pub(crate) fn n1291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1289);
	match s {
		[] => true,
		['a', 'j', s @ ..] => {
			super::push(out, 5565);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5566);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14278);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 14279);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 5567);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 14280);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rse.
pub(crate) fn n1292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1290);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5568);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 14281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5569);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14282);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24623);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24624);
							match s {
								[] => true,
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 33128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 24625);
							match s {
								[] => true,
								['r', 't', 'n', 'o', 'k', '-', s @ ..] => {
									super::push(out, 33129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5570);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 14284);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 14285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 5571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sse.
pub(crate) fn n1293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1291);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5572);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14286);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24626);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 24627);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 14287);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24628);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 33130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 24629);
							match s {
								[] => true,
								['k', 'c', 'ü', 'r', '\u{1}', s @ ..] => {
									super::push(out, 33131);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 33132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 24630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14288);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24631);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 33133);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 24632);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33134);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 33135);
									match s {
										[] => true,
										['v', s @ ..] => {
											super::push(out, 38907);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14289);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 33136);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 38908);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 33137);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 38909);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14290);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24633);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 33138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 24634);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 33139);
									match s {
										[] => true,
										['a', 's', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 38910);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14291);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24635);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 33140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 24636);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 24637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14292);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24638);
							match s {
								[] => true,
								['i', 'e', s @ ..] => {
									super::push(out, 33141);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38911);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5573);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14294);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 14295);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 14296);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24641);
							s.is_empty()
						}
						['l', 'e', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 24642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14298);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14299);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14300);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14301);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14302);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 24644);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 24645);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 33142);
									match s {
										[] => true,
										['n', 'i', 's', 'e', s @ ..] => {
											super::push(out, 38912);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 42260);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 24646);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 24647);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 24648);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 33143);
									s.is_empty()
								}
								['d', s @ ..] => super::t12::n34857(s, out),
								['h', 'c', s @ ..] => super::t12::n34858(s, out),
								['i', s @ ..] => super::t12::n34859(s, out),
								['l', s @ ..] => super::t12::n34860(s, out),
								['m', s @ ..] => super::t12::n34861(s, out),
								['n', 'e', 'g', s @ ..] => super::t12::n34862(s, out),
								['r', s @ ..] => super::t12::n34863(s, out),
								['s', s @ ..] => super::t12::n34864(s, out),
								['t', s @ ..] => super::t12::n34865(s, out),
								['y', s @ ..] => super::t12::n34866(s, out),
								_ => false,
							}
						}
						['t', s @ ..] => super::t12::n25516(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t12::n14562(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t12::n5620(s, out),
		['o', s @ ..] => super::t12::n5621(s, out),
		['u', s @ ..] => super::t12::n5622(s, out),
		['ä', s @ ..] => super::t12::n5623(s, out),
		_ => false,
	}
}

/// Words ending -tse.
pub(crate) fn n1294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1292);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5578);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5579);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -use.
pub(crate) fn n1295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1293);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5580);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14325);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14326);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 24667);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									super::push(out, 33152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14327);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 24668);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14328);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 14329);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24670);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 33154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5581);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14332);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 24672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5582);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14333);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5583);
			match s {
				[] => true,
				['x', s @ ..] => {
					super::push(out, 14334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5584);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5585);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 14335);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24673);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 24674);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 24675);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 24676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5586);
			match s {
				[] => true,
				['a', 'l', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 14337);
					s.is_empty()
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 14338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5587);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 14339);
					match s {
						[] => true,
						['e', 'g', 's', s @ ..] => {
							super::push(out, 24677);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14340);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yse.
pub(crate) fn n1296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1294);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', 'o', s @ ..] => {
							super::push(out, 24678);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['o', 't', s @ ..] => {
									super::push(out, 33156);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 33157);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 33158);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 38923);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'a', 'k', 's', s @ ..] => {
							super::push(out, 24679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							super::push(out, 24680);
							s.is_empty()
						}
						['m', 'r', s @ ..] => {
							super::push(out, 24681);
							s.is_empty()
						}
						['r', 't', 'k', 'e', 'l', 'e', 'r', s @ ..] => {
							super::push(out, 24682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äse.
pub(crate) fn n1297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1295);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5588);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5589);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14342);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24684);
							s.is_empty()
						}
						['e', 'l', s @ ..] => {
							super::push(out, 24685);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 33159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', 'b', s @ ..] => {
					super::push(out, 14343);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5590);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 14345);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 14346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5591);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['j', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 24686);
							s.is_empty()
						}
						['l', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 24687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5592);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14347);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öse.
pub(crate) fn n1298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1296);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5593);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5594);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5595);
			match s {
				[] => true,
				['k', 'e', s @ ..] => {
					super::push(out, 14349);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5596);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14350);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üse.
pub(crate) fn n1299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1297);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5597);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 5598);
			match s {
				[] => true,
				['x', s @ ..] => {
					super::push(out, 14351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5599);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14352);
					match s {
						[] => true,
						['g', 's', s @ ..] => {
							super::push(out, 24688);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 33160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5600);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ete.
pub(crate) fn n1303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1301);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5619);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5620);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5621);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14389);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 5622);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14390);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 14391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5623);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14392);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5624);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 14394);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5625);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14395);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5626);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 14397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5627);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14398);
					match s {
						[] => true,
						['a', 'h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 24709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24710);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24711);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 33165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 14399);
					match s {
						[] => true,
						['t', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 24712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äfte.
pub(crate) fn n5679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5632);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 14402);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 14403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öfte.
pub(crate) fn n5680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5633);
	match s {
		[] => true,
		['h', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 14404);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gte.
pub(crate) fn n1305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1303);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5634);
			match s {
				[] => true,
				['l', 'k', 'e', 'b', s @ ..] => {
					super::push(out, 14405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hte.
pub(crate) fn n1306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1304);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14406);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 14407);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24714);
							match s {
								[] => true,
								['f', 's', s @ ..] => {
									super::push(out, 33166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 24715);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 33167);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 33168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 14408);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 24716);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 24717);
							match s {
								[] => true,
								['c', 's', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 38924);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 42261);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 38925);
											s.is_empty()
										}
										['h', 'c', 'a', s @ ..] => {
											super::push(out, 38926);
											s.is_empty()
										}
										['k', 'i', 't', s @ ..] => {
											super::push(out, 38927);
											s.is_empty()
										}
										['l', s @ ..] => {
											match s {
												[] => true,
												['a', 'b', s @ ..] => {
													super::push(out, 42262);
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 42263);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['e', s @ ..] => super::t12::n46814(s, out),
														['h', 'c', s @ ..] => super::t12::n46815(s, out),
														_ => false,
													}
												}
												['i', s @ ..] => {
													super::push(out, 42264);
													match s {
														[] => true,
														['z', s @ ..] => super::t12::n46816(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 42265);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 42266);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 42267);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 38928);
											match s {
												[] => true,
												['g', 'n', 'u', 'l', s @ ..] => {
													super::push(out, 42268);
													match s {
														[] => true,
														['d', s @ ..] => super::t12::n46817(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 42269);
													match s {
														[] => true,
														['i', s @ ..] => super::t12::n46818(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 42270);
													s.is_empty()
												}
												['t', 'e', 'l', s @ ..] => {
													super::push(out, 42271);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 24718);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 33169);
									match s {
										[] => true,
										['g', 'n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 38929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 14409);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 33170);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 38930);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 33171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 14410);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 24719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 14411);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24720);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5636);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5637);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14413);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ite.
pub(crate) fn n1307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1305);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5638);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5639);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14414);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14415);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5640);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 14416);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 24722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14417);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 24723);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 33172);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 38931);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 24724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33173);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 33174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 24725);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 33175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 14418);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24726);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'e', s @ ..] => {
											super::push(out, 38932);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 38933);
											match s {
												[] => true,
												['r', 'o', 'v', 'r', s @ ..] => {
													super::push(out, 42272);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 38934);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 14419);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 24727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5641);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5642);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5643);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5644);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14423);
					s.is_empty()
				}
				['d', 'o', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 14424);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14425);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14426);
					match s {
						[] => true,
						['f', 'o', 't', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 24728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5645);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14427);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 14428);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14429);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14430);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 24730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5646);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14431);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14432);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jte.
pub(crate) fn n1308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1306);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 5647);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kte.
pub(crate) fn n1309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1307);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5648);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5649);
			match s {
				[] => true,
				['ü', s @ ..] => {
					super::push(out, 14433);
					match s {
						[] => true,
						['r', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 24731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5650);
			match s {
				[] => true,
				['s', '-', s @ ..] => {
					super::push(out, 14434);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5651);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 14435);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lte.
pub(crate) fn n1310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1308);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5652);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14436);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 14437);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 14438);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 14439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5653);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 14440);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14441);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 14442);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 14443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5654);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 14444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5655);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14445);
					match s {
						[] => true,
						['t', 's', 'e', 'g', 'n', 'a', 'f', s @ ..] => {
							super::push(out, 24733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5656);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 5657);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 5658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mte.
pub(crate) fn n1311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1309);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 5659);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5660);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5661);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5662);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nte.
pub(crate) fn n1312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1310);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5663);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14448);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14449);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 24735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 14450);
					match s {
						[] => true,
						['r', 'a', 'v', '-', s @ ..] => {
							super::push(out, 24736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14451);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24737);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 33176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14452);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 24738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14453);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 24739);
							match s {
								[] => true,
								['n', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 38935);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 38936);
											s.is_empty()
										}
										['s', 'n', 'o', 'i', 't', 'a', 't', s @ ..] => {
											super::push(out, 38937);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 14454);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 24740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5664);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14455);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 24741);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 24742);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 24743);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14456);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24745);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 33177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14457);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14458);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24747);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 33178);
									match s {
										[] => true,
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 38938);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14459);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24748);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 24749);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 33179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 24750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5665);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5666);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5667);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5668);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14462);
					match s {
						[] => true,
						['s', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 24751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5669);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ote.
pub(crate) fn n1313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1311);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5670);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 5671);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5672);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14463);
					match s {
						[] => true,
						['r', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 24752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5673);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5674);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 14464);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 14465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5675);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5676);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 5677);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 5678);
			match s {
				[] => true,
				['h', 'c', 's', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 14466);
					s.is_empty()
				}
				['s', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 14467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', s @ ..] => {
			super::push(out, 5679);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 14468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5680);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5681);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 14469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5682);
			match s {
				[] => true,
				['q', 'i', s @ ..] => {
					super::push(out, 14470);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 5683);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 5684);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14471);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 14472);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rte.
pub(crate) fn n1314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1312);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5685);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14473);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24754);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 33180);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38939);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14474);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 24755);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 33181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 14475);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 24756);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 33182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5686);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14476);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24757);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 33183);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 33184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 14477);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 24758);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 24759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 14478);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 14479);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 14480);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24760);
							match s {
								[] => true,
								['m', 'k', 'c', 'a', 'l', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 33185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 24761);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 33186);
									match s {
										[] => true,
										['a', 'd', 'a', '\u{1}', s @ ..] => {
											super::push(out, 38940);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 't', s @ ..] => {
									super::push(out, 33187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 24762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14481);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14482);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 14483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5687);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14484);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 14485);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33188);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 33189);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 33190);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 33191);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 33192);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 33193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 24764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5688);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 14486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5689);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 14487);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24765);
							match s {
								[] => true,
								['n', 'a', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 33194);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'n', s @ ..] => {
					super::push(out, 14488);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24766);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 33195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'm', s @ ..] => {
			super::push(out, 5690);
			match s {
				[] => true,
				['d', 'u', '\u{1}', s @ ..] => {
					super::push(out, 14489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5691);
			match s {
				[] => true,
				['h', 's', s @ ..] => {
					super::push(out, 14490);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 5692);
			match s {
				[] => true,
				['d', 'l', 'e', 'k', 'n', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 14491);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14492);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ste.
pub(crate) fn n1315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1313);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5693);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 14493);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 14494);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 14495);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 24768);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 33196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 14496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5694);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14497);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 14498);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 14499);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 14500);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14501);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14502);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24769);
							s.is_empty()
						}
						['r', 'e', 'w', 'r', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 24770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 14503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5695);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 14504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5696);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14505);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 24771);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 33197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 24772);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 33198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5697);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 14508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5698);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 14509);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5699);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14510);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24775);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 24776);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 24777);
							s.is_empty()
						}
						['r', 'e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 24778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 14511);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 14512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5700);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 14513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5701);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 14514);
					s.is_empty()
				}
				['r', 'k', 'd', s @ ..] => {
					super::push(out, 14515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5702);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 14516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 5703);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 14517);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24780);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 33199);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14518);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 14519);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tte.
pub(crate) fn n1316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1314);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5704);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 14520);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 33200);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 33201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14521);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 24782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14522);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 24783);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 33202);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 38941);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 24784);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 33203);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14523);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24785);
							match s {
								[] => true,
								['s', 'a', 'm', s @ ..] => {
									super::push(out, 33204);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 38942);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14524);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 14525);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 24786);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5705);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 14527);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 24788);
							match s {
								[] => true,
								['o', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 33205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 14528);
					match s {
						[] => true,
						['s', 'g', s @ ..] => {
							super::push(out, 24789);
							match s {
								[] => true,
								['n', 'u', 'm', s @ ..] => {
									super::push(out, 33206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24790);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 24791);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 24792);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 24793);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 24794);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 33208);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 38943);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 38944);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14530);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 24795);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 33209);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 38945);
											s.is_empty()
										}
										['n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 38946);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 24796);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 33210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14531);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 14532);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 24797);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 33211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 14533);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 24798);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 33212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 14534);
					match s {
						[] => true,
						['u', 'o', s @ ..] => {
							super::push(out, 24799);
							match s {
								[] => true,
								['r', 'p', 'e', '\u{1}', s @ ..] => {
									super::push(out, 33213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5706);
			match s {
				[] => true,
				['m', 'r', s @ ..] => {
					super::push(out, 14535);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							super::push(out, 24800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14536);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24801);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 24802);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5707);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 14537);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24803);
							match s {
								[] => true,
								['a', 'y', 'w', '\u{1}', s @ ..] => {
									super::push(out, 33215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14538);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 24804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14539);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 24805);
							match s {
								[] => true,
								['r', 'e', 'e', s @ ..] => {
									super::push(out, 33216);
									match s {
										[] => true,
										['m', 'z', s @ ..] => {
											super::push(out, 38947);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 24806);
							match s {
								[] => true,
								['n', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 33217);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 33218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14540);
					match s {
						[] => true,
						['a', 'h', s @ ..] => {
							super::push(out, 24807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14541);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 24808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 14542);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 14543);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 24809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 5708);
			match s {
				[] => true,
				['l', s @ ..] => super::t12::n14810(s, out),
				['t', s @ ..] => {
					super::push(out, 14545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => super::t12::n5757(s, out),
		_ => false,
	}
}

/// Words ending -ute.
pub(crate) fn n1317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1315);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5710);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 14547);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 24812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 14548);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14549);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 24813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5711);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 14550);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', s @ ..] => {
									super::push(out, 33219);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 33220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 14551);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 14552);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 24814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33221);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 24815);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14553);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 14554);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 24816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 14555);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 5713);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 5714);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5715);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14558);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 24817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 14559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5716);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5717);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 14560);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 14561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5718);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5719);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 5720);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14562);
					match s {
						[] => true,
						['e', 'g', 'd', s @ ..] => {
							super::push(out, 24818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yte.
pub(crate) fn n1318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1316);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5721);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14563);
					match s {
						[] => true,
						['n', 'n', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 24819);
							s.is_empty()
						}
						['r', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 24820);
							s.is_empty()
						}
						['t', 't', 'e', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 24821);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 24822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'b', s @ ..] => {
					match s {
						[] => true,
						['e', 'z', s @ ..] => {
							super::push(out, 24823);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 24824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 's', '\u{1}', s @ ..] => {
			super::push(out, 5722);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zte.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1317);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 5723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äte.
pub(crate) fn n1320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1318);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 5724);
			match s {
				[] => true,
				['s', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 14564);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ête.
pub(crate) fn n1321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1319);
	match s {
		[] => true,
		['t', '-', s @ ..] => {
			super::push(out, 5725);
			match s {
				[] => true,
				['à', '-', 'e', 't', 'ê', 't', '\u{1}', s @ ..] => {
					super::push(out, 14565);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

