use flexion::Path;

/// Words ending -ts.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1338);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4044);
					match s {
						[] => true,
						['i', 'l', 'i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4045);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9832);
									s.is_empty()
								}
								['e', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4046);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4047);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					super::push(out, 4048);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7372);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7373);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7374);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11240);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11241);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1339);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4049);
					match s {
						[] => true,
						['r', 'o', 't', 'h', 'e', 'r', 'm', 'o', 'p', 'h', 'i', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4050);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4051);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7376);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 7377);
							match s {
								[] => true,
								['t', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 'd', s @ ..] => {
											super::push(out, 11242);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 11243);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1340);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4052);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9834);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11244);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 11918);
													s.is_empty()
												}
												['v', s @ ..] => {
													super::push(out, 11919);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 11245);
											s.is_empty()
										}
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
					super::push(out, 4053);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4054);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7378);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9835);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9836);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 7379);
							s.is_empty()
						}
						['i', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7380);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['a', 't', 'a', s @ ..] => {
					super::push(out, 4055);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1342);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4056);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4057);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 7383);
							match s {
								[] => true,
								['n', 'i', 'f', 'e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 4058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1343);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4059);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7384);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7385);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7386);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7387);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9837);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 9838);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9839);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11246);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11247);
											s.is_empty()
										}
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
					super::push(out, 4060);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7388);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4061);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7390);
							match s {
								[] => true,
								['t', 'e', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 7391);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7392);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4062);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7394);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7395);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t4::n1358(s, out),
		['y', s @ ..] => {
			super::push(out, 1345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -us.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 191);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4066);
					match s {
						[] => true,
						['e', 'l', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4067);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7399);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9843);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 9844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7400);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7401);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4068);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4069);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7403);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9845);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 9846);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 11248);
											match s {
												[] => true,
												['o', 'i', 'd', 'e', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 11920);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 9847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7404);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9848);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9849);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 9850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7405);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7406);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11249);
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t4::n14180(s, out),
														['s', s @ ..] => super::t4::n14181(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 11250);
											s.is_empty()
										}
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
					super::push(out, 4070);
					s.is_empty()
				}
				['f', s @ ..] => super::t4::n4248(s, out),
				['g', s @ ..] => {
					super::push(out, 4072);
					s.is_empty()
				}
				['i', s @ ..] => super::t4::n4250(s, out),
				['l', s @ ..] => super::t4::n4251(s, out),
				['m', s @ ..] => super::t4::n4252(s, out),
				['n', s @ ..] => super::t4::n4253(s, out),
				['o', s @ ..] => super::t4::n4254(s, out),
				['p', s @ ..] => super::t4::n4255(s, out),
				['r', s @ ..] => super::t4::n4256(s, out),
				['s', s @ ..] => super::t4::n4257(s, out),
				['t', s @ ..] => super::t4::n4258(s, out),
				['u', s @ ..] => super::t4::n4259(s, out),
				['v', s @ ..] => {
					super::push(out, 4083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t4::n1362(s, out),
		['d', s @ ..] => super::t4::n1363(s, out),
		['e', s @ ..] => super::t4::n1364(s, out),
		['f', s @ ..] => super::t4::n1365(s, out),
		['g', s @ ..] => super::t4::n1366(s, out),
		['i', s @ ..] => super::t4::n1367(s, out),
		['l', s @ ..] => super::t4::n1368(s, out),
		['m', s @ ..] => super::t4::n1369(s, out),
		['p', s @ ..] => super::t4::n1370(s, out),
		['r', s @ ..] => super::t4::n1371(s, out),
		['s', s @ ..] => super::t4::n1372(s, out),
		['t', s @ ..] => super::t4::n1373(s, out),
		_ => false,
	}
}

/// Words ending -ys.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1360);
			match s {
				[] => true,
				['v', 'a', s @ ..] => {
					super::push(out, 4119);
					match s {
						[] => true,
						['t', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1361);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4120);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4121);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4122);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4124);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7476);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tcet.
pub(crate) fn n4318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4138);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7488);
			match s {
				[] => true,
				['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 9933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -let.
pub(crate) fn n1392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1378);
	match s {
		[] => true,
		['e', 'v', 'i', 's', 'i', 'f', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4139);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -met.
pub(crate) fn n1393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1379);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 4141);
			match s {
				[] => true,
				['e', 'r', 'a', 't', s @ ..] => {
					super::push(out, 7490);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							super::push(out, 9934);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'b', s @ ..] => {
					super::push(out, 7491);
					match s {
						[] => true,
						['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -net.
pub(crate) fn n1394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1380);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4143);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4144);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4145);
			match s {
				[] => true,
				['b', 'r', 'o', s @ ..] => {
					super::push(out, 7493);
					match s {
						[] => true,
						['s', 'i', 's', s @ ..] => {
							super::push(out, 9937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'o', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4146);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4147);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7494);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9938);
							s.is_empty()
						}
						['c', 'u', 'l', 'a', 'r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 9939);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9940);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 11301);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11956);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11957);
											s.is_empty()
										}
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
					super::push(out, 7495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4148);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 7496);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11302);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7497);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', 'i', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pet.
pub(crate) fn n1395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1381);
	s.is_empty()
}

/// Words ending -ret.
pub(crate) fn n1396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1382);
	s.is_empty()
}

/// Words ending -set.
pub(crate) fn n1397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1383);
	match s {
		[] => true,
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7498);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							super::push(out, 9941);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11304);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7499);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tet.
pub(crate) fn n1398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1384);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4149);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
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
				['a', s @ ..] => {
					super::push(out, 7501);
					match s {
						[] => true,
						['g', 'o', 'n', 'o', 'l', 'o', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9943);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xet.
pub(crate) fn n1399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1385);
	match s {
		[] => true,
		['e', 'n', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7502);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ht.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1386);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1387);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4150);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4151);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4152);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4153);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7503);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7504);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4154);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7506);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4155);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7508);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1388);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					super::push(out, 4156);
					match s {
						[] => true,
						['c', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1389);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4157);
					match s {
						[] => true,
						['c', 'i', 'u', s @ ..] => {
							super::push(out, 7511);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 7512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1390);
			match s {
				[] => true,
				['r', 'i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7513);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1391);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4159);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4160);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -it.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1392);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4161);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4162);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1393);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1394);
			match s {
				[] => true,
				['i', 'a', 'g', s @ ..] => {
					super::push(out, 4164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1395);
			match s {
				[] => true,
				['o', 'r', 'a', s @ ..] => {
					super::push(out, 4165);
					match s {
						[] => true,
						['t', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1396);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4166);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4167);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 7516);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11306);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11307);
											s.is_empty()
										}
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
					super::push(out, 4168);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4169);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9946);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9947);
									s.is_empty()
								}
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
			super::push(out, 1397);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4170);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7517);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4172);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4173);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7519);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7520);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4174);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ot.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4175);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1400);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4177);
					match s {
						[] => true,
						['r', 'a', 'b', s @ ..] => {
							super::push(out, 7522);
							match s {
								[] => true,
								['i', 'l', 'i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 11308);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11309);
											s.is_empty()
										}
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
					super::push(out, 4178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1401);
			match s {
				[] => true,
				['e', 'n', 't', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1402);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4180);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4181);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4182);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4183);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4185);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9948);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9949);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 11310);
											s.is_empty()
										}
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
					super::push(out, 4186);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 7523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1404);
			match s {
				[] => true,
				['u', 'p', 'l', 'e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 4188);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4189);
					match s {
						[] => true,
						['e', 'c', s @ ..] => {
							super::push(out, 7524);
							match s {
								[] => true,
								['u', 'l', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9950);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4190);
					match s {
						[] => true,
						['t', 'a', 'b', s @ ..] => {
							super::push(out, 7525);
							match s {
								[] => true,
								['i', 'l', 'i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 11311);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11312);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4191);
					match s {
						[] => true,
						['u', 'c', 'e', s @ ..] => {
							super::push(out, 7526);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11313);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11314);
											s.is_empty()
										}
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
					super::push(out, 4192);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4193);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4194);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4195);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7527);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4196);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4197);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7529);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9951);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9952);
									s.is_empty()
								}
								_ => false,
							}
						}
						['q', s @ ..] => {
							super::push(out, 7530);
							s.is_empty()
						}
						['s', s @ ..] => super::t4::n8156(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4198);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4199);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4200);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7532);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1406);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4201);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7534);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4202);
					match s {
						[] => true,
						['e', 'n', 'n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4203);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9964);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n1421(s, out),
		['o', s @ ..] => {
			super::push(out, 1408);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4220);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9980);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => super::t4::n4406(s, out),
				['i', s @ ..] => super::t4::n4407(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t4::n1423(s, out),
		_ => false,
	}
}

