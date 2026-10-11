use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['e', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 40);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8);
					match s {
						[] => true,
						['a', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 41);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 42);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 43);
							match s {
								[] => true,
								['m', 'e', 'm', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 11);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 44);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 45);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 266);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1095);
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 267);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1096);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 268);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1097);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 2507);
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 1098);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 269);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1099);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 270);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 1100);
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 2508);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 271);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1101);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 272);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1102);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2509);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 1103);
											match s {
												[] => true,
												['p', s @ ..] => {
													super::push(out, 2510);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n4(s, out),
		['r', s @ ..] => super::t0::n5(s, out),
		['t', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 35);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 250);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1078);
									match s {
										[] => true,
										['c', 'u', 'l', 'i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t0::n256(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t0::n36(s, out),
				['i', s @ ..] => super::t0::n37(s, out),
				['r', s @ ..] => super::t0::n38(s, out),
				['s', s @ ..] => super::t0::n39(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -o.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 273);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1104);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1105);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 47);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 275);
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
				['i', s @ ..] => {
					super::push(out, 48);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 276);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1106);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 2511);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 2512);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 277);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 278);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1108);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t0::n285(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 280);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t0::n51(s, out),
				['o', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['r', s @ ..] => super::t0::n289(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t0::n53(s, out),
				['u', s @ ..] => super::t0::n54(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n15(s, out),
		['d', s @ ..] => super::t0::n16(s, out),
		['e', s @ ..] => super::t0::n17(s, out),
		['f', s @ ..] => super::t0::n18(s, out),
		['g', s @ ..] => super::t0::n19(s, out),
		['h', s @ ..] => super::t0::n20(s, out),
		['i', s @ ..] => super::t0::n21(s, out),
		['l', s @ ..] => super::t0::n22(s, out),
		['m', s @ ..] => super::t0::n23(s, out),
		['n', s @ ..] => super::t0::n24(s, out),
		['o', s @ ..] => super::t0::n25(s, out),
		['p', s @ ..] => super::t0::n26(s, out),
		['r', s @ ..] => super::t0::n27(s, out),
		['s', s @ ..] => super::t0::n28(s, out),
		['t', s @ ..] => super::t0::n29(s, out),
		['u', s @ ..] => super::t0::n30(s, out),
		['v', s @ ..] => super::t0::n31(s, out),
		['x', s @ ..] => super::t0::n32(s, out),
		['z', s @ ..] => super::t0::n33(s, out),
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['o', s @ ..] => super::t0::n34(s, out),
		_ => false,
	}
}

/// Words ending -nat.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1079);
			match s {
				[] => true,
				['d', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -et.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['l', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1081);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3954);
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2496);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1083);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2497);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4918);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5320);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t0::n8141(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1084);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2498);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3955);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3956);
									match s {
										[] => true,
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 255);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1085);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2499);
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
			super::push(out, 256);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 257);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1087);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -it.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2500);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3957);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 4919);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3958);
									match s {
										[] => true,
										['e', 'p', 's', 'e', 'v', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 5321);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 5322);
													match s {
														[] => true,
														['i', s @ ..] => super::t0::n8142(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2501);
							match s {
								[] => true,
								['c', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 3959);
									s.is_empty()
								}
								['n', 'm', 'u', 't', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3960);
									s.is_empty()
								}
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
			super::push(out, 259);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1089);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2502);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2503);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3961);
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
		['g', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1090);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1091);
					match s {
						[] => true,
						['n', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1092);
					match s {
						[] => true,
						['p', 'm', s @ ..] => {
							super::push(out, 2506);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3962);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1093);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -st.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ribo.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1109);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
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
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mbo.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2513);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1111);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2515);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2516);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3964);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2517);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3966);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 3967);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4920);
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
						['o', s @ ..] => {
							super::push(out, 2518);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2519);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1112);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2520);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3968);
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
						['p', s @ ..] => {
							super::push(out, 2521);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -robo.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	s.is_empty()
}

