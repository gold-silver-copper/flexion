use flexion::Path;

/// Words ending -rfen.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2668);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2669);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4924);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2670);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2671);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4925);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2672);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4926);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2673);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4927);
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
						['n', s @ ..] => {
							super::push(out, 2674);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4928);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 4929);
									match s {
										[] => true,
										['h', 'c', 's', 'i', 'w', 'z', 'a', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4930);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 6561);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 6562);
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
						['r', s @ ..] => {
							super::push(out, 2675);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4931);
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											super::push(out, 6563);
											s.is_empty()
										}
										['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 6564);
											s.is_empty()
										}
										['h', 'r', 'e', 't', 'n', 'i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 6565);
											s.is_empty()
										}
										['t', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 4932);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2676);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 2677);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4933);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 4934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2678);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 4935);
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
		['u', s @ ..] => {
			super::push(out, 369);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4936);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4937);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => super::t3::n385(s, out),
		_ => false,
	}
}

/// Words ending -ufen.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2682);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2683);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2684);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2685);
							match s {
								[] => true,
								['e', 'r', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 2686);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2687);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4938);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4939);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2688);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4940);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 7415);
													match s {
														[] => true,
														['e', 't', 'i', 'e', 'w', '\u{1}', s @ ..] => super::t3::n13891(s, out),
														_ => false,
													}
												}
												['s', s @ ..] => {
													super::push(out, 7416);
													match s {
														[] => true,
														['u', 'a', '\u{1}', s @ ..] => super::t3::n13892(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2689);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4941);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 4942);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2690);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t3::n1323(s, out),
				['n', s @ ..] => {
					super::push(out, 1192);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2703);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6573);
											s.is_empty()
										}
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 6574);
											s.is_empty()
										}
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
					super::push(out, 1193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2704);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t3::n1326(s, out),
				['t', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2710);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => super::t3::n6744(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t3::n3324(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => super::t3::n387(s, out),
		['r', s @ ..] => super::t3::n388(s, out),
		['t', s @ ..] => super::t3::n389(s, out),
		['ä', s @ ..] => super::t3::n390(s, out),
		_ => false,
	}
}