/// Words ending -st.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 199);
	s.is_empty()
}

/// Words ending -ut.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1410);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4227);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4228);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1411);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4230);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7558);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4231);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1412);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4233);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1413);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4235);
					match s {
						[] => true,
						['n', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 4236);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 7561);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11332);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11333);
											s.is_empty()
										}
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
					super::push(out, 4237);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 7562);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4238);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7564);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9982);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 9983);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7565);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1414);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4239);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					super::push(out, 4240);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1415);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4242);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7566);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4243);
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wt.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	s.is_empty()
}

/// Words ending -yt.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['p', 'h', s @ ..] => {
			super::push(out, 1416);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1417);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word surutarebu.
pub(crate) fn n13870(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -dnu.
pub(crate) fn n1444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1430);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4256);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7579);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7580);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 7581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4257);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4258);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4259);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inu.
pub(crate) fn n1446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1432);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4260);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4261);
			match s {
				[] => true,
				['i', 'n', 'e', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4262);
			match s {
				[] => true,
				['r', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7583);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'i', s @ ..] => {
			super::push(out, 4263);
			match s {
				[] => true,
				['t', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ru.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1435);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1436);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4265);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9991);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9992);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['a', 'l', 'y', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4267);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4268);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4269);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7586);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1438);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1439);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7588);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7589);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11337);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11338);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1440);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1441);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1442);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -su.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1443);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1444);
			match s {
				[] => true,
				['t', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7590);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 's', s @ ..] => {
									super::push(out, 9994);
									s.is_empty()
								}
								['u', 'r', s @ ..] => {
									super::push(out, 9995);
									s.is_empty()
								}
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
			super::push(out, 1445);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1446);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7591);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1447);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4270);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7593);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4271);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4272);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4273);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tu.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1448);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1449);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4274);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 9996);
									match s {
										[] => true,
										['l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 11339);
											s.is_empty()
										}
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
					super::push(out, 4275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1450);
			match s {
				[] => true,
				['l', 'i', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rutallav.
pub(crate) fn n13871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11968);
	s.is_empty()
}

/// Words ending -sutallav.
pub(crate) fn n13872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11969);
	s.is_empty()
}

/// Words ending -rav.
pub(crate) fn n1473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1459);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7611);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10003);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10004);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 10005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7612);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10006);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4297);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sav.
pub(crate) fn n1474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1460);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4298);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7614);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10008);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4299);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10010);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10011);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10012);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 11342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -atav.
pub(crate) fn n4494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4300);
	s.is_empty()
}

/// Words ending -itav.
pub(crate) fn n4495(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7616);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10013);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7617);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ev.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4301);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4302);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7618);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10015);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 10016);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7619);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4303);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1464);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1465);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4305);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7621);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7622);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11343);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11344);
											s.is_empty()
										}
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
					super::push(out, 4306);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7623);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7624);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7625);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10017);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4307);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4308);
					match s {
						[] => true,
						['c', 'i', 'o', s @ ..] => {
							super::push(out, 7626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 'i', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'e', 'n', 's', '\u{1}', s @ ..] => {
			super::push(out, 1466);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1467);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4310);
					match s {
						[] => true,
						['l', 'i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10019);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7627);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10021);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11345);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11346);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4311);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4312);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7628);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7629);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7630);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									super::push(out, 10022);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11347);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11348);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7631);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 7632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4313);
					match s {
						[] => true,
						['a', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 7633);
							s.is_empty()
						}
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10023);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 10024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7634);
							s.is_empty()
						}
						['u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4314);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4315);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4316);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 7636);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7637);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1468);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1469);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4317);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4318);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4319);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7639);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4320);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4321);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7641);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7642);
							match s {
								[] => true,
								['i', 'm', 'i', 'l', 'i', 's', s @ ..] => {
									super::push(out, 10025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4322);
					s.is_empty()
				}
				['n', s @ ..] => super::t4::n4518(s, out),
				['o', s @ ..] => {
					super::push(out, 4324);
					s.is_empty()
				}
				['r', s @ ..] => super::t4::n4520(s, out),
				['s', s @ ..] => super::t4::n4521(s, out),
				['t', s @ ..] => super::t4::n4522(s, out),
				['u', s @ ..] => {
					super::push(out, 4328);
					match s {
						[] => true,
						['s', s @ ..] => super::t4::n8302(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t4::n4524(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t4::n1484(s, out),
		['t', s @ ..] => super::t4::n1485(s, out),
		['x', s @ ..] => super::t4::n1486(s, out),
		_ => false,
	}
}

/// Words ending -iv.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1473);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4339);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4340);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7657);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7658);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10041);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10042);
									s.is_empty()
								}
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
			super::push(out, 1475);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4341);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7659);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4342);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7661);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7662);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4343);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10043);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4344);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7664);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7665);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7666);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10045);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1476);
			match s {
				[] => true,
				['u', 'a', 't', s @ ..] => {
					super::push(out, 4345);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10047);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10048);
									s.is_empty()
								}
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
			super::push(out, 1477);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4346);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4347);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7667);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11355);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 11356);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4348);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7669);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 10049);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10050);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 7670);
							match s {
								[] => true,
								['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10051);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4349);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
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
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4350);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10052);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10053);
									s.is_empty()
								}
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
			super::push(out, 1479);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4351);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4352);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7673);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10054);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10055);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7674);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1480);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1481);
			match s {
				[] => true,
				['a', s @ ..] => super::t4::n4549(s, out),
				['c', s @ ..] => super::t4::n4550(s, out),
				['d', s @ ..] => super::t4::n4551(s, out),
				['e', s @ ..] => super::t4::n4552(s, out),
				['i', s @ ..] => super::t4::n4553(s, out),
				['n', s @ ..] => super::t4::n4554(s, out),
				['o', s @ ..] => super::t4::n4555(s, out),
				['u', 'l', 'e', 'n', 't', 'u', 's', '\u{1}', s @ ..] => super::t4::n4556(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n1496(s, out),
		['p', s @ ..] => super::t4::n1497(s, out),
		['r', s @ ..] => super::t4::n1498(s, out),
		['s', s @ ..] => super::t4::n1499(s, out),
		['t', s @ ..] => super::t4::n1500(s, out),
		['v', s @ ..] => super::t4::n1501(s, out),
		_ => false,
	}
}