/// Words ending -rbo.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ubo.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 285);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 286);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 287);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2523);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2524);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 3969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1117);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1118);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2526);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2529);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 3970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2531);
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
				['x', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 289);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -co.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 293);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 294);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1126);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1127);
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
					super::push(out, 297);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 300);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 301);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 302);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1129);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2533);
									match s {
										[] => true,
										['p', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 303);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1130);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1131);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2534);
									match s {
										[] => true,
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'p', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 304);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 305);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1132);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2535);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1133);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1134);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n58(s, out),
		['l', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['a', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 316);
					match s {
						[] => true,
						['c', 'c', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1155);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2578);
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
						['m', 'e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 317);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1156);
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
				['i', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 1157);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1158);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1159);
							match s {
								[] => true,
								['t', 'e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 60);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 321);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1160);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 2579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 322);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 324);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 325);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 1161);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 2580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1163);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => super::t0::n1274(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n336(s, out),
				['u', s @ ..] => super::t0::n337(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t0::n63(s, out),
		['u', s @ ..] => super::t0::n64(s, out),
		_ => false,
	}
}

/// Words ending -do.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 339);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1209);
							s.is_empty()
						}
						['s', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1210);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 341);
					match s {
						[] => true,
						['c', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 342);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1213);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 343);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1214);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 1216);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 2700);
									match s {
										[] => true,
										['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1217);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1218);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1219);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2701);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 344);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 346);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1220);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 347);
					match s {
						[] => true,
						['m', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 348);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1223);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 2702);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1224);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1225);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', s @ ..] => super::t0::n1361(s, out),
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => super::t0::n1363(s, out),
						['x', 'e', '\u{1}', s @ ..] => super::t0::n1364(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t0::n360(s, out),
				['m', s @ ..] => super::t0::n361(s, out),
				['o', s @ ..] => super::t0::n362(s, out),
				['p', s @ ..] => super::t0::n363(s, out),
				['r', s @ ..] => super::t0::n364(s, out),
				['s', s @ ..] => super::t0::n365(s, out),
				['x', s @ ..] => super::t0::n366(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t0::n70(s, out),
		['l', 'a', 'c', 'x', 'e', '\u{1}', s @ ..] => super::t0::n71(s, out),
		['m', s @ ..] => super::t0::n72(s, out),
		['n', s @ ..] => super::t0::n73(s, out),
		['o', s @ ..] => super::t0::n74(s, out),
		['r', s @ ..] => super::t0::n75(s, out),
		['s', s @ ..] => super::t0::n76(s, out),
		['u', s @ ..] => super::t0::n77(s, out),
		_ => false,
	}
}

/// Words ending -eo.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 77);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 78);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 390);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1310);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 79);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1312);
							match s {
								[] => true,
								['i', 't', 'a', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1313);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 2800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1314);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 393);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1316);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1317);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 394);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1318);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									super::push(out, 2803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 395);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1319);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 396);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 398);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1322);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 2804);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4214);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1323);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4215);
											s.is_empty()
										}
										['b', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5048);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n7906(s, out),
														_ => false,
													}
												}
												['o', s @ ..] => {
													super::push(out, 5049);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 4216);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 5050);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n7907(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 5051);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 4217);
											s.is_empty()
										}
										['x', s @ ..] => {
											super::push(out, 4218);
											s.is_empty()
										}
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
					super::push(out, 399);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1324);
							s.is_empty()
						}
						['o', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1325);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1326);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 2805);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 4219);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n81(s, out),
		['d', s @ ..] => super::t0::n82(s, out),
		['e', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 416);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 417);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t0::n84(s, out),
		['i', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 425);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1377);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 2896);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 1378);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t0::n86(s, out),
		['m', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1403);
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
					super::push(out, 437);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n449(s, out),
				['m', s @ ..] => super::t0::n450(s, out),
				['r', s @ ..] => super::t0::n451(s, out),
				['s', s @ ..] => super::t0::n452(s, out),
				['u', s @ ..] => super::t0::n453(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n88(s, out),
		['o', s @ ..] => super::t0::n89(s, out),
		['p', s @ ..] => super::t0::n90(s, out),
		['r', s @ ..] => super::t1::n91(s, out),
		['s', s @ ..] => super::t1::n92(s, out),
		['t', s @ ..] => super::t1::n93(s, out),
		['u', s @ ..] => super::t1::n94(s, out),
		['v', s @ ..] => super::t1::n95(s, out),
		['x', s @ ..] => super::t1::n96(s, out),
		_ => false,
	}
}