/// Words ending -üfen.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2728);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2729);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2730);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2731);
							match s {
								[] => true,
								['e', 'b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -agen.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ß', 'i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1218);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1219);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1220);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4959);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 4960);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6575);
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
								['e', s @ ..] => super::t3::n6774(s, out),
								['f', s @ ..] => super::t3::n6775(s, out),
								['h', s @ ..] => super::t3::n6776(s, out),
								['k', s @ ..] => super::t3::n6777(s, out),
								['l', s @ ..] => super::t3::n6778(s, out),
								['m', s @ ..] => super::t3::n6779(s, out),
								['n', s @ ..] => super::t3::n6780(s, out),
								['r', s @ ..] => super::t3::n6781(s, out),
								['s', s @ ..] => super::t3::n6782(s, out),
								['t', s @ ..] => super::t3::n6783(s, out),
								['u', s @ ..] => super::t3::n6784(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => super::t3::n1355(s, out),
				['n', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				['p', s @ ..] => super::t3::n1357(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t3::n395(s, out),
		['r', s @ ..] => super::t3::n396(s, out),
		['s', s @ ..] => super::t3::n397(s, out),
		['t', s @ ..] => super::t3::n398(s, out),
		['w', s @ ..] => super::t3::n399(s, out),
		['z', s @ ..] => super::t3::n400(s, out),
		_ => false,
	}
}

/// Words ending -fegen.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1250);
			match s {
				[] => true,
				['e', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4984);
							match s {
								[] => true,
								['i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1251);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iegen.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2779);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2780);
					match s {
						[] => true,
						['d', 'a', 'r', 'e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2781);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2782);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2783);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2784);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1253);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2785);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 2786);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4985);
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
				['b', s @ ..] => {
					super::push(out, 2787);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4986);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4987);
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
				['d', s @ ..] => {
					super::push(out, 2788);
					match s {
						[] => true,
						['n', 'u', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2789);
					match s {
						[] => true,
						['h', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2790);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4988);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4989);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7424);
											match s {
												[] => true,
												['e', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4990);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4991);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4992);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4993);
							match s {
								[] => true,
								['c', 'o', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4994);
							match s {
								[] => true,
								['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4995);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6601);
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4996);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7425);
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
						['u', s @ ..] => {
							super::push(out, 4997);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2791);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2792);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4998);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2793);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2794);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2795);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5000);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6602);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 6603);
									match s {
										[] => true,
										['n', 'a', 'n', 'i', 'e', s @ ..] => {
											match s {
												[] => true,
												['f', 'u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => super::t3::n13902(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => super::t3::n10271(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t3::n6864(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t3::n1392(s, out),
		['r', s @ ..] => super::t3::n1393(s, out),
		['s', s @ ..] => super::t3::n1394(s, out),
		['w', s @ ..] => super::t3::n1395(s, out),
		_ => false,
	}
}

/// Words ending -legen.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1258);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1259);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2804);
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
		['e', s @ ..] => {
			super::push(out, 1260);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2805);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'i', 'e', 's', 'i', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1261);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 2806);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5007);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5008);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5009);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2807);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1262);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2808);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1263);
			match s {
				[] => true,
				['c', 'a', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1264);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5010);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', 'l', 'i', 't', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 2809);
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
		['n', s @ ..] => {
			super::push(out, 1266);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2810);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2811);
					match s {
						[] => true,
						['f', 'f', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2812);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6607);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 5011);
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
		['r', s @ ..] => {
			super::push(out, 1267);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2813);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'ü', '\u{1}', s @ ..] => {
							super::push(out, 5012);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5013);
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6608);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6609);
									match s {
										[] => true,
										['a', 'n', 'i', 'e', s @ ..] => {
											match s {
												[] => true,
												['f', 'u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 5014);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5015);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5016);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6610);
									match s {
										[] => true,
										['o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2815);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1268);
			match s {
				[] => true,
				['o', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2816);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1269);
			match s {
				[] => true,
				['h', 'c', 'e', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'e', 'r', 'e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'o', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1270);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 2817);
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
		['y', 'e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ß', 'u', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -regen.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1271);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1272);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'b', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1273);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1274);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1275);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wegen.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1276);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5017);
							s.is_empty()
						}
						['m', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', 'm', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'r', 'o', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5018);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ggen.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 393);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2818);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2819);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 394);
			match s {
				[] => true,
				['b', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1277);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -igen.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 395);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 396);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1278);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1279);
					match s {
						[] => true,
						['u', 'h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5019);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 5020);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['e', 'b', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n13907(s, out),
														['n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => super::t3::n13908(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 7427);
													match s {
														[] => true,
														['a', '\u{1}', s @ ..] => super::t3::n13909(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1280);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5021);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 2822);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5022);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 5023);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 6611);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 6612);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 2823);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 5025);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 6613);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 6614);
											match s {
												[] => true,
												['a', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n13910(s, out),
														['r', s @ ..] => super::t3::n13911(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['d', 'n', 'o', 's', '\u{1}', s @ ..] => {
													super::push(out, 7428);
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													super::push(out, 7429);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 5026);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 5027);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 6615);
											match s {
												[] => true,
												['e', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1281);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2824);
							s.is_empty()
						}
						['ü', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5028);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5029);
									match s {
										[] => true,
										['a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 6616);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2825);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n411(s, out),
		['h', s @ ..] => {
			super::push(out, 398);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['i', 'e', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2836);
							s.is_empty()
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', s @ ..] => super::t3::n1436(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t3::n414(s, out),
		['r', s @ ..] => {
			super::push(out, 401);
			s.is_empty()
		}
		['s', 's', 'ä', 'm', '\u{1}', s @ ..] => {
			super::push(out, 402);
			s.is_empty()
		}
		['t', s @ ..] => super::t3::n417(s, out),
		['z', s @ ..] => super::t3::n418(s, out),
		['ß', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['i', 'e', 'l', 'f', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1297);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lgen.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 406);
			s.is_empty()
		}
		['e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 407);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2845);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2846);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2847);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 409);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2848);
							s.is_empty()
						}
						['b', 'a', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2849);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2850);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2851);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 6640);
											match s {
												[] => true,
												['n', 'a', 'n', 'i', 'e', 'f', 'u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 6641);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7432);
													s.is_empty()
												}
												['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2852);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2853);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 5068);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ngen.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1298);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2855);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2856);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2857);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5069);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5070);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'm', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2858);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2859);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2860);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5071);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 6642);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 5072);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 6643);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 2861);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5073);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2862);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 6644);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 6645);
											match s {
												[] => true,
												['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 6646);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['b', s @ ..] => {
													super::push(out, 7433);
													match s {
														[] => true,
														['a', '\u{1}', s @ ..] => super::t3::n13912(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2863);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2864);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1302);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2865);
							s.is_empty()
						}
						['i', s @ ..] => super::t3::n3556(s, out),
						['n', s @ ..] => super::t3::n3557(s, out),
						['r', s @ ..] => super::t3::n3558(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t3::n1459(s, out),
				['r', s @ ..] => super::t3::n1460(s, out),
				['s', s @ ..] => super::t3::n1461(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t3::n426(s, out),
		['o', s @ ..] => {
			super::push(out, 413);
			s.is_empty()
		}
		['ä', s @ ..] => super::t3::n428(s, out),
		['ü', s @ ..] => super::t3::n429(s, out),
		_ => false,
	}
}

/// Words ending -rgen.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2911);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2912);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 6691);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 6692);
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
				['z', '\u{1}', s @ ..] => {
					super::push(out, 1321);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2913);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2914);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1323);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2915);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2916);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 5129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2917);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 's', 'r', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5130);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2919);
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2920);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2921);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'u', 'r', '\u{1}', s @ ..] => {
											super::push(out, 6693);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ugen.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1324);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2922);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2923);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5131);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 5132);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2924);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2925);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2926);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2927);
							match s {
								[] => true,
								['s', 'e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1327);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2928);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5133);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 6694);
											match s {
												[] => true,
												['ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 6695);
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
								['o', s @ ..] => {
									super::push(out, 5134);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2929);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 422);
			s.is_empty()
		}
		['l', '\u{1}', s @ ..] => {
			super::push(out, 423);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 424);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ägen.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2930);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2931);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2932);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 427);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1330);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1331);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1332);
					match s {
						[] => true,
						['e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1333);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1335);
					match s {
						[] => true,
						['a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2933);
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ögen.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1336);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1337);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ügen.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1339);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2934);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2935);
							match s {
								[] => true,
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2936);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1340);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1341);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 5135);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5136);
									match s {
										[] => true,
										['i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1345);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1346);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2938);
							match s {
								[] => true,
								['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1348);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2940);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 432);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1349);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2941);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -chen.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1352);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1353);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2944);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 2945);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5137);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2946);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2947);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2948);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5138);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5139);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 5140);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5141);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 5142);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', 'i', 'k', 'c', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2949);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2950);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5143);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 5144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'p', 'a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 2951);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2952);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2953);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 5145);
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 5146);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2954);
							match s {
								[] => true,
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 5147);
									s.is_empty()
								}
								['t', 'u', 'p', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t3::n1520(s, out),
				['r', s @ ..] => {
					super::push(out, 1357);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 2964);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5155);
									s.is_empty()
								}
								['r', 'e', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1358);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2965);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2966);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => super::t3::n7164(s, out),
								_ => false,
							}
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => super::t3::n3692(s, out),
						['r', s @ ..] => super::t3::n3693(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n452(s, out),
		['i', s @ ..] => super::t3::n453(s, out),
		['l', s @ ..] => super::t3::n454(s, out),
		['n', s @ ..] => super::t3::n455(s, out),
		['o', s @ ..] => super::t3::n456(s, out),
		['r', s @ ..] => super::t3::n457(s, out),
		['s', s @ ..] => super::t3::n458(s, out),
		['t', s @ ..] => super::t3::n459(s, out),
		['u', s @ ..] => super::t3::n460(s, out),
		['ä', s @ ..] => super::t3::n461(s, out),
		_ => false,
	}
}