/// Words ending -ov.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1488);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4378);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7701);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7702);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7703);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7704);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 10065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4379);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7705);
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11358);
											s.is_empty()
										}
										['a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', 'u', 's', '\u{1}', s @ ..] => super::t4::n14211(s, out),
														['s', '\u{1}', s @ ..] => super::t4::n14212(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', 'u', 's', '\u{1}', s @ ..] => super::t4::n14213(s, out),
														['s', '\u{1}', s @ ..] => super::t4::n14214(s, out),
														_ => false,
													}
												}
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
							super::push(out, 7706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1489);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4381);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7707);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7708);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10066);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 10067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4382);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4383);
					match s {
						[] => true,
						['a', 't', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4384);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4385);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7710);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 7711);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7712);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7713);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7714);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10068);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 10069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1490);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1491);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7715);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7716);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1492);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1493);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uv.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4387);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10070);
									s.is_empty()
								}
								['r', 'i', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11359);
											match s {
												[] => true,
												['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 's', s @ ..] => {
											super::push(out, 11360);
											s.is_empty()
										}
										['u', 'r', s @ ..] => {
											super::push(out, 11361);
											s.is_empty()
										}
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
					super::push(out, 4388);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4389);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 11978);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 11979);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 11362);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4390);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eca.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1559);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1560);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1561);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4457);
					s.is_empty()
				}
				['a', 't', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4458);
					s.is_empty()
				}
				['i', 'n', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4459);
					match s {
						[] => true,
						['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4460);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4461);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1562);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1563);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4462);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4463);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7763);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 7764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hca.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1564);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4464);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4465);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4466);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1565);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1566);
			match s {
				[] => true,
				['l', 'l', 'e', 'i', s @ ..] => {
					super::push(out, 4468);
					match s {
						[] => true,
						['f', 'o', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1567);
			match s {
				[] => true,
				['o', 'm', 'i', 'a', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 4469);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ica.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1568);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1569);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1570);
			s.is_empty()
		}
		['n', 'a', s @ ..] => {
			super::push(out, 1571);
			match s {
				[] => true,
				['c', 'i', 'f', 'o', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4470);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mca.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4471);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4472);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qca.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['u', 'i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4473);
					match s {
						[] => true,
						['s', 'c', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 7766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7767);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rca.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1572);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4475);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1573);
			match s {
				[] => true,
				['c', 'y', s @ ..] => {
					super::push(out, 4477);
					match s {
						[] => true,
						['a', 'n', 'e', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4478);
					match s {
						[] => true,
						['e', 'm', 'a', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1574);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tca.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1575);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1576);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4479);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4480);
					match s {
						[] => true,
						['o', 'p', 'e', 't', 'a', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4481);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1577);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 4483);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10111);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uca.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1578);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1579);
			match s {
				[] => true,
				['e', 'a', 't', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7772);
							match s {
								[] => true,
								['c', 'a', 'r', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1580);
			match s {
				[] => true,
				['i', 'n', 'a', 't', 'i', s @ ..] => {
					super::push(out, 4484);
					match s {
						[] => true,
						['f', 'o', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1581);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sudnasnedda.
pub(crate) fn n13890(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word snasnedda.
pub(crate) fn n13891(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word surutasnedda.
pub(crate) fn n13892(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word sutasnedda.
pub(crate) fn n13893(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -afda.
pub(crate) fn n1624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1605);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4499);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4500);
			s.is_empty()
		}
		['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -efda.
pub(crate) fn n1625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1606);
	s.is_empty()
}

/// Words ending -ifda.
pub(crate) fn n1626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1607);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 4502);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7790);
					s.is_empty()
				}
				['u', 'r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10118);
							s.is_empty()
						}
						['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4503);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lfda.
pub(crate) fn n1627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1608);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4504);
			match s {
				[] => true,
				['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4505);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4506);
			match s {
				[] => true,
				['c', 't', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10120);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 7792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4507);
			match s {
				[] => true,
				['x', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rfda.
pub(crate) fn n1628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1609);
	s.is_empty()
}

/// Words ending -ufda.
pub(crate) fn n1629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1610);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 4508);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10122);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gda.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 4509);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1612);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4511);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 4513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hda.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1616);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4514);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4515);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1617);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4517);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4518);
					match s {
						[] => true,
						['u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ida.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1620);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1621);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1622);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1623);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1624);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1625);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1626);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4519);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4520);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1627);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4522);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1628);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 4524);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1629);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4526);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4527);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4528);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4529);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7795);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4530);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10124);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 11370);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11371);
											s.is_empty()
										}
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
					super::push(out, 4531);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7797);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7798);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10125);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4532);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lda.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1630);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4533);
					match s {
						[] => true,
						['o', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10127);
									s.is_empty()
								}
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4534);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1631);
			match s {
				[] => true,
				['c', 't', 'a', s @ ..] => {
					super::push(out, 4536);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7800);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11372);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 11373);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11374);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 11375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7802);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1632);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4537);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 4538);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4539);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1633);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1634);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4540);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4541);
					s.is_empty()
				}
				['e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 4542);
					s.is_empty()
				}
				['s', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4543);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mda.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1635);
			match s {
				[] => true,
				['t', 'u', 'r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10129);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10131);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10132);
									s.is_empty()
								}
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
			super::push(out, 1636);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1637);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4544);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4545);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 7804);
							match s {
								[] => true,
								['i', 'l', 'i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 11376);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11377);
											s.is_empty()
										}
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
					super::push(out, 4546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1638);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4547);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10133);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10134);
									s.is_empty()
								}
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
			super::push(out, 1639);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nda.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4548);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7805);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1641);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 4550);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7807);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4551);
					s.is_empty()
				}
				['x', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1643);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4553);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1644);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4555);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4556);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4557);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4558);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4559);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7809);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 7810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oda.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1645);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7811);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1646);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4560);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4561);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10135);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1647);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4563);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7813);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7814);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10137);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10138);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4564);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4565);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4566);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1648);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pda.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1649);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7815);
							s.is_empty()
						}
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10139);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 10140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10141);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 10142);
									s.is_empty()
								}
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
			super::push(out, 1650);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4568);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7816);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1651);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1652);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['o', 'n', s @ ..] => {
					super::push(out, 4570);
					match s {
						[] => true,
						['a', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 7818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qda.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['u', 'i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4571);
					s.is_empty()
				}
				['r', 'e', 'n', 'd', s @ ..] => {
					super::push(out, 4572);
					s.is_empty()
				}
				['s', 'i', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7819);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rda.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1654);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1655);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4573);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4574);
					s.is_empty()
				}
				['s', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1656);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sda.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1657);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4576);
					match s {
						[] => true,
						['n', 's', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4577);
					match s {
						[] => true,
						['s', 'c', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10143);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 10144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10145);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4578);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'e', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10147);
									s.is_empty()
								}
								['p', 't', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10148);
									s.is_empty()
								}
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
			super::push(out, 1658);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4579);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7822);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 4580);
					match s {
						[] => true,
						['e', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10149);
									s.is_empty()
								}
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10150);
									s.is_empty()
								}
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
			super::push(out, 1659);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4581);
					match s {
						[] => true,
						['i', 'l', 'a', 'n', 's', '\u{1}', s @ ..] => {
							super::push(out, 7824);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1660);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1661);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4582);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4583);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7825);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', '\u{1}', s @ ..] => {
									super::push(out, 10151);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10152);
									s.is_empty()
								}
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
			super::push(out, 1662);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1663);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4584);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7826);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7827);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7828);
							s.is_empty()
						}
						['t', 'u', 's', s @ ..] => {
							super::push(out, 7829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4585);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4586);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4587);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4588);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10153);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tda.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1664);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1665);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1666);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1667);
			match s {
				[] => true,
				['n', 's', s @ ..] => {
					super::push(out, 4589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1668);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7830);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7831);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11378);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 11379);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uda.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1669);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4590);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7832);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7833);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 10155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4591);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4592);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
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
								['i', 'n', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1670);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1671);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4593);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7834);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1672);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1673);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4595);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1674);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vda.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1675);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4597);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4598);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1676);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 4600);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 4601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1677);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zda.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	s.is_empty()
}