/// Words ending -fo.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['p', 'r', 'u', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'a', 'c', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -go.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 489);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 490);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 491);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 492);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 493);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1482);
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
				['m', s @ ..] => {
					super::push(out, 494);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 1483);
							match s {
								[] => true,
								['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 495);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1484);
							match s {
								[] => true,
								['t', 'e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 496);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 497);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1486);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 2999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1487);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1488);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3000);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 498);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1489);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1490);
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
				['v', s @ ..] => {
					super::push(out, 499);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1492);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3001);
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
		['e', s @ ..] => {
			super::push(out, 98);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 500);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 501);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1496);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3002);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3003);
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
						['c', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1499);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3004);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3005);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 3006);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 3007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 1500);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3008);
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1501);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3009);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 3010);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 4352);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 4353);
											match s {
												[] => true,
												['n', 'i', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7947(s, out),
														['b', 'u', 's', '\u{1}', s @ ..] => super::t1::n7948(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1502);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3011);
									match s {
										[] => true,
										['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1503);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3012);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 503);
					match s {
						[] => true,
						['b', s @ ..] => super::t1::n1677(s, out),
						['e', s @ ..] => super::t1::n1678(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t1::n517(s, out),
				['t', s @ ..] => super::t1::n518(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n100(s, out),
		['l', s @ ..] => super::t1::n101(s, out),
		['n', s @ ..] => super::t1::n102(s, out),
		['o', s @ ..] => super::t1::n103(s, out),
		['r', s @ ..] => super::t1::n104(s, out),
		['u', s @ ..] => super::t1::n105(s, out),
		_ => false,
	}
}

/// Words ending -ho.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 532);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4398);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3089);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['p', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7961(s, out),
														['u', s @ ..] => super::t1::n7962(s, out),
														_ => false,
													}
												}
												['t', 'n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['b', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 106);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 534);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 1576);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3090);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4399);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 108);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 537);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1577);
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 538);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -io.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 111);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 539);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 540);
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
			super::push(out, 112);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 541);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1579);
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
				['m', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1580);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 3091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4400);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1581);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t1::n114(s, out),
		['d', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['l', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3137);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1625);
							match s {
								[] => true,
								['b', 'o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 4425);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5141);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n7985(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1626);
							match s {
								[] => true,
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4426);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'd', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 3138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1627);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3139);
									match s {
										[] => true,
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4427);
											s.is_empty()
										}
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
					super::push(out, 558);
					match s {
						[] => true,
						['m', s @ ..] => super::t1::n1842(s, out),
						['r', s @ ..] => super::t1::n1843(s, out),
						['t', s @ ..] => super::t1::n1844(s, out),
						_ => false,
					}
				}
				['n', s @ ..] => super::t1::n573(s, out),
				['o', s @ ..] => super::t1::n574(s, out),
				['r', 'o', '\u{1}', s @ ..] => super::t1::n575(s, out),
				['u', s @ ..] => super::t1::n576(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n116(s, out),
		['f', s @ ..] => super::t1::n117(s, out),
		['g', s @ ..] => super::t1::n118(s, out),
		['h', s @ ..] => super::t1::n119(s, out),
		['i', s @ ..] => super::t1::n120(s, out),
		['l', s @ ..] => super::t1::n121(s, out),
		['m', s @ ..] => super::t1::n122(s, out),
		['n', s @ ..] => super::t1::n123(s, out),
		['p', s @ ..] => super::t1::n124(s, out),
		['r', s @ ..] => super::t1::n125(s, out),
		['s', s @ ..] => super::t1::n126(s, out),
		['t', s @ ..] => super::t1::n127(s, out),
		['u', s @ ..] => super::t1::n128(s, out),
		['v', s @ ..] => super::t1::n129(s, out),
		['x', s @ ..] => super::t1::n130(s, out),
		_ => false,
	}
}

/// Words ending -lo.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 641);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 643);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 644);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 645);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 646);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 647);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1804);
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
				['p', s @ ..] => {
					super::push(out, 648);
					match s {
						[] => true,
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1805);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3332);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 131);
			match s {
				[] => true,
				['n', 'a', 'x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 649);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1806);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 132);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', 'd', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 134);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 650);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 1807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1808);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4554);
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4555);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 4556);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5191);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n8029(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 4557);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5192);
													s.is_empty()
												}
												_ => false,
											}
										}
										['x', s @ ..] => {
											super::push(out, 4558);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1809);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3333);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3334);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 4559);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 5193);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n8030(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 4560);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 652);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1810);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3335);
									match s {
										[] => true,
										['t', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'p', 'p', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['i', 'v', 'd', s @ ..] => {
							super::push(out, 1812);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['i', 's', 'n', s @ ..] => {
							super::push(out, 1813);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => super::t1::n2037(s, out),
						['p', s @ ..] => {
							super::push(out, 1815);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t1::n678(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n137(s, out),
		['o', s @ ..] => super::t1::n138(s, out),
		['p', s @ ..] => super::t1::n139(s, out),
		['u', s @ ..] => super::t1::n140(s, out),
		['y', s @ ..] => super::t1::n141(s, out),
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 685);
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', 'l', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 686);
					match s {
						[] => true,
						['c', 'x', s @ ..] => {
							super::push(out, 1877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 142);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 687);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 689);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1878);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3408);
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
							super::push(out, 1879);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3409);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3410);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4598);
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
				['h', 'p', 's', 'a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 690);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 691);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 692);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1881);
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
				['r', s @ ..] => {
					super::push(out, 693);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1882);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1883);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3411);
									match s {
										[] => true,
										['n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 4599);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1884);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3412);
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3413);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3414);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4600);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3415);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4601);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 4602);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 5213);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n8041(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3416);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3417);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 143);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 694);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 695);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 697);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1889);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 698);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t1::n723(s, out),
				['s', s @ ..] => {
					super::push(out, 700);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 701);
					match s {
						[] => true,
						['i', 'g', 'e', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', s @ ..] => {
							super::push(out, 1896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 702);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1897);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['a', 'p', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => super::t1::n146(s, out),
		['o', s @ ..] => super::t1::n147(s, out),
		['r', s @ ..] => super::t1::n148(s, out),
		['s', s @ ..] => super::t1::n149(s, out),
		['u', s @ ..] => super::t1::n150(s, out),
		['y', s @ ..] => super::t1::n151(s, out),
		_ => false,
	}
}