/// Words ending -ehen.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1400);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1401);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3055);
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
				['d', 'm', 'e', 'r', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1402);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'r', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1403);
					match s {
						[] => true,
						['e', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 3057);
							s.is_empty()
						}
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1404);
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1405);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'u', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'z', 't', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'h', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1406);
					match s {
						[] => true,
						['i', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5283);
									s.is_empty()
								}
								['r', 'e', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1407);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3058);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => super::t3::n10709(s, out),
										['o', 'v', '\u{1}', s @ ..] => super::t3::n10710(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => super::t3::n3808(s, out),
						['i', s @ ..] => super::t3::n3809(s, out),
						['o', 'v', 'a', 'd', '\u{1}', s @ ..] => super::t3::n3810(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n1578(s, out),
				['s', s @ ..] => super::t3::n1579(s, out),
				['t', s @ ..] => super::t3::n1580(s, out),
				['u', s @ ..] => super::t3::n1581(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t3::n463(s, out),
		['i', s @ ..] => super::t3::n464(s, out),
		['l', s @ ..] => super::t3::n465(s, out),
		['r', s @ ..] => super::t3::n466(s, out),
		['s', s @ ..] => super::t3::n467(s, out),
		['t', s @ ..] => super::t3::n468(s, out),
		['w', s @ ..] => super::t3::n469(s, out),
		_ => false,
	}
}

/// The word ausruhen.
pub(crate) fn n3911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3134);
	s.is_empty()
}