/// The word sudnarapiuqea.
pub(crate) fn n13898(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word snarapiuqea.
pub(crate) fn n13899(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word surutarapiuqea.
pub(crate) fn n13900(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word sutarapiuqea.
pub(crate) fn n13901(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -sea.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1688);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1689);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4619);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 7841);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7842);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 11385);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 11386);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11387);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4620);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10163);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10164);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10165);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dnarugiffa.
pub(crate) fn n13902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11980);
	s.is_empty()
}

/// Words ending -snarugiffa.
pub(crate) fn n13903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11981);
	s.is_empty()
}

/// Words ending -rutarugiffa.
pub(crate) fn n13904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11982);
	s.is_empty()
}

/// Words ending -sutarugiffa.
pub(crate) fn n13905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11983);
	s.is_empty()
}

/// Words ending -balla.
pub(crate) fn n4889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4657);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7867);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7868);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11984);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11985);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11986);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11987);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -talla.
pub(crate) fn n4892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4660);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 7869);
			s.is_empty()
		}
		['u', 's', s @ ..] => {
			super::push(out, 7870);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ualla.
pub(crate) fn n4893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4661);
	s.is_empty()
}

/// Words ending -ella.
pub(crate) fn n1760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1736);
	match s {
		[] => true,
		['c', 't', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7871);
					s.is_empty()
				}
				['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7872);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10169);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 10170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4662);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7873);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7874);
					s.is_empty()
				}
				['h', 'a', 'n', 'i', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 7875);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'i', s @ ..] => {
			super::push(out, 4663);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -illa.
pub(crate) fn n1761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1737);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4664);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4665);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4666);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4667);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10171);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olla.
pub(crate) fn n1762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1738);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4668);
			match s {
				[] => true,
				['i', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 7877);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ulla.
pub(crate) fn n1763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1739);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4669);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 4670);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4671);
			s.is_empty()
		}
		['s', 'u', 's', s @ ..] => {
			super::push(out, 4672);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 4673);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pla.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['e', 's', 't', 'r', 'u', s @ ..] => {
			super::push(out, 1740);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 4674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'n', 'a', s @ ..] => {
			super::push(out, 1741);
			match s {
				[] => true,
				['l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 4675);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sla.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	s.is_empty()
}

/// Words ending -tla.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	s.is_empty()
}

/// Words ending -ula.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1742);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1743);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vla.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 1744);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dnecidetna.
pub(crate) fn n11448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10177);
	s.is_empty()
}

/// Words ending -tcidetna.
pub(crate) fn n11449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10178);
	s.is_empty()
}

/// Words ending -petna.
pub(crate) fn n4961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4725);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7901);
			match s {
				[] => true,
				['n', 'u', 'l', 't', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 10179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 10180);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11988);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10181);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -itna.
pub(crate) fn n1816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1790);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4726);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4727);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4728);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 4729);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4730);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -una.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	s.is_empty()
}

/// Words ending -xna.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4731);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7902);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 7903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4732);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4733);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -assa.
pub(crate) fn n1887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1856);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 4785);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4786);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4787);
			s.is_empty()
		}
		['t', 'u', 'r', s @ ..] => {
			super::push(out, 4788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -essa.
pub(crate) fn n1888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1857);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4789);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4790);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 7942);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4791);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 4792);
			match s {
				[] => true,
				['e', 'r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11398);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11400);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -issa.
pub(crate) fn n1889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1858);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4793);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ossa.
pub(crate) fn n1890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1859);
	s.is_empty()
}

/// Words ending -ussa.
pub(crate) fn n1891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1860);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4794);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7944);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7945);
					s.is_empty()
				}
				['t', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4795);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4796);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4797);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4798);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4799);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10196);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tsa.
pub(crate) fn n362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1861);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1862);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1863);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4800);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7947);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7948);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11402);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11403);
											s.is_empty()
										}
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
					super::push(out, 4801);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4802);
					match s {
						[] => true,
						['c', 't', 'u', 'r', s @ ..] => {
							super::push(out, 7949);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1864);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 4803);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4804);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4805);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7951);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 7952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ysa.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['m', 'm', 'e', 't', 'r', 'i', s @ ..] => {
			super::push(out, 1865);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dnanoitcua.
pub(crate) fn n13912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11990);
	s.is_empty()
}

/// Words ending -snanoitcua.
pub(crate) fn n13913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11991);
	s.is_empty()
}

/// Words ending -rutanoitcua.
pub(crate) fn n13914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11992);
	s.is_empty()
}

/// Words ending -sutanoitcua.
pub(crate) fn n13915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11993);
	s.is_empty()
}

/// Words ending -irua.
pub(crate) fn n1918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1886);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4836);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 4837);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4838);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7965);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orua.
pub(crate) fn n1919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1887);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 4839);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urua.
pub(crate) fn n1920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1888);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 4841);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4842);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sua.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1889);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1890);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1891);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4843);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7967);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', 's', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11409);
											s.is_empty()
										}
										['i', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 11994);
													s.is_empty()
												}
												['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 7968);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7969);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10207);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xua.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	s.is_empty()
}

/// The word sudnaregilleb.
pub(crate) fn n13918(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word snaregilleb.
pub(crate) fn n13919(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word surutaregilleb.
pub(crate) fn n13920(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word sutaregilleb.
pub(crate) fn n13921(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rec.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2048);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4973);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4974);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4975);
					match s {
						[] => true,
						['o', 'c', 'a', 'r', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8048);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10243);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10244);
									s.is_empty()
								}
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
			super::push(out, 2049);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4976);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4977);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4978);
					s.is_empty()
				}
				['v', 'i', 's', 'i', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 4979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2050);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 4980);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2052);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8049);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2053);
			match s {
				[] => true,
				['l', 'e', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 4983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 2054);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					super::push(out, 4984);
					match s {
						[] => true,
						['o', 'r', 'n', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8051);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word sopracosyrhc.
pub(crate) fn n13924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11997);
	s.is_empty()
}

/// The word supracosyrhc.
pub(crate) fn n13925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11998);
	s.is_empty()
}

/// Words ending -dnagivanmucric.
pub(crate) fn n13926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11999);
	s.is_empty()
}

/// Words ending -rutagivanmucric.
pub(crate) fn n13927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12000);
	s.is_empty()
}