/// Words ending -no.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 152);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1910);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 1911);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3427);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3428);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4604);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 720);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', s @ ..] => {
							super::push(out, 1912);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 721);
					match s {
						[] => true,
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 723);
					match s {
						[] => true,
						['c', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 725);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 726);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1914);
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 727);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1915);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3430);
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
					super::push(out, 728);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['a', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 732);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1916);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1917);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1918);
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 3431);
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1919);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3432);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 4605);
											s.is_empty()
										}
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
					super::push(out, 733);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3433);
									s.is_empty()
								}
								['p', 'o', s @ ..] => {
									super::push(out, 3434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n157(s, out),
		['m', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1947);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1948);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 748);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1949);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1950);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 3462);
									s.is_empty()
								}
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
			super::push(out, 158);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1951);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1952);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 1953);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 750);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n160(s, out),
		['r', s @ ..] => super::t1::n161(s, out),
		['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['g', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oo.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 162);
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

/// Words ending -po.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 760);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 761);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 762);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1987);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1988);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3491);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 3492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 1989);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3493);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3494);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 4634);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3495);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4635);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 4636);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 3496);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4637);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													super::push(out, 5220);
													s.is_empty()
												}
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
							super::push(out, 1990);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 3497);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1991);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3498);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4638);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1992);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1993);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 3499);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1994);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3500);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3501);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 3502);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3503);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4639);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 3504);
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
							super::push(out, 1995);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3505);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 4640);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5221);
													s.is_empty()
												}
												_ => false,
											}
										}
										['b', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4641);
											s.is_empty()
										}
										['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 165);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 764);
					match s {
						[] => true,
						['i', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 765);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1996);
							match s {
								[] => true,
								['i', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 766);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'd', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n168(s, out),
		['m', s @ ..] => super::t1::n169(s, out),
		['o', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['o', 'p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n172(s, out),
		['s', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n174(s, out),
		_ => false,
	}
}

/// Words ending -ro.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 782);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2012);
							match s {
								[] => true,
								['s', 's', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2013);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2015);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 3522);
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
				['e', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2016);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2017);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2018);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 3523);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4651);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 2019);
							match s {
								[] => true,
								['e', 't', 'x', s @ ..] => {
									super::push(out, 3524);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4652);
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
				['i', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['r', 'c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['u', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['e', 's', 'e', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['a', 'u', 'q', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'h', 't', 'a', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n179(s, out),
		['g', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 803);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2074);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2075);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3566);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 804);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2076);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3568);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4676);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 3569);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 3570);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4677);
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 't', 'a', 'l', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['g', 'e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 805);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2077);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 806);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 807);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2078);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3572);
									match s {
										[] => true,
										['a', 'r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 4678);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 5228);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t1::n849(s, out),
				['v', s @ ..] => {
					super::push(out, 809);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t1::n183(s, out),
		['p', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['u', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t1::n185(s, out),
		['t', s @ ..] => super::t1::n186(s, out),
		['u', s @ ..] => super::t1::n187(s, out),
		['y', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['g', 'e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -so.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['n', 'e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2121);
							match s {
								[] => true,
								['q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 188);
			match s {
				[] => true,
				['h', 'c', 'n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 845);
					s.is_empty()
				}
				['l', 'i', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 846);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 847);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2122);
							s.is_empty()
						}
						['t', 'c', 'e', 'l', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'p', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 848);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 849);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2124);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3617);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3618);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4697);
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
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 189);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 190);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 851);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2125);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 2126);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3619);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4698);
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
						['h', s @ ..] => {
							super::push(out, 2127);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2128);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2129);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3620);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 3621);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4699);
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 4700);
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
				['i', s @ ..] => {
					super::push(out, 852);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2130);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3622);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 853);
					match s {
						[] => true,
						['p', 's', 'e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 854);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2131);
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
					super::push(out, 855);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2132);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 856);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 192);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 857);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2134);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3623);
									s.is_empty()
								}
								['r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 2135);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 858);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3625);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 3626);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 4701);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 3627);
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t1::n196(s, out),
		['u', s @ ..] => {
			super::push(out, 194);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 864);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2148);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => super::t1::n913(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -to.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 866);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 867);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 868);
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2153);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3645);
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
							super::push(out, 2154);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', s @ ..] => {
							super::push(out, 2155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3647);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2156);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', 'o', 'p', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 869);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2157);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3648);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 4711);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5242);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', s @ ..] => {
											super::push(out, 4712);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5243);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 4713);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 5244);
													match s {
														[] => true,
														['c', '\u{1}', s @ ..] => super::t1::n8059(s, out),
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
									super::push(out, 3649);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2158);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2159);
							match s {
								[] => true,
								['l', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2160);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t1::n921(s, out),
				['i', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2170);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2171);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2172);
							match s {
								[] => true,
								['d', s @ ..] => super::t1::n4680(s, out),
								['n', s @ ..] => super::t1::n4681(s, out),
								_ => false,
							}
						}
						['r', s @ ..] => super::t1::n2531(s, out),
						_ => false,
					}
				}
				['l', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => super::t1::n924(s, out),
				['o', s @ ..] => super::t1::n925(s, out),
				['u', s @ ..] => super::t1::n926(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n200(s, out),
		['i', s @ ..] => super::t1::n201(s, out),
		['l', s @ ..] => super::t1::n202(s, out),
		['n', s @ ..] => super::t1::n203(s, out),
		['o', s @ ..] => super::t1::n204(s, out),
		['p', s @ ..] => super::t1::n205(s, out),
		['r', s @ ..] => super::t1::n206(s, out),
		['s', s @ ..] => super::t1::n207(s, out),
		['t', s @ ..] => super::t2::n208(s, out),
		['u', s @ ..] => super::t2::n209(s, out),
		['x', s @ ..] => super::t2::n210(s, out),
		['æ', s @ ..] => super::t2::n211(s, out),
		_ => false,
	}
}