/// Words ending -ähen.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 465);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3135);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 467);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1444);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5334);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1446);
					match s {
						[] => true,
						['i', 'h', 'a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1447);
					match s {
						[] => true,
						['e', 'd', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1448);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1449);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3136);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3137);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öhen.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ühen.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3138);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3139);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3140);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3141);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 6827);
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
						['s', s @ ..] => {
							super::push(out, 3142);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3143);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3144);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3145);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5335);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 473);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1452);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1453);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1454);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3146);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3147);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5336);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3148);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -cken.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1466);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3156);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3157);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3158);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3159);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 6828);
											match s {
												[] => true,
												['ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 6829);
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
						['t', s @ ..] => {
							super::push(out, 3160);
							match s {
								[] => true,
								['s', 'e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1467);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3161);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3162);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1468);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3164);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3165);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5345);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1469);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3166);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1470);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3167);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5346);
									match s {
										[] => true,
										['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5347);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3168);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3169);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5348);
									s.is_empty()
								}
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3170);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3171);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1472);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 3172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1473);
					s.is_empty()
				}
				['w', 'z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n503(s, out),
		['i', s @ ..] => super::t4::n504(s, out),
		['o', s @ ..] => super::t4::n505(s, out),
		['u', s @ ..] => super::t4::n506(s, out),
		['ö', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		['ü', s @ ..] => super::t4::n508(s, out),
		_ => false,
	}
}

/// Words ending -nken.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3266);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3267);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5437);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1527);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5438);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3269);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3270);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3271);
							match s {
								[] => true,
								['l', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3272);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3273);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3274);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3275);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6870);
											match s {
												[] => true,
												['n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'u', 'd', '\u{1}', s @ ..] => {
											super::push(out, 6871);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3276);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3277);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3278);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5439);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5440);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3279);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5441);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 6872);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 5442);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3280);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t4::n4131(s, out),
						_ => false,
					}
				}
				['h', s @ ..] => super::t4::n1720(s, out),
				['l', s @ ..] => super::t4::n1721(s, out),
				['r', s @ ..] => super::t4::n1722(s, out),
				['s', s @ ..] => super::t4::n1723(s, out),
				['w', s @ ..] => super::t4::n1724(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n514(s, out),
		['u', s @ ..] => super::t4::n515(s, out),
		['ä', s @ ..] => super::t4::n516(s, out),
		['ü', s @ ..] => super::t4::n517(s, out),
		_ => false,
	}
}

/// Words ending -nmerken.
pub(crate) fn n4181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3317);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmerken.
pub(crate) fn n4182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3318);
	match s {
		[] => true,
		['e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5463);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -werken.
pub(crate) fn n1742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1550);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irken.
pub(crate) fn n520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 505);
	match s {
		[] => true,
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3319);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 3320);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3321);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3322);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5464);
							match s {
								[] => true,
								['g', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['a', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5465);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 6881);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 6882);
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
				['r', s @ ..] => {
					super::push(out, 3323);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3324);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3325);
					match s {
						[] => true,
						['i', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urken.
pub(crate) fn n522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 507);
	s.is_empty()
}

/// Words ending -ärken.
pub(crate) fn n523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 508);
	match s {
		[] => true,
		['t', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3326);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word türken.
pub(crate) fn n524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 509);
	s.is_empty()
}

/// Words ending -uken.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['p', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äken.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	s.is_empty()
}

/// Words ending -öken.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	s.is_empty()
}