/// Words ending -tallavmucric.
pub(crate) fn n13928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12001);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12258);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12259);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lloc.
pub(crate) fn n2166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2120);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5032);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8079);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10259);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10260);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 12002);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 12209);
													s.is_empty()
												}
												['u', s @ ..] => {
													match s {
														[] => true,
														['r', s @ ..] => super::t4::n14300(s, out),
														['s', s @ ..] => super::t4::n14301(s, out),
														_ => false,
													}
												}
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
							super::push(out, 10261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'u', 'r', s @ ..] => {
					super::push(out, 8080);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8081);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10262);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 10263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5033);
			match s {
				[] => true,
				['c', 't', 'u', 's', s @ ..] => {
					super::push(out, 8083);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5034);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8085);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11432);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11433);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8086);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8087);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11434);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 8088);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8089);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11436);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11437);
									s.is_empty()
								}
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
			super::push(out, 5035);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5036);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10264);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8090);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -moc.
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 484);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2123);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5041);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8093);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8094);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11438);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11439);
											s.is_empty()
										}
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
					super::push(out, 5042);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8095);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11440);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11441);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2124);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5043);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5044);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5045);
					match s {
						[] => true,
						['s', 'a', 'b', 'u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8097);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10266);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10267);
									s.is_empty()
								}
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
			super::push(out, 2125);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5046);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5047);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5048);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2126);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5050);
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 8098);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11442);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11443);
											s.is_empty()
										}
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
					super::push(out, 5051);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8099);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8100);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10268);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10269);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12003);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12004);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8101);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8102);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11444);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11445);
											s.is_empty()
										}
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
					super::push(out, 5052);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8103);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10271);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 10272);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 10273);
									match s {
										[] => true,
										['s', 'c', 'e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12005);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12006);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10274);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5053);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8105);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8106);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8107);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10275);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 10276);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10277);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8108);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8109);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => super::t4::n13146(s, out),
										['s', s @ ..] => super::t4::n13147(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t4::n5337(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n2173(s, out),
		['p', s @ ..] => super::t4::n2174(s, out),
		_ => false,
	}
}

/// Words ending -noc.
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 485);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2129);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5065);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5066);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8137);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8138);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2130);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8139);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11462);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11463);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2131);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5067);
					match s {
						[] => true,
						['t', 'e', 'n', 'a', 't', s @ ..] => {
							super::push(out, 8141);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11464);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11465);
											s.is_empty()
										}
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
					super::push(out, 5068);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8142);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11466);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11467);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 8143);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10301);
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11468);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11469);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 10302);
									match s {
										[] => true,
										['b', 'r', 'a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['d', s @ ..] => super::t4::n14229(s, out),
														['s', s @ ..] => super::t4::n14230(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													match s {
														[] => true,
														['r', s @ ..] => super::t4::n14231(s, out),
														['s', s @ ..] => super::t4::n14232(s, out),
														_ => false,
													}
												}
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
							super::push(out, 8144);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8145);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 5069);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8147);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 8148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5070);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8149);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11470);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11471);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8150);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8151);
							match s {
								[] => true,
								['u', 'r', s @ ..] => {
									super::push(out, 10303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5071);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8152);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12015);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12016);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12017);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12018);
													s.is_empty()
												}
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
							super::push(out, 8153);
							match s {
								[] => true,
								['d', 'e', 'n', 'd', s @ ..] => {
									super::push(out, 10304);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5072);
					s.is_empty()
				}
				['r', s @ ..] => super::t4::n5358(s, out),
				['u', s @ ..] => super::t4::n5359(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t4::n2178(s, out),
		['e', s @ ..] => super::t4::n2179(s, out),
		['f', s @ ..] => super::t4::n2180(s, out),
		['g', s @ ..] => super::t4::n2181(s, out),
		['i', s @ ..] => super::t4::n2182(s, out),
		['l', s @ ..] => super::t4::n2183(s, out),
		['m', s @ ..] => super::t4::n2184(s, out),
		['n', s @ ..] => super::t4::n2185(s, out),
		['o', 'i', 'd', 'e', 'u', 's', '\u{1}', s @ ..] => super::t4::n2186(s, out),
		['p', s @ ..] => super::t4::n2187(s, out),
		['q', s @ ..] => super::t4::n2188(s, out),
		['r', s @ ..] => super::t4::n2189(s, out),
		['s', s @ ..] => super::t4::n2190(s, out),
		['t', s @ ..] => super::t4::n2191(s, out),
		['u', s @ ..] => super::t4::n2192(s, out),
		['v', s @ ..] => super::t4::n2193(s, out),
		_ => false,
	}
}

/// Words ending -uqoc.
pub(crate) fn n2197(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5141);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5142);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11544);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -roc.
pub(crate) fn n494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2151);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2152);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2153);
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8284);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2154);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5143);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8286);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2156);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2157);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2158);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5144);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8288);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11546);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12065);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12066);
													s.is_empty()
												}
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
							super::push(out, 8289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5145);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5146);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8290);
							s.is_empty()
						}
						['d', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10406);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 8291);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10408);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11547);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11548);
											s.is_empty()
										}
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
					super::push(out, 5147);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 8292);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11549);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11550);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 8293);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8294);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2159);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2160);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8295);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2161);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itoc.
pub(crate) fn n2210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2163);
	s.is_empty()
}

/// Words ending -otoc.
pub(crate) fn n2211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2164);
	s.is_empty()
}

/// Words ending -ttoc.
pub(crate) fn n2212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2165);
	s.is_empty()
}

/// Words ending -uoc.
pub(crate) fn n497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 492);
	s.is_empty()
}

/// Words ending -xoc.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 493);
	match s {
		[] => true,
		['a', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssiledurc.
pub(crate) fn n13302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11555);
	match s {
		[] => true,
		['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sedurc.
pub(crate) fn n9098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8309);
	s.is_empty()
}

/// Words ending -idurc.
pub(crate) fn n5455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5163);
	s.is_empty()
}

/// Words ending -eurc.
pub(crate) fn n2236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2189);
	s.is_empty()
}

/// The word silarurc.
pub(crate) fn n2237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2190);
	s.is_empty()
}

/// Words ending -surc.
pub(crate) fn n2238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2191);
	s.is_empty()
}

/// Words ending -yrc.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 499);
	s.is_empty()
}

/// Words ending -loced.
pub(crate) fn n5498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5198);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 8331);
			s.is_empty()
		}
		['o', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 10433);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12067);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 12068);
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
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12069);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 12070);
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

/// Words ending -qoced.
pub(crate) fn n5499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5199);
	s.is_empty()
}

/// Words ending -roced.
pub(crate) fn n5500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5200);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8332);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8333);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rced.
pub(crate) fn n2282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2231);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 8335);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8336);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10434);
							s.is_empty()
						}
						['o', 'r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10435);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11562);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uced.
pub(crate) fn n2283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2232);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8337);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 8338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5201);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8339);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8340);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 's', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8342);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8343);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ded.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 533);
	match s {
		[] => true,
		['e', 'c', 'o', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2233);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2234);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2235);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8344);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fed.
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 534);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5202);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5203);
					match s {
						[] => true,
						['i', 'g', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11564);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11565);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11566);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11567);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2237);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5204);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5205);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8346);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5206);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5207);
					match s {
						[] => true,
						['i', 'g', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'd', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10436);
									s.is_empty()
								}
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10437);
									s.is_empty()
								}
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
			super::push(out, 2238);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5208);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5209);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 8348);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8350);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2239);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5210);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5211);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 8352);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11568);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11569);
											s.is_empty()
										}
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
					super::push(out, 5212);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5213);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8353);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 8354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2240);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5214);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5215);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2241);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5217);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8355);
							match s {
								[] => true,
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11570);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11571);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5218);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2242);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5220);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5221);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8357);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ged.
pub(crate) fn n540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 535);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2243);
			match s {
				[] => true,
				['n', 'e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2244);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['b', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 8359);
							s.is_empty()
						}
						['p', 't', 'u', 'r', s @ ..] => {
							super::push(out, 8360);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hed.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 536);
	s.is_empty()
}