/// Words ending -uo.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 209);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3783);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3784);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4800);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 210);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2296);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 2297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 2298);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['s', 'e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2300);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2301);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3787);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4801);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 3788);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 4802);
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 4803);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 3789);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 5272);
													match s {
														[] => true,
														['i', s @ ..] => super::t2::n8089(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2302);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 3790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 946);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2304);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2305);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 949);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => super::t2::n1006(s, out),
				['i', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2317);
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
				['l', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2318);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2319);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 954);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2320);
							match s {
								[] => true,
								['c', s @ ..] => super::t2::n4883(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 's', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 955);
					s.is_empty()
				}
				['p', s @ ..] => super::t2::n1011(s, out),
				['r', s @ ..] => super::t2::n1012(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t2::n218(s, out),
		['p', s @ ..] => super::t2::n219(s, out),
		['q', s @ ..] => super::t2::n220(s, out),
		['r', s @ ..] => super::t2::n221(s, out),
		['s', s @ ..] => super::t2::n222(s, out),
		['t', s @ ..] => super::t2::n223(s, out),
		['x', s @ ..] => super::t2::n224(s, out),
		_ => false,
	}
}

/// Words ending -vo.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 990);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2359);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3832);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 991);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2360);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2361);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2362);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 992);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 993);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 2363);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3833);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4829);
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
						['p', s @ ..] => {
							super::push(out, 2364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['a', 'g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2366);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3834);
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
					super::push(out, 997);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['l', 'u', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2368);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2369);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2370);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3836);
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
							super::push(out, 2371);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2372);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2373);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3837);
									match s {
										[] => true,
										['p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 224);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1000);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2374);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1001);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2376);
							s.is_empty()
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3838);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 4830);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 5286);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8099(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 225);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 226);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1003);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2377);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3839);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 3840);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4831);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2378);
							match s {
								[] => true,
								['e', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2379);
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 3841);
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => super::t2::n4932(s, out),
								['r', 'e', 'p', '\u{1}', s @ ..] => super::t2::n4933(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t2::n1062(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t2::n231(s, out),
		_ => false,
	}
}