/// The word nachspielen.
pub(crate) fn n13061(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word hochspielen.
pub(crate) fn n13062(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word durchspielen.
pub(crate) fn n13063(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word anspielen.
pub(crate) fn n13064(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -einspielen.
pub(crate) fn n13065(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'i', 'h', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word überspielen.
pub(crate) fn n13066(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word herunterspielen.
pub(crate) fn n13067(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -verspielen.
pub(crate) fn n13068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7515);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word vorspielen.
pub(crate) fn n13069(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -zielen.
pub(crate) fn n1756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1560);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3327);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3328);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3329);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -welen.
pub(crate) fn n534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 518);
	s.is_empty()
}

/// Words ending -glen.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hlen.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 520);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1561);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3330);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3331);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1563);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 3332);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5474);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5475);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5476);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 5477);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6887);
											s.is_empty()
										}
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
					super::push(out, 1564);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3333);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3334);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3335);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5478);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 6888);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', 'u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3336);
							match s {
								[] => true,
								['c', 'ü', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'i', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3337);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5479);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5480);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3338);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 6889);
											match s {
												[] => true,
												['ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'i', 'e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3339);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'r', 'o', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3340);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 521);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1565);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3341);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3342);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6890);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'm', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3343);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3344);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => super::t4::n7781(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => super::t4::n1762(s, out),
				['k', s @ ..] => super::t4::n1763(s, out),
				['t', s @ ..] => super::t4::n1764(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n538(s, out),
		['u', s @ ..] => super::t4::n539(s, out),
		['ä', s @ ..] => super::t4::n540(s, out),
		['ö', s @ ..] => super::t4::n541(s, out),
		['ü', s @ ..] => super::t4::n542(s, out),
		_ => false,
	}
}

/// Words ending -ilen.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 3376);
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1581);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3377);
							match s {
								[] => true,
								['h', 'c', 'e', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3378);
							match s {
								[] => true,
								['z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1583);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3380);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 3381);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 5489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3382);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3383);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7516);
													s.is_empty()
												}
												['e', 'b', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', 'e', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 5490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1585);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3384);
							match s {
								[] => true,
								['b', 'r', s @ ..] => {
									match s {
										[] => true,
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['e', 'g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'v', 'a', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1588);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3385);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1590);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3386);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3387);
							match s {
								[] => true,
								['a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t4::n1791(s, out),
				['u', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 3397);
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
				['w', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3398);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -llen.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 529);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3399);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3400);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5502);
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
						['e', s @ ..] => {
							super::push(out, 3401);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3402);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 5503);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3403);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5504);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3404);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3405);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3406);
							match s {
								[] => true,
								['i', 'e', 'h', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
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
						['n', s @ ..] => {
							super::push(out, 3407);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5506);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 5507);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 6898);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', s @ ..] => super::t4::n13085(s, out),
												_ => false,
											}
										}
										['h', s @ ..] => super::t4::n11046(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t4::n4304(s, out),
						['s', s @ ..] => super::t4::n4305(s, out),
						['t', s @ ..] => super::t4::n4306(s, out),
						['u', s @ ..] => super::t4::n4307(s, out),
						['ß', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 3412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => super::t4::n1797(s, out),
				['l', s @ ..] => super::t4::n1798(s, out),
				['n', s @ ..] => super::t4::n1799(s, out),
				['r', s @ ..] => super::t4::n1800(s, out),
				['w', s @ ..] => super::t4::n1801(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n546(s, out),
		['i', s @ ..] => super::t4::n547(s, out),
		['o', s @ ..] => super::t4::n548(s, out),
		['u', s @ ..] => super::t4::n549(s, out),
		['ä', s @ ..] => super::t4::n550(s, out),
		['ü', s @ ..] => super::t4::n551(s, out),
		_ => false,
	}
}

/// Words ending -olen.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1634);
					match s {
						[] => true,
						['i', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1635);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1636);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5551);
									match s {
										[] => true,
										['ü', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 7534);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['d', 'e', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 5552);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 5553);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1637);
					match s {
						[] => true,
						['u', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5555);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 537);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 538);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1638);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rlen.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 539);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 540);
			match s {
				[] => true,
				['u', 'q', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3459);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ulen.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 541);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1639);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3461);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3462);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3463);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1640);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3464);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 3465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3466);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1641);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3467);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3468);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3469);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3470);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3471);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3472);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 545);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['m', 'u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3473);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wlen.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	s.is_empty()
}

/// Words ending -ylen.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	s.is_empty()
}