/// Words ending -ied.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 537);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2245);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2246);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5223);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2247);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2248);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2249);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2250);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2251);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5225);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5226);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8362);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5227);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5228);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -led.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 538);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2252);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5229);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5230);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5231);
					s.is_empty()
				}
				['t', 'u', 's', s @ ..] => {
					super::push(out, 5232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2253);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5233);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10438);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10439);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5234);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8364);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8365);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', 's', s @ ..] => {
					super::push(out, 5235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2254);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2255);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5236);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8367);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8368);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8369);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5237);
					match s {
						[] => true,
						['a', 't', 'i', s @ ..] => {
							super::push(out, 8371);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10440);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5238);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5239);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8372);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10442);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 8373);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8374);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 8375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 5240);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5241);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8376);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8377);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5242);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2256);
			match s {
				[] => true,
				['h', 'i', 'n', 'a', 'l', s @ ..] => {
					super::push(out, 5243);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2257);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2258);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5244);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5245);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8379);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -med.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2259);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2260);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8381);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10444);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10445);
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
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
						['t', s @ ..] => {
							super::push(out, 8382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5246);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5247);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8383);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5248);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5249);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5250);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2262);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5252);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5253);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5254);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8385);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10446);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2263);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2264);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5256);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5257);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ned.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2265);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5258);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8386);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2266);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2267);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2268);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5259);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2269);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8388);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8389);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11572);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11573);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2270);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oed.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 2272);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ped.
pub(crate) fn n547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2274);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5261);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5262);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5263);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2275);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5265);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8390);
							match s {
								[] => true,
								['s', 'c', 'e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11574);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8391);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8392);
							match s {
								[] => true,
								['l', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12071);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12072);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12073);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12074);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5266);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8393);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2276);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2277);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5268);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8395);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5269);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5270);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10448);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10449);
									s.is_empty()
								}
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
			super::push(out, 2278);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5271);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5272);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5273);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5274);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2279);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5276);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8397);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 8398);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11576);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11577);
											s.is_empty()
										}
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
					super::push(out, 5277);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8399);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 8400);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8401);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10450);
									s.is_empty()
								}
								['s', 'u', 's', s @ ..] => {
									super::push(out, 10451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5278);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5279);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', 's', s @ ..] => {
									super::push(out, 10452);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 10453);
									s.is_empty()
								}
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
			super::push(out, 2280);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2281);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -red.
pub(crate) fn n548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2282);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5280);
					s.is_empty()
				}
				['s', 'u', 's', s @ ..] => {
					super::push(out, 5281);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2283);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2284);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5282);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5283);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2285);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5285);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5286);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5287);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5288);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2286);
			match s {
				[] => true,
				['g', 'a', 't', 'o', s @ ..] => {
					super::push(out, 5290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sed.
pub(crate) fn n549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 544);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2288);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5291);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2289);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5293);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10454);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10455);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 11578);
											s.is_empty()
										}
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
					super::push(out, 5294);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5295);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12075);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12076);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 11579);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 't', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11580);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11581);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2290);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2291);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5296);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5297);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8402);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10456);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8403);
							match s {
								[] => true,
								['o', 's', 'i', 's', s @ ..] => {
									super::push(out, 10458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5298);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5299);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5300);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5301);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2292);
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8404);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10459);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10460);
									s.is_empty()
								}
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
			super::push(out, 2293);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5302);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8405);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8406);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 11582);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12077);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12078);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 11583);
											match s {
												[] => true,
												['i', 's', s @ ..] => {
													super::push(out, 12079);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 12080);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5303);
					match s {
						[] => true,
						['c', 'a', 't', 'i', s @ ..] => {
							super::push(out, 8407);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10461);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5304);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8408);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8409);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10463);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10464);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 11584);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11585);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12081);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12082);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5305);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8410);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8411);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11586);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11587);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2294);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5306);
					s.is_empty()
				}
				['i', s @ ..] => super::t4::n5626(s, out),
				['r', s @ ..] => super::t4::n5627(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t4::n2347(s, out),
		_ => false,
	}
}

/// Words ending -ted.
pub(crate) fn n550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2296);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5313);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2297);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5315);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8417);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 8418);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8419);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2298);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5317);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5318);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5319);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8421);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10467);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5320);
					match s {
						[] => true,
						['d', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10469);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10471);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10472);
									s.is_empty()
								}
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
			super::push(out, 2299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ued.
pub(crate) fn n551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 546);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2300);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2301);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 5321);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ved.
pub(crate) fn n552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 547);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2303);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2304);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5323);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8422);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5324);
					s.is_empty()
				}
				['n', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8424);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8425);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5325);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2306);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 5327);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10473);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xed.
pub(crate) fn n553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 548);
	s.is_empty()
}

/// Words ending -gnuiid.
pub(crate) fn n9260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8439);
	s.is_empty()
}

/// Words ending -lid.
pub(crate) fn n562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 557);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2327);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5354);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5355);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5356);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8440);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5357);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5358);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8442);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 8443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2328);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2329);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2330);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5359);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5360);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8444);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 8445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mid.
pub(crate) fn n563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 558);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2331);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2332);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5361);
					s.is_empty()
				}
				['t', 'i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8446);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2333);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2334);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5362);
					match s {
						[] => true,
						['p', 'h', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5363);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10486);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5364);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nid.
pub(crate) fn n564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 559);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2335);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 5365);
					s.is_empty()
				}
				['t', 'u', 's', s @ ..] => {
					super::push(out, 5366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2336);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oid.
pub(crate) fn n565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['n', 'y', 's', 'i', 'a', 'c', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 2337);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pid.
pub(crate) fn n566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2338);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2339);
			match s {
				[] => true,
				['o', 'p', 'o', 'd', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 5367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rid.
pub(crate) fn n567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 562);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2340);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5368);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5369);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2341);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5371);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5372);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5373);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5374);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5375);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2342);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sid.
pub(crate) fn n568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 563);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2343);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5377);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5378);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8449);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11589);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11590);
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
								['d', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['i', s @ ..] => {
					super::push(out, 5379);
					match s {
						[] => true,
						['d', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 8450);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10488);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 10489);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 10490);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 10491);
									s.is_empty()
								}
								['u', 'r', s @ ..] => {
									super::push(out, 10492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5380);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['d', 'e', 'n', 'd', s @ ..] => {
									super::push(out, 10493);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5381);
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 8452);
							match s {
								[] => true,
								['o', 'l', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5382);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8453);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8454);
							match s {
								[] => true,
								['b', 'e', 'n', 's', s @ ..] => {
									super::push(out, 10496);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 10497);
									s.is_empty()
								}
								['p', 't', 'u', 's', s @ ..] => {
									super::push(out, 10498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2344);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5384);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2345);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2346);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5386);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5387);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5388);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5389);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10499);
									match s {
										[] => true,
										['t', 'i', s @ ..] => {
											super::push(out, 11591);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 10500);
									s.is_empty()
								}
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
			super::push(out, 2347);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5390);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
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
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => super::t4::n11893(s, out),
								['t', 'i', s @ ..] => super::t4::n11894(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t4::n5715(s, out),
				['l', s @ ..] => {
					super::push(out, 5392);
					s.is_empty()
				}
				['o', s @ ..] => super::t4::n5717(s, out),
				['u', s @ ..] => super::t4::n5718(s, out),
				_ => false,
			}
		}
		['q', s @ ..] => super::t4::n2404(s, out),
		['s', s @ ..] => super::t4::n2405(s, out),
		['t', s @ ..] => super::t4::n2406(s, out),
		_ => false,
	}
}