/// Words ending -xo.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 228);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1006);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1007);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 2384);
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
		['e', s @ ..] => {
			super::push(out, 229);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1008);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1009);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2386);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3843);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2387);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'l', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 230);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zo.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1011);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2388);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3844);
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
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'a', s @ ..] => {
							super::push(out, 2389);
							match s {
								[] => true,
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['i', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1013);
					match s {
						[] => true,
						['e', 'o', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 'a', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'm', 'y', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 't', 'a', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'e', 'l', 'l', 'o', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1015);
					match s {
						[] => true,
						['a', 'b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 't', 'u', 'a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'g', s @ ..] => {
							super::push(out, 2390);
							match s {
								[] => true,
								['e', 'l', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2391);
							match s {
								[] => true,
								['a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 2392);
							match s {
								[] => true,
								['t', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2393);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3849);
									match s {
										[] => true,
										['b', 'a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5287);
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													super::push(out, 5288);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 3850);
									s.is_empty()
								}
								['t', 's', 'o', 'p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'a', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 2394);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3851);
									match s {
										[] => true,
										['l', 'o', 'p', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2395);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', 't', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4832);
											s.is_empty()
										}
										['t', 'c', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4833);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 3852);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4834);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -or.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1018);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 2396);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 3853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1020);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2398);
							match s {
								[] => true,
								['r', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2399);
							match s {
								[] => true,
								['p', 'm', s @ ..] => {
									super::push(out, 3854);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 4835);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 5289);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8100(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t2::n1084(s, out),
				['o', s @ ..] => super::t2::n1085(s, out),
				['r', s @ ..] => super::t2::n1086(s, out),
				['s', s @ ..] => super::t2::n1087(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t2::n239(s, out),
		['e', s @ ..] => super::t2::n240(s, out),
		['f', s @ ..] => super::t2::n241(s, out),
		['g', s @ ..] => super::t2::n242(s, out),
		['h', s @ ..] => super::t2::n243(s, out),
		['i', s @ ..] => super::t2::n244(s, out),
		['l', s @ ..] => super::t2::n245(s, out),
		['m', s @ ..] => {
			super::push(out, 241);
			s.is_empty()
		}
		['n', s @ ..] => super::t2::n247(s, out),
		['p', s @ ..] => super::t2::n248(s, out),
		['r', s @ ..] => super::t2::n249(s, out),
		['s', s @ ..] => super::t2::n250(s, out),
		['t', s @ ..] => super::t2::n251(s, out),
		['u', s @ ..] => super::t2::n252(s, out),
		['v', s @ ..] => super::t2::n253(s, out),
		['x', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word distaedet.
pub(crate) fn n8141(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -invesperascit.
pub(crate) fn n8142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5449);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ico.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 57);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 306);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 307);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1136);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3971);
									s.is_empty()
								}
								['p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 3972);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3973);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 4921);
											match s {
												[] => true,
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1137);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1138);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2536);
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
					super::push(out, 1139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 3974);
									s.is_empty()
								}
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3975);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2538);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2539);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3976);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
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
						['m', s @ ..] => {
							super::push(out, 2540);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2541);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2542);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3977);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4922);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2543);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2544);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3978);
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
							super::push(out, 2545);
							match s {
								[] => true,
								['l', 'p', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2546);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 3979);
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 4923);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2547);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 3980);
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
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2548);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2549);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 3981);
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
					super::push(out, 1141);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2550);
							match s {
								[] => true,
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1142);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 308);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t2::n315(s, out),
		['l', s @ ..] => {
			super::push(out, 310);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2558);
							s.is_empty()
						}
						['m', 'i', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2559);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3992);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 2560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				['e', s @ ..] => super::t2::n1245(s, out),
				['i', s @ ..] => super::t2::n1246(s, out),
				['r', s @ ..] => super::t2::n1247(s, out),
				['u', 'p', '\u{1}', s @ ..] => super::t2::n1248(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t2::n318(s, out),
		['p', '\u{1}', s @ ..] => super::t2::n319(s, out),
		['r', s @ ..] => super::t2::n320(s, out),
		['s', s @ ..] => super::t2::n321(s, out),
		['t', s @ ..] => super::t2::n322(s, out),
		['x', 'o', 't', '\u{1}', s @ ..] => super::t2::n323(s, out),
		_ => false,
	}
}