/// Words ending -älen.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1642);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ölen.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1643);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1644);
					s.is_empty()
				}
				['é', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1645);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ülen.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['p', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1646);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1647);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1649);
					match s {
						[] => true,
						['c', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1650);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1651);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1652);
					match s {
						[] => true,
						['a', 'l', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 5556);
									match s {
										[] => true,
										['ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'u', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1654);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nehmen.
pub(crate) fn n1872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1660);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3482);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3483);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5558);
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
		['e', s @ ..] => {
			super::push(out, 3484);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3485);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5559);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3486);
			match s {
				[] => true,
				['e', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3487);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'e', 'r', 'f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3488);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3489);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5561);
					match s {
						[] => true,
						['g', 'e', 'g', 't', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5562);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6935);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6936);
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
		['r', s @ ..] => {
			super::push(out, 3490);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5563);
					match s {
						[] => true,
						['b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6937);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7751);
											match s {
												[] => true,
												['n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 7752);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7535);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5564);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5565);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3491);
			match s {
				[] => true,
				['s', 'a', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3492);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5566);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5567);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'e', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3493);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 5568);
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
		['ß', 'a', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ühmen.
pub(crate) fn n578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 558);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3498);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3499);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -imen.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1661);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3500);
							s.is_empty()
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3502);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3503);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5569);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3504);
							match s {
								[] => true,
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3505);
							match s {
								[] => true,
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 560);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lmen.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['a', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'z', '\u{1}', s @ ..] => {
					super::push(out, 1665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1666);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mmen.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 561);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1667);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 5570);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6938);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5571);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'e', 'b', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 5572);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5573);
									match s {
										[] => true,
										['e', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1670);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5574);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5575);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 562);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3506);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5576);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5577);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5578);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 6939);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1672);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1673);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 5579);
									match s {
										[] => true,
										['e', 'w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5580);
									match s {
										[] => true,
										['e', 'b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5581);
									match s {
										[] => true,
										['r', 'o', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 563);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1674);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1675);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5582);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3508);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5583);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => super::t4::n7996(s, out),
								['r', 'e', '\u{1}', s @ ..] => super::t4::n7997(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t4::n1894(s, out),
				['t', s @ ..] => super::t4::n1895(s, out),
				['w', s @ ..] => super::t4::n1896(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n586(s, out),
		['u', s @ ..] => super::t4::n587(s, out),
		['ä', s @ ..] => super::t4::n588(s, out),
		['ü', s @ ..] => super::t4::n589(s, out),
		_ => false,
	}
}