/// Words ending -tid.
pub(crate) fn n569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 564);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			super::push(out, 2351);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8470);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2352);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uid.
pub(crate) fn n570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 565);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2353);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2354);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2355);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vid.
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 566);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2356);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5405);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5406);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2357);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5407);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5408);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5409);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 5410);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2358);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5411);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5412);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8472);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8474);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8475);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10510);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2359);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dnacidneme.
pub(crate) fn n14055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12083);
	s.is_empty()
}

/// Words ending -snacidneme.
pub(crate) fn n14056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12084);
	s.is_empty()
}

/// Words ending -rutacidneme.
pub(crate) fn n14057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12085);
	s.is_empty()
}

/// Words ending -sutacidneme.
pub(crate) fn n14058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12086);
	s.is_empty()
}

/// Words ending -ercxe.
pub(crate) fn n5924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5566);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 8583);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11606);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ircxe.
pub(crate) fn n5925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5567);
	match s {
		[] => true,
		['b', 'e', 'n', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10558);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 't', 'u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 10560);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucxe.
pub(crate) fn n2619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2551);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 5569);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5570);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5571);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5572);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5573);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8584);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10562);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11608);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11609);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8585);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10563);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5574);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -exe.
pub(crate) fn n677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 670);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2552);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5575);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5576);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10565);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10566);
									s.is_empty()
								}
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
			super::push(out, 2553);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2554);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8586);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2555);
			match s {
				[] => true,
				['c', 'i', 't', 'a', 't', s @ ..] => {
					super::push(out, 5577);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							super::push(out, 8588);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2556);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fxe.
pub(crate) fn n678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 671);
	s.is_empty()
}

/// Words ending -hxe.
pub(crate) fn n679(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 672);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2557);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5578);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2558);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8590);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12087);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12088);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12089);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12090);
													s.is_empty()
												}
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
							super::push(out, 8591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2559);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2560);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5580);
					match s {
						[] => true,
						['o', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11610);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11611);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11612);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11613);
											s.is_empty()
										}
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
					super::push(out, 5581);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ixe.
pub(crate) fn n680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 673);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2561);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2562);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8592);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8593);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11614);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11615);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2564);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2565);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2566);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8594);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2567);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2568);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oxe.
pub(crate) fn n681(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 674);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2569);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2570);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5582);
					match s {
						[] => true,
						['t', 'u', 's', s @ ..] => {
							super::push(out, 8596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5583);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10567);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2571);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2572);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2573);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5585);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8597);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8598);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10569);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 5586);
					s.is_empty()
				}
				['d', 'i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8599);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5587);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5588);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5589);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5590);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2574);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5592);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5593);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pxe.
pub(crate) fn n682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 675);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2576);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5594);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5595);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8601);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2577);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5597);
					match s {
						[] => true,
						['i', 't', 'i', s @ ..] => {
							super::push(out, 8603);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10571);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5598);
					match s {
						[] => true,
						['u', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11616);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11617);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11618);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11619);
											s.is_empty()
										}
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
					super::push(out, 5599);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5600);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8604);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5601);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 8606);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10573);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 11620);
											match s {
												[] => true,
												['a', 'c', s @ ..] => {
													match s {
														[] => true,
														['i', s @ ..] => super::t4::n14256(s, out),
														['t', 'u', 's', s @ ..] => super::t4::n14257(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 11621);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 10574);
									match s {
										[] => true,
										['s', 'c', 'e', 'n', 'd', s @ ..] => {
											super::push(out, 11622);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11623);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8607);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10575);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 10576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8608);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2578);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5603);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8610);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5604);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5605);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5606);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2579);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5608);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10577);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5609);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8612);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8613);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 10579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5610);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5611);
					match s {
						[] => true,
						['d', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10580);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11624);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11625);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10582);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11626);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11627);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10583);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10584);
									s.is_empty()
								}
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
			super::push(out, 2580);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5612);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5613);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5614);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t4::n2649(s, out),
		['u', s @ ..] => super::t4::n2650(s, out),
		_ => false,
	}
}

/// Words ending -qxe.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 676);
	match s {
		[] => true,
		['u', 'i', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5623);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10588);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10589);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5624);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10590);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sxe.
pub(crate) fn n684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 677);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2583);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2584);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5625);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5626);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8621);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8622);
							match s {
								[] => true,
								['b', 'e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11632);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11633);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 't', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11634);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11635);
											s.is_empty()
										}
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
					super::push(out, 5627);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2585);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2586);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5628);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 5629);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5630);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2587);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2588);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5631);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5632);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10592);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5633);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5634);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8623);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2589);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5635);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8625);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8626);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5636);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8627);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8628);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10594);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 10595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8629);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5637);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', 't', 'u', 'r', s @ ..] => {
									super::push(out, 10596);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10597);
									s.is_empty()
								}
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
			super::push(out, 2590);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5638);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -txe.
pub(crate) fn n685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 678);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2591);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5639);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5640);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5641);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2592);
			match s {
				[] => true,
				['n', 's', s @ ..] => {
					super::push(out, 5642);
					match s {
						[] => true,
						['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8631);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'o', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2593);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5643);
					match s {
						[] => true,
						['e', 's', 'c', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 8633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5644);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8634);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2594);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2595);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5646);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5647);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10598);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5648);
					match s {
						[] => true,
						['c', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10600);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5649);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8637);
							s.is_empty()
						}
						['d', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 8638);
							s.is_empty()
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 8639);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uxe.
pub(crate) fn n686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 679);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2597);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2598);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2599);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2600);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5650);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8640);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8641);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2601);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2602);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5651);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5652);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5653);
					match s {
						[] => true,
						['e', 'c', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10602);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10603);
									s.is_empty()
								}
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
			super::push(out, 2603);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2604);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dnanitsef.
pub(crate) fn n14064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12091);
	s.is_empty()
}

/// Words ending -snanitsef.
pub(crate) fn n14065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12092);
	s.is_empty()
}

/// Words ending -unitsef.
pub(crate) fn n12039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10615);
	s.is_empty()
}

/// Words ending -vitsef.
pub(crate) fn n9569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8675);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10616);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10617);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utsef.
pub(crate) fn n6082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5706);
	s.is_empty()
}

/// Words ending -tef.
pub(crate) fn n707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 700);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2655);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5707);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 5708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2656);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5709);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uef.
pub(crate) fn n708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 701);
	match s {
		[] => true,
		['d', 'a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5711);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5712);
					match s {
						[] => true,
						['u', 's', s @ ..] => {
							super::push(out, 8676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -isoitigalf.
pub(crate) fn n14066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12093);
	s.is_empty()
}

/// Words ending -usoitigalf.
pub(crate) fn n14067(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12094);
	s.is_empty()
}

/// Words ending -nurf.
pub(crate) fn n2806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2733);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10639);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 10640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8706);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10641);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11649);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11650);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -surf.
pub(crate) fn n2807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2734);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 5767);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8707);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8708);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8709);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -turf.
pub(crate) fn n2808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2735);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5768);
			match s {
				[] => true,
				['s', 'c', 'e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 8710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5769);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oilibaronoh.
pub(crate) fn n14068(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12095);
	s.is_empty()
}

/// Words ending -silibaronoh.
pub(crate) fn n14069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12096);
	s.is_empty()
}