/// Words ending -perco.
pub(crate) fn n1274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1164);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2582);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irco.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 327);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1165);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urco.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 328);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1166);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1167);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sco.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1168);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1169);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2584);
							match s {
								[] => true,
								['b', 'l', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1170);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4007);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4933);
											match s {
												[] => true,
												['d', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2585);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4008);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2586);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4009);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1172);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4010);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4934);
											match s {
												[] => true,
												['t', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', 'u', 'a', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 4011);
									match s {
										[] => true,
										['e', 'v', 'n', s @ ..] => {
											super::push(out, 4935);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 5329);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1173);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2587);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4012);
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
		['e', s @ ..] => super::t2::n339(s, out),
		['i', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1189);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2656);
							match s {
								[] => true,
								['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4133);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2657);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4134);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5026);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t2::n3141(s, out),
						['s', s @ ..] => super::t2::n3142(s, out),
						['u', s @ ..] => {
							super::push(out, 2660);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => super::t2::n5375(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t2::n1300(s, out),
				['f', s @ ..] => super::t2::n1301(s, out),
				['g', s @ ..] => super::t2::n1302(s, out),
				['h', s @ ..] => super::t2::n1303(s, out),
				['l', s @ ..] => super::t2::n1304(s, out),
				['m', s @ ..] => super::t2::n1305(s, out),
				['n', s @ ..] => super::t2::n1306(s, out),
				['p', s @ ..] => super::t2::n1307(s, out),
				['t', s @ ..] => super::t2::n1308(s, out),
				['v', s @ ..] => super::t2::n1309(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t2::n341(s, out),
		['u', s @ ..] => super::t2::n342(s, out),
		_ => false,
	}
}

/// Words ending -uco.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2696);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2697);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2698);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'r', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1206);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
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
								['t', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 336);
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 337);
			match s {
				[] => true,
				['x', s @ ..] => {
					super::push(out, 1207);
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
		['t', 's', 'e', 'f', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 338);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rocedo.
pub(crate) fn n1361(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word discedo.
pub(crate) fn n1363(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word excedo.
pub(crate) fn n1364(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -dedo.
pub(crate) fn n360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 349);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1226);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1227);
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

/// Words ending -medo.
pub(crate) fn n361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1228);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oedo.
pub(crate) fn n362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pedo.
pub(crate) fn n363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1230);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1231);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2704);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1232);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 2705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2706);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4167);
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

/// Words ending -redo.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1233);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2707);
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2708);
					match s {
						[] => true,
						['x', s @ ..] => {
							super::push(out, 4168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2709);
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
		['r', s @ ..] => {
			super::push(out, 1235);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2710);
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

/// Words ending -sedo.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1236);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xedo.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1237);
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

/// Words ending -ido.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1238);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2712);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4170);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 2713);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1239);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2714);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2716);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1241);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2717);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4174);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4175);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5038);
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
						['o', s @ ..] => {
							super::push(out, 2718);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1242);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2719);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 4177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1243);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 4178);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 4179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 1246);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4180);
									s.is_empty()
								}
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
			super::push(out, 357);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1247);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2722);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1249);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 360);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1250);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 2723);
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
					super::push(out, 1251);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['i', 't', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['a', 'h', 'c', 'e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'f', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4181);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4182);
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 4183);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5039);
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
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t2::n1401(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t2::n373(s, out),
		['r', s @ ..] => super::t2::n374(s, out),
		['s', s @ ..] => super::t2::n375(s, out),
		['t', s @ ..] => super::t2::n376(s, out),
		['v', s @ ..] => super::t2::n377(s, out),
		_ => false,
	}
}

/// The word excaldo.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	s.is_empty()
}

/// Words ending -mdo.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 367);
			match s {
				[] => true,
				['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ndo.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 368);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1271);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2739);
							match s {
								[] => true,
								['a', 'r', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1272);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2740);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2741);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1273);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2742);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2743);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2744);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 2745);
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
		['e', s @ ..] => {
			super::push(out, 369);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1274);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2746);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 4185);
									match s {
										[] => true,
										['a', 'r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 5040);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5388);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => super::t2::n1424(s, out),
				['m', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2748);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t2::n1426(s, out),
				['r', s @ ..] => super::t2::n1427(s, out),
				['t', s @ ..] => super::t2::n1428(s, out),
				['v', s @ ..] => super::t2::n1429(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n381(s, out),
		['o', s @ ..] => super::t2::n382(s, out),
		['u', s @ ..] => super::t2::n383(s, out),
		_ => false,
	}
}

/// Words ending -odo.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2787);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4210);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1297);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdo.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1299);
					match s {
						[] => true,
						['s', 'a', 'b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 1300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['s', 'x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sdo.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -udo.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 382);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 2791);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
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
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1303);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2795);
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
				['c', s @ ..] => {
					super::push(out, 1304);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2796);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4211);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1306);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2797);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4212);
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
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2798);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 4213);
									s.is_empty()
								}
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
			super::push(out, 387);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1307);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1308);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'b', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word absorbeo.
pub(crate) fn n7906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5390);
	s.is_empty()
}