/// Words ending -omen.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3532);
							s.is_empty()
						}
						['s', 'u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmen.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 571);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3534);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3535);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 572);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3536);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3537);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['d', 't', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1693);
					s.is_empty()
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3538);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5605);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3539);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 6960);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 3540);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 5606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3541);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 574);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3542);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3543);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5608);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 5609);
									match s {
										[] => true,
										['c', 'a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5610);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 5611);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tmen.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1694);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1695);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1696);
					match s {
						[] => true,
						['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1697);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1698);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -umen.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['l', 'f', 'p', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 577);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1701);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1702);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5612);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 5613);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5614);
									match s {
										[] => true,
										['e', 'b', 'ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3544);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3545);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3546);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3547);
							match s {
								[] => true,
								['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3548);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3549);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 5615);
									match s {
										[] => true,
										['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5616);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3550);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 6961);
											match s {
												[] => true,
												['r', 'e', 't', 'n', 'i', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 6962);
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
						['s', s @ ..] => {
							super::push(out, 3551);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3552);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 5617);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 5618);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6963);
											match s {
												[] => true,
												['l', 'k', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 6964);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 6965);
											match s {
												[] => true,
												['f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 5619);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1704);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3553);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3554);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ämen.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				['d', 'm', 'e', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3555);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1708);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ömen.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3556);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3557);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 5620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3558);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 6966);
											match s {
												[] => true,
												['ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3559);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3560);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 5621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word verordnen.
pub(crate) fn n13197(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -egnen.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 586);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3575);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3576);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3577);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3578);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1712);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3579);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ignen.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 587);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1713);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3580);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3581);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3582);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3583);
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'g', 'n', 'a', 'w', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1714);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 3585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -läugnen.
pub(crate) fn n1948(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3587);
			s.is_empty()
		}
		['r', 'e', 'v', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hnen.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3588);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3589);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3590);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3591);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3592);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1717);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3593);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3594);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 3595);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3596);
							match s {
								[] => true,
								['a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
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
						['\u{1}', s @ ..] => {
							super::push(out, 3597);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3598);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 590);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1719);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5626);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 5627);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 6968);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7542);
													s.is_empty()
												}
												['r', 'e', 'h', 'r', 'o', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', 'u', 'a', 'r', 'o', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 5628);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 5629);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['a', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 5630);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => super::t4::n8108(s, out),
								['r', s @ ..] => super::t4::n8109(s, out),
								['s', s @ ..] => {
									super::push(out, 5633);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => super::t4::n8111(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t4::n1956(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n617(s, out),
		['o', s @ ..] => super::t4::n618(s, out),
		['ä', s @ ..] => super::t4::n619(s, out),
		['ö', s @ ..] => super::t4::n620(s, out),
		['ü', 's', '\u{1}', s @ ..] => super::t4::n621(s, out),
		_ => false,
	}
}

/// Words ending -inen.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1733);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1734);
					match s {
						[] => true,
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 3621);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6977);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 6978);
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['f', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 6979);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 6980);
											match s {
												[] => true,
												['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
													super::push(out, 7543);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 6981);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 6982);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1738);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3623);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 3624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5648);
									s.is_empty()
								}
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3625);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3626);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3627);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 597);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1741);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -knen.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 598);
			match s {
				[] => true,
				['o', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3628);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3629);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3630);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3631);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mnen.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 599);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nnen.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 600);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3632);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1743);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'ü', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5649);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5650);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 5651);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 5652);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											super::push(out, 6983);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5653);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 6984);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 6985);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5654);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 6986);
											match s {
												[] => true,
												['b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 6987);
											match s {
												[] => true,
												['v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 5655);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5656);
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 601);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3633);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 3634);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 6988);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t4::n4651(s, out),
						['s', s @ ..] => {
							super::push(out, 3636);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t4::n1986(s, out),
				['n', s @ ..] => super::t4::n1987(s, out),
				['p', s @ ..] => super::t4::n1988(s, out),
				['r', s @ ..] => super::t4::n1989(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n628(s, out),
		['o', s @ ..] => super::t4::n629(s, out),
		['ö', s @ ..] => super::t4::n630(s, out),
		['ü', s @ ..] => super::t4::n631(s, out),
		_ => false,
	}
}

/// Words ending -onen.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1761);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'h', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 606);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pnen.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 607);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnen.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 608);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1763);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3676);
							s.is_empty()
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3677);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 3678);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 5679);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3679);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3680);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3681);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5680);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 5681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3682);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3683);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3684);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 610);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1766);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5682);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 611);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 612);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3685);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', 'z', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1767);
					s.is_empty()
				}
				['r', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word jausnen.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	s.is_empty()
}

/// Words ending -unen.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3687);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3689);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3690);
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 614);
			s.is_empty()
		}
		['t', '\u{1}', s @ ..] => {
			super::push(out, 615);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1769);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5683);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3692);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3693);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3694);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -änen.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	s.is_empty()
}

/// Words ending -önen.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['p', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 617);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 618);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1772);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 3695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1773);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1774);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5684);
									match s {
										[] => true,
										['ü', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ünen.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['r', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1775);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1776);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1777);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -klappen.