/// The word sumissinammi.
pub(crate) fn n14070(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -pmi.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 830);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2895);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5924);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8805);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8806);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5925);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 't', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'i', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
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
						_ => false,
					}
				}
				['s', 't', s @ ..] => {
					super::push(out, 5927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2896);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5928);
					match s {
						[] => true,
						['i', 't', 'i', s @ ..] => {
							super::push(out, 8807);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10680);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5929);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8808);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8809);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5930);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8811);
							match s {
								[] => true,
								['t', 'o', s @ ..] => {
									super::push(out, 10682);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8812);
							match s {
								[] => true,
								['t', 'i', s @ ..] => {
									super::push(out, 10683);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 11662);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11663);
											s.is_empty()
										}
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
					super::push(out, 5931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2897);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2898);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5932);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10684);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 10685);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10686);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5933);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 8813);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11664);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11665);
											s.is_empty()
										}
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
					super::push(out, 5934);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5935);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10688);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5936);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2899);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5937);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5938);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10690);
									s.is_empty()
								}
								['u', 'n', s @ ..] => {
									super::push(out, 10691);
									match s {
										[] => true,
										['i', 's', s @ ..] => {
											super::push(out, 11666);
											s.is_empty()
										}
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
					super::push(out, 5939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
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
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2900);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5940);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5941);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8814);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 8815);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 8816);
							s.is_empty()
						}
						['m', 'p', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8817);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8818);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 8819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2901);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 5942);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8820);
							s.is_empty()
						}
						['i', s @ ..] => super::t4::n9764(s, out),
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5943);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5944);
					s.is_empty()
				}
				['n', s @ ..] => super::t4::n6354(s, out),
				['r', s @ ..] => super::t4::n6355(s, out),
				['t', s @ ..] => {
					super::push(out, 5947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -acni.
pub(crate) fn n3003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2910);
	match s {
		[] => true,
		['n', 'e', s @ ..] => {
			super::push(out, 5959);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5960);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 8832);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ecni.
pub(crate) fn n3004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2911);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5961);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5962);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8834);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8835);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8836);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5963);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5964);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5965);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hcni.
pub(crate) fn n3005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2912);
	s.is_empty()
}

/// Words ending -icni.
pub(crate) fn n3006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2913);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5966);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10695);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 10696);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8839);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8840);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5967);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5968);
			match s {
				[] => true,
				['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 8842);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10697);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5969);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lcni.
pub(crate) fn n3007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2914);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5970);
			match s {
				[] => true,
				['m', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 10699);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'u', s @ ..] => {
					super::push(out, 8843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5971);
			match s {
				[] => true,
				['m', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10701);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5972);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10703);
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11671);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11672);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 10704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5973);
			match s {
				[] => true,
				['d', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10705);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10706);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10707);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocni.
pub(crate) fn n3008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2915);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 5974);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8845);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5975);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5976);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5977);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8847);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10709);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10710);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8848);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10712);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5978);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8849);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8850);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 8851);
					match s {
						[] => true,
						['u', 's', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11673);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11674);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8852);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8853);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 10714);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 10715);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8854);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10716);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 10717);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8855);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10719);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10720);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10721);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11675);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 8856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcni.
pub(crate) fn n3009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2916);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5980);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5981);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8857);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8858);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8859);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8860);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5982);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5983);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ucni.
pub(crate) fn n3010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2917);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5984);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8862);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8863);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8864);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 8866);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['s', s @ ..] => {
					super::push(out, 8867);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['t', s @ ..] => {
					super::push(out, 8868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dni.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 835);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2918);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5986);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2919);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5988);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8869);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8870);
							match s {
								[] => true,
								['i', 'n', 'a', 'b', s @ ..] => {
									super::push(out, 10722);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8871);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 5989);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8872);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5990);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5991);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5992);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5993);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5994);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2920);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8874);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8875);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 5996);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 8877);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5997);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5998);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5999);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2921);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2922);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6002);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8879);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6003);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6004);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6005);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6006);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 6007);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eni.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 836);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2923);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2924);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2925);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2926);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6008);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2927);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2928);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6010);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2929);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 2930);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2931);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 6012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2932);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 2933);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 2934);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6013);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8881);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8882);
							match s {
								[] => true,
								['s', 'a', 'b', s @ ..] => {
									super::push(out, 10723);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6014);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6015);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6016);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fni.
pub(crate) fn n848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 837);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2935);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8884);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6017);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8886);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6018);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6019);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2936);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 8888);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6021);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10724);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 12097);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12098);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 10725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6022);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6023);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6024);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10726);
									s.is_empty()
								}
								['i', 'o', s @ ..] => {
									super::push(out, 10727);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10728);
									s.is_empty()
								}
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
			super::push(out, 2937);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6025);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6026);
					match s {
						[] => true,
						['e', 'l', 'i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10729);
									match s {
										[] => true,
										['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 6027);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6028);
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10730);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 10731);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6029);
					match s {
						[] => true,
						['m', 'i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6030);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 6031);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10733);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10734);
									s.is_empty()
								}
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
			super::push(out, 2938);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6032);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8890);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8891);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11677);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11678);
											s.is_empty()
										}
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
					super::push(out, 6033);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6034);
					match s {
						[] => true,
						['c', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10735);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10737);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10738);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6035);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8892);
							s.is_empty()
						}
						['x', 'u', 'r', s @ ..] => {
							super::push(out, 8893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2939);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6036);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6037);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8894);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2940);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6039);
					match s {
						[] => true,
						['c', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10739);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10740);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 8896);
							s.is_empty()
						}
						['o', 'r', 'b', 'i', 't', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t4::n6456(s, out),
				['i', s @ ..] => super::t4::n6457(s, out),
				['o', s @ ..] => {
					super::push(out, 6042);
					s.is_empty()
				}
				['u', s @ ..] => super::t4::n6459(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t4::n3034(s, out),
		_ => false,
	}
}

/// Words ending -gni.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 838);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2942);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 6047);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8904);
							match s {
								[] => true,
								['s', 'c', 'e', 'n', 'd', s @ ..] => {
									super::push(out, 10743);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8905);
							match s {
								[] => true,
								['s', 'c', 'e', 'n', 'd', s @ ..] => {
									super::push(out, 10744);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11681);
											s.is_empty()
										}
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
					super::push(out, 6048);
					match s {
						[] => true,
						['i', 'o', s @ ..] => {
							super::push(out, 8906);
							match s {
								[] => true,
								['s', 'i', 's', s @ ..] => {
									super::push(out, 10745);
									s.is_empty()
								}
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
			super::push(out, 2943);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 8907);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10746);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 8908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2944);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hni.
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 839);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2945);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6050);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6051);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6052);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2946);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2947);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 6053);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10748);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 6054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2948);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6055);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8909);
							s.is_empty()
						}
						['o', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'd', s @ ..] => {
									super::push(out, 10750);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 10751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2949);
			match s {
				[] => true,
				['m', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8910);
							match s {
								[] => true,
								['i', 's', s @ ..] => {
									super::push(out, 10752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ini.
pub(crate) fn n851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 840);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2950);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2951);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2952);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2953);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11682);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11683);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10754);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8912);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2954);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 2955);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2956);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2957);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 6057);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6058);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8913);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6059);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8915);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8917);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lni.
pub(crate) fn n852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 841);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2958);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 6060);
					match s {
						[] => true,
						['u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2959);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8920);
							s.is_empty()
						}
						['e', 'r', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6061);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6062);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6063);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8922);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2961);
			match s {
				[] => true,
				['s', 'u', 's', s @ ..] => {
					super::push(out, 6064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