/// The word desorbeo.
pub(crate) fn n7907(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ceo.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2807);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1329);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2809);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4220);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 5052);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 4221);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5053);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4222);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1331);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2811);
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
				['t', s @ ..] => {
					super::push(out, 1332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 401);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1333);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 402);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2814);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 4223);
									match s {
										[] => true,
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 1335);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2815);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4224);
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
							super::push(out, 2816);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4225);
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
							super::push(out, 2817);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4226);
									s.is_empty()
								}
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
			super::push(out, 403);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1336);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2818);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1337);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2819);
							match s {
								[] => true,
								['e', 'd', s @ ..] => {
									super::push(out, 4227);
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1338);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1339);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2822);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 4228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 406);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2824);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2825);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1342);
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2827);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 2828);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 4230);
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
						['x', s @ ..] => {
							super::push(out, 2829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 4231);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4232);
									s.is_empty()
								}
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
			super::push(out, 408);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1345);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2830);
							s.is_empty()
						}
						['a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2831);
							s.is_empty()
						}
						['d', s @ ..] => super::t2::n3427(s, out),
						['e', s @ ..] => {
							super::push(out, 2833);
							s.is_empty()
						}
						['l', s @ ..] => super::t2::n3429(s, out),
						['n', 'i', '\u{1}', s @ ..] => super::t2::n3430(s, out),
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => super::t2::n3431(s, out),
						['s', s @ ..] => super::t2::n3432(s, out),
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => super::t2::n1514(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -deo.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1347);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1348);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2838);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1349);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2839);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4239);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 4240);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 5058);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 4241);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4242);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5059);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 410);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1350);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 2840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1352);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2842);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2843);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4243);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2844);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2845);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4245);
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5391);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 5392);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 411);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1353);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2846);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4246);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1354);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2847);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4247);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 5060);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2848);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4248);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 5061);
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
							super::push(out, 2849);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 4249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 2850);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2851);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2852);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4250);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 5062);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 4251);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 2853);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2854);
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2855);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2856);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4253);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4254);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5063);
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
									super::push(out, 4255);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5064);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1356);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2857);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2858);
							match s {
								[] => true,
								['i', 'b', s @ ..] => {
									super::push(out, 4256);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 5065);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t2::n423(s, out),
		['o', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1360);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2868);
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
		['r', s @ ..] => super::t2::n425(s, out),
		['u', s @ ..] => super::t2::n426(s, out),
		_ => false,
	}
}

/// Words ending -geo.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 1367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1368);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2876);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1369);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2878);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2879);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4278);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1372);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2881);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4279);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5085);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 2882);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 4280);
									match s {
										[] => true,
										['e', 'p', s @ ..] => {
											super::push(out, 5086);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 5402);
													match s {
														[] => true,
														['s', '\u{1}', s @ ..] => super::t2::n8196(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4281);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 4282);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4283);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 5087);
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
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 4284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 422);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1373);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 2884);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4285);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 5088);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 5089);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 4286);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 5090);
											match s {
												[] => true,
												['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1374);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2885);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 2886);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 2887);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2888);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2889);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2890);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4288);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 2891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1375);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2892);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 2893);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1376);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2894);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2895);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -leo.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2897);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2898);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2899);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2900);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 2901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2902);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2903);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2904);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2905);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4290);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4291);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 5091);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2906);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4292);
									match s {
										[] => true,
										['p', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 5092);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 428);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 2907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 2908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1387);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1388);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2909);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2910);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2911);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 4293);
									s.is_empty()
								}
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
			super::push(out, 431);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1389);
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
		['l', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1390);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2912);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1391);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2914);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 2915);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5093);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 5403);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5094);
											match s {
												[] => true,
												['n', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 433);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1392);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1393);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2916);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4295);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 2917);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 2918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1394);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2919);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 2920);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 2921);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2922);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4297);
									match s {
										[] => true,
										['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2923);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2924);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4299);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5095);
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
				['e', s @ ..] => {
					super::push(out, 1395);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2925);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1396);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t2::n1565(s, out),
				['v', s @ ..] => {
					super::push(out, 1398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t2::n3528(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t2::n445(s, out),
		_ => false,
	}
}

/// Words ending -imeo.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mmeo.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1406);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmeo.
pub(crate) fn n451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -smeo.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1408);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -umeo.
pub(crate) fn n453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1409);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1410);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -neo.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 443);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1411);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2931);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2932);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2933);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 4300);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 4301);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2934);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 4302);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2935);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4303);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 5096);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 5097);
											match s {
												[] => true,
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 445);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1416);
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
		['i', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2938);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2939);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 4304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2940);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2941);
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									super::push(out, 4305);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 5098);
											match s {
												[] => true,
												['p', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1420);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2942);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2943);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2944);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4306);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 2945);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4307);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 5099);
											match s {
												[] => true,
												['r', 't', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 447);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['m', 'o', s @ ..] => {
							super::push(out, 2946);
							match s {
								[] => true,
								['c', 'e', s @ ..] => {
									super::push(out, 4308);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 5100);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 449);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1424);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oeo.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['t', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -peo.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 't', 'b', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2948);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 4309);
									match s {
										[] => true,
										['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 2949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1427);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