pub(crate) fn n4744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3701);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 5687);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5688);
			match s {
				[] => true,
				['c', 'o', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5689);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5690);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5691);
			match s {
				[] => true,
				['z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nappen.
pub(crate) fn n2033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1783);
	match s {
		[] => true,
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5692);
					match s {
						[] => true,
						['e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rappen.
pub(crate) fn n2034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1784);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['h', 'c', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tappen.
pub(crate) fn n2035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1785);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3703);
			match s {
				[] => true,
				['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3704);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zappen.
pub(crate) fn n2036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1786);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3705);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3706);
			match s {
				[] => true,
				['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3707);
			match s {
				[] => true,
				['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eppen.
pub(crate) fn n654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 622);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1787);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5693);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5694);
							match s {
								[] => true,
								['i', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7005);
									s.is_empty()
								}
								['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7006);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5695);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5696);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'f', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1788);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1789);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ippen.
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 623);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 1790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3708);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 't', 'n', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3709);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3710);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1791);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5697);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1792);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1793);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1794);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3711);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3712);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5698);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5699);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oppen.
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 624);
	match s {
		[] => true,
		['l', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 3714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 't', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uppen.
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1795);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öppen.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['d', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 1796);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -spen.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 627);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -upen.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eren.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 634);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3722);
							s.is_empty()
						}
						['n', 'e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3723);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3724);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3725);
							s.is_empty()
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 635);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3726);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3727);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5702);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7007);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3728);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3729);
							match s {
								[] => true,
								['h', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 5703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 3730);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n669(s, out),
		['u', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['q', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3786);
							s.is_empty()
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', 'h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1817);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hren.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1818);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3787);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3788);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3789);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5754);
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
						['e', s @ ..] => {
							super::push(out, 3790);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 7054);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3791);
							match s {
								[] => true,
								['u', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7055);
											match s {
												[] => true,
												['e', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 3792);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 5755);
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5756);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3793);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 7056);
											match s {
												[] => true,
												['h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 7057);
											match s {
												[] => true,
												['u', 'd', '\u{1}', s @ ..] => {
													super::push(out, 7595);
													s.is_empty()
												}
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
							super::push(out, 3794);
							match s {
								[] => true,
								['e', 'b', 'r', 'o', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3795);
							match s {
								[] => true,
								['c', s @ ..] => super::t4::n8436(s, out),
								_ => false,
							}
						}
						['l', s @ ..] => super::t4::n4887(s, out),
						['m', s @ ..] => super::t4::n4888(s, out),
						['n', s @ ..] => super::t4::n4889(s, out),
						['r', s @ ..] => super::t4::n4890(s, out),
						['s', s @ ..] => super::t4::n4891(s, out),
						['t', s @ ..] => super::t4::n4892(s, out),
						['z', s @ ..] => super::t4::n4893(s, out),
						_ => false,
					}
				}
				['w', s @ ..] => super::t4::n2082(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n673(s, out),
		['o', s @ ..] => super::t4::n674(s, out),
		['ä', s @ ..] => super::t4::n675(s, out),
		['ö', s @ ..] => {
			super::push(out, 642);
			s.is_empty()
		}
		['ü', s @ ..] => super::t4::n677(s, out),
		_ => false,
	}
}

/// Words ending -arren.
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 646);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3850);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3851);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7076);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 7598);
											match s {
												[] => true,
												['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 7599);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7077);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 7078);
									match s {
										[] => true,
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3852);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3853);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3854);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5791);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1836);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3855);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5792);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7079);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7080);
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5793);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 7600);
											match s {
												[] => true,
												['e', 'i', 'n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sperren.
pub(crate) fn n2110(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3856);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3857);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3858);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3859);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3860);
			match s {
				[] => true,
				['i', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3861);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3862);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3863);
			match s {
				[] => true,
				['n', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'z', '\u{1}', s @ ..] => {
			super::push(out, 3864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irren.
pub(crate) fn n684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 648);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1839);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 3866);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1840);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['k', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1841);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1842);
			match s {
				[] => true,
				['e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 1843);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3867);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3868);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orren.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 649);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3869);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 1844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urren.
pub(crate) fn n686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['z', s @ ..] => {
			super::push(out, 1845);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3870);
					match s {
						[] => true,
						['s', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -örren.
pub(crate) fn n687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 651);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3871);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uren.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1846);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 652);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ären.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1848);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1849);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 655);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3872);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3873);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3874);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3875);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 7081);
											match s {
												[] => true,
												['e', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 7082);
											match s {
												[] => true,
												['i', 'e', 'r', 'e', 'b', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ören.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1851);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3876);
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
				['e', s @ ..] => {
					super::push(out, 1852);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5794);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 5795);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7083);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 7084);
											match s {
												[] => true,
												['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 7085);
											match s {
												[] => true,
												['h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5796);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 5797);
									match s {
										[] => true,
										['z', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 7601);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t4::n14145(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1853);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 3877);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1854);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3878);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1855);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 3879);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3880);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1856);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3881);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 5798);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 5799);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1857);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 3882);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1858);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 3883);
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
		['k', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 659);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 660);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 661);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3884);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3885);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3886);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1859);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5801);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5802);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 5803);
									match s {
										[] => true,
										['b', s @ ..] => super::t4::n11649(s, out),
										_ => false,
									}
								}
								['n', s @ ..] => super::t4::n8541(s, out),
								['r', s @ ..] => super::t4::n8542(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üren.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1861);
					s.is_empty()
				}
				['r', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 665);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3887);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3888);
							match s {
								[] => true,
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3889);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3890);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3891);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 3892);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3893);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

