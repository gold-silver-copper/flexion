use flexion::Path;

/// Words ending -lk.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 241);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1843);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6935);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6936);
					match s {
						[] => true,
						['a', 't', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 16099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6937);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 6938);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16100);
							s.is_empty()
						}
						['l', 'e', 'h', 'c', 's', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 16101);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 26252);
									s.is_empty()
								}
								['u', 'a', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 26253);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16102);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 26254);
									match s {
										[] => true,
										['a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 34340);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 26255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 16103);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 16104);
							s.is_empty()
						}
						['z', 't', 'ä', '\u{1}', s @ ..] => {
							super::push(out, 16105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6939);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16106);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 16107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 6941);
					match s {
						[] => true,
						['e', 'k', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 16108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1844);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1845);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 6942);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1846);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6943);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 6944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6945);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 16112);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 16113);
							match s {
								[] => true,
								['m', 'i', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 26256);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 16114);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16115);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 34341);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 34342);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 39705);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', s @ ..] => {
											super::push(out, 34343);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 16116);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 26257);
									match s {
										[] => true,
										['n', 'i', 'l', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 34344);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 26258);
									s.is_empty()
								}
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
			super::push(out, 1847);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 6946);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6947);
					match s {
						[] => true,
						['r', 'e', 'b', 'm', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 16117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 1848);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 1849);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 6950);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nk.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1851);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6951);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 16118);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 26259);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 26260);
									s.is_empty()
								}
								['i', 'r', s @ ..] => {
									super::push(out, 26261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 16119);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 16120);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 26262);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 34345);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 34346);
											s.is_empty()
										}
										['s', 'h', 'c', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 34347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 16121);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 34348);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 34349);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 34350);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 34351);
											s.is_empty()
										}
										['s', 'i', 'e', 'f', 'f', 'i', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 34352);
											s.is_empty()
										}
										['t', s @ ..] => super::t7::n36210(s, out),
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 26263);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 26264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16122);
							s.is_empty()
						}
						['r', s @ ..] => super::t7::n16516(s, out),
						['s', s @ ..] => super::t7::n16517(s, out),
						['t', s @ ..] => {
							super::push(out, 16125);
							match s {
								[] => true,
								['l', s @ ..] => super::t7::n27315(s, out),
								_ => false,
							}
						}
						['z', s @ ..] => super::t7::n16519(s, out),
						['ß', 'o', s @ ..] => super::t7::n16520(s, out),
						_ => false,
					}
				}
				['d', s @ ..] => super::t7::n7029(s, out),
				['h', s @ ..] => super::t7::n7030(s, out),
				['l', s @ ..] => super::t7::n7031(s, out),
				['r', s @ ..] => super::t7::n7032(s, out),
				['t', s @ ..] => super::t7::n7033(s, out),
				['w', s @ ..] => super::t7::n7034(s, out),
				['z', s @ ..] => super::t7::n7035(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t7::n1854(s, out),
		['i', s @ ..] => super::t7::n1855(s, out),
		['o', s @ ..] => super::t7::n1856(s, out),
		['u', s @ ..] => super::t7::n1857(s, out),
		['ä', s @ ..] => super::t7::n1858(s, out),
		_ => false,
	}
}

/// Words ending -ok.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1857);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1858);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1859);
			match s {
				[] => true,
				['c', 's', 'r', s @ ..] => {
					super::push(out, 6980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1860);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1861);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1862);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1863);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1864);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6982);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 16167);
							match s {
								[] => true,
								['i', 'd', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 26309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6984);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 16169);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 16170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1865);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					super::push(out, 6985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1866);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6986);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1867);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 1868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rk.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1869);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6988);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6989);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 16171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6990);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 16172);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 16173);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 16174);
							match s {
								[] => true,
								['l', 'e', s @ ..] => {
									super::push(out, 26310);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 16175);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 26311);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 16176);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 26312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 16177);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 34383);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 34384);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 34385);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 34386);
											match s {
												[] => true,
												['e', 'i', 'n', 'n', s @ ..] => {
													super::push(out, 39733);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 34387);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 39734);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 39735);
													s.is_empty()
												}
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
							super::push(out, 16178);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 26313);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 34388);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 34389);
											match s {
												[] => true,
												['n', 'u', 'l', 'o', 'h', '\u{1}', s @ ..] => {
													super::push(out, 39736);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 34390);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 26314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 16179);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 26315);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 26316);
									match s {
										[] => true,
										['e', 'd', 'a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 34391);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 16180);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 16181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6991);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 16182);
							match s {
								[] => true,
								['n', 'i', 'w', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34392);
											s.is_empty()
										}
										['e', 'r', 'o', 'h', 's', 'f', 'f', 'o', '\u{1}', s @ ..] => {
											super::push(out, 34393);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 16183);
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									super::push(out, 26317);
									match s {
										[] => true,
										['t', 's', 'u', 'd', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 34394);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 16184);
							match s {
								[] => true,
								['u', 'e', 'z', 'r', 'h', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 26318);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 16185);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 26319);
									match s {
										[] => true,
										['n', 'o', 'i', 't', 'a', 'n', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 39737);
													s.is_empty()
												}
												['-', s @ ..] => {
													super::push(out, 39738);
													s.is_empty()
												}
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
							super::push(out, 16186);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									super::push(out, 26320);
									match s {
										[] => true,
										['i', 'h', 'c', 's', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 34395);
											s.is_empty()
										}
										['r', 'e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 34396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => super::t7::n16584(s, out),
						['s', s @ ..] => {
							super::push(out, 16188);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 26324);
									match s {
										[] => true,
										['o', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => super::t7::n36261(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t7::n16586(s, out),
						_ => false,
					}
				}
				['q', s @ ..] => super::t7::n7069(s, out),
				['u', s @ ..] => super::t7::n7070(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t7::n1872(s, out),
		['e', s @ ..] => super::t7::n1873(s, out),
		['i', s @ ..] => super::t7::n1874(s, out),
		['o', s @ ..] => super::t7::n1875(s, out),
		_ => false,
	}
}

/// Words ending -isk.
pub(crate) fn n1877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1875);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7006);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7007);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -osk.
pub(crate) fn n1878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1876);
	match s {
		[] => true,
		['i', 'k', '\u{1}', s @ ..] => {
			super::push(out, 7008);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rsk.
pub(crate) fn n1879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1877);
	s.is_empty()
}

/// Words ending -usk.
pub(crate) fn n1880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1878);
	s.is_empty()
}

/// Words ending -ysk.
pub(crate) fn n1881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1879);
	match s {
		[] => true,
		['j', s @ ..] => {
			super::push(out, 7009);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uk.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1880);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1881);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7010);
					match s {
						[] => true,
						['y', 'd', '\u{1}', s @ ..] => {
							super::push(out, 16214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7011);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1882);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1883);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7014);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7015);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7016);
					match s {
						[] => true,
						['o', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 16215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7017);
					s.is_empty()
				}
				['m', 'r', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 7018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1886);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1887);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1888);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 7020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1889);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1890);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 7021);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7022);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wk.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1891);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16216);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 16217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yk.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1892);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ök.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 250);
	s.is_empty()
}

/// Words ending -ük.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 1894);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -al.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7023);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7024);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 16218);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 26358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16219);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7026);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 7027);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 16221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7028);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 7029);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7030);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7031);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7032);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16222);
							match s {
								[] => true,
								['b', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 26359);
									s.is_empty()
								}
								['h', 'c', 'u', 'ä', 'r', '\u{1}', s @ ..] => {
									super::push(out, 26360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 16223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7033);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16224);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 26361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 16225);
							match s {
								[] => true,
								['l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 26362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16226);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16227);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 16228);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 26363);
									match s {
										[] => true,
										['r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 34429);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 26364);
									match s {
										[] => true,
										['l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 34430);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 16229);
							match s {
								[] => true,
								['s', 'a', s @ ..] => {
									super::push(out, 26365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7034);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 7035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1897);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7036);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7037);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1898);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7039);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 16230);
							match s {
								[] => true,
								['i', 'b', 'm', 'u', '\u{1}', s @ ..] => {
									super::push(out, 26366);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7040);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16231);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1899);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7041);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 16233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7042);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 16234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7043);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16235);
							match s {
								[] => true,
								['k', 's', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 34431);
											match s {
												[] => true,
												['e', 'z', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 39760);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'e', 's', 'e', 'i', 'r', '\u{1}', s @ ..] => {
											super::push(out, 34432);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 16236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7044);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 16237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7045);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 16238);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16239);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16240);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 16241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7046);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['k', 'i', 't', 's', 'a', 'l', 'p', '\u{1}', s @ ..] => {
									super::push(out, 26367);
									s.is_empty()
								}
								['z', 'l', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 26368);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7047);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 16242);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7048);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16243);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 16244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1901);
			match s {
				[] => true,
				['e', s @ ..] => super::t7::n7128(s, out),
				['i', s @ ..] => super::t7::n7129(s, out),
				['n', s @ ..] => {
					super::push(out, 7052);
					match s {
						[] => true,
						['i', 't', s @ ..] => super::t7::n16649(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t7::n7131(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t7::n1904(s, out),
		['i', s @ ..] => super::t7::n1905(s, out),
		['j', s @ ..] => super::t7::n1906(s, out),
		['k', s @ ..] => super::t7::n1907(s, out),
		['l', s @ ..] => super::t7::n1908(s, out),
		['m', s @ ..] => super::t7::n1909(s, out),
		['n', s @ ..] => super::t7::n1910(s, out),
		['o', s @ ..] => super::t7::n1911(s, out),
		['p', s @ ..] => super::t7::n1912(s, out),
		['r', s @ ..] => super::t7::n1913(s, out),
		['s', s @ ..] => super::t7::n1914(s, out),
		['t', s @ ..] => super::t7::n1915(s, out),
		['u', s @ ..] => super::t7::n1916(s, out),
		['v', s @ ..] => super::t7::n1917(s, out),
		['w', s @ ..] => super::t7::n1918(s, out),
		['y', s @ ..] => super::t7::n1919(s, out),
		['z', s @ ..] => super::t7::n1920(s, out),
		_ => false,
	}
}

/// Words ending -el.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1923);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1924);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 7150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1925);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7151);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 16381);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 26456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 16382);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 16383);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 16384);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 26457);
									match s {
										[] => true,
										['v', s @ ..] => {
											super::push(out, 34492);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 16385);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16386);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26458);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 26459);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 39794);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 39795);
													match s {
														[] => true,
														['a', s @ ..] => super::t7::n45485(s, out),
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
									super::push(out, 26460);
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 34493);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16387);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 16388);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 26461);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 34494);
											match s {
												[] => true,
												['n', 'o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 39796);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 34495);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 16389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 7152);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16390);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 26462);
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 34496);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 16391);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 26463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7153);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 16392);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16393);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 26464);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 26465);
									match s {
										[] => true,
										['z', 'g', s @ ..] => {
											super::push(out, 34497);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 16394);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 26466);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 26467);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 34498);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 34499);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 16395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7154);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 16396);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16397);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 26468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 16398);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 16399);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 26469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16400);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 16401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7155);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16402);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 16403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7156);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 16404);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 16405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t7::n7238(s, out),
				['u', s @ ..] => super::t7::n7239(s, out),
				['ä', s @ ..] => {
					super::push(out, 7159);
					s.is_empty()
				}
				['ö', s @ ..] => super::t7::n7241(s, out),
				['ü', s @ ..] => {
					super::push(out, 7161);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 16417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t7::n1928(s, out),
		['d', s @ ..] => super::t8::n1929(s, out),
		['e', s @ ..] => super::t8::n1930(s, out),
		['f', s @ ..] => super::t8::n1931(s, out),
		['g', s @ ..] => super::t8::n1932(s, out),
		['h', s @ ..] => super::t8::n1933(s, out),
		['i', s @ ..] => super::t8::n1934(s, out),
		['k', s @ ..] => super::t8::n1935(s, out),
		['l', s @ ..] => super::t8::n1936(s, out),
		['m', s @ ..] => super::t8::n1937(s, out),
		['n', s @ ..] => super::t8::n1938(s, out),
		['o', s @ ..] => super::t8::n1939(s, out),
		['p', s @ ..] => super::t8::n1940(s, out),
		['r', s @ ..] => super::t8::n1941(s, out),
		['s', s @ ..] => super::t8::n1942(s, out),
		['t', s @ ..] => super::t8::n1943(s, out),
		['u', s @ ..] => super::t8::n1944(s, out),
		['v', s @ ..] => super::t8::n1945(s, out),
		['w', s @ ..] => super::t8::n1946(s, out),
		['x', s @ ..] => super::t8::n1947(s, out),
		['z', s @ ..] => super::t8::n1948(s, out),
		['ß', s @ ..] => super::t8::n1949(s, out),
		_ => false,
	}
}

/// Words ending -hl.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1951);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7298);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 16720);
							match s {
								[] => true,
								['k', 'e', 'i', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 26854);
									s.is_empty()
								}
								['n', 'e', 'n', s @ ..] => {
									super::push(out, 26855);
									match s {
										[] => true,
										['r', 'e', 't', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 34730);
											s.is_empty()
										}
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
					super::push(out, 7299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16721);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16722);
							match s {
								[] => true,
								['g', 'z', s @ ..] => {
									super::push(out, 26856);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'ü', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 16723);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 16724);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', 'r', 'o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 34731);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 34732);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16725);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 16726);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 26857);
									s.is_empty()
								}
								['g', 'a', 't', 't', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 26858);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 26859);
									s.is_empty()
								}
								['t', 'i', 'e', 'z', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 26860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 16727);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 26861);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 26862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7300);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 16728);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', 'm', s @ ..] => {
											super::push(out, 34733);
											s.is_empty()
										}
										['n', 'e', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 39911);
													match s {
														[] => true,
														['o', s @ ..] => super::t8::n45538(s, out),
														_ => false,
													}
												}
												['g', s @ ..] => {
													super::push(out, 39912);
													match s {
														[] => true,
														['t', 'n', 'ö', 'r', '\u{1}', s @ ..] => super::t8::n45539(s, out),
														_ => false,
													}
												}
												['n', 'o', 'r', 't', 'k', 'e', 'l', 'e', '\u{1}', s @ ..] => {
													super::push(out, 39913);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7301);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 16729);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 26863);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 26864);
									match s {
										[] => true,
										['e', 'i', 'd', s @ ..] => {
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 39914);
													match s {
														[] => true,
														['a', s @ ..] => super::t8::n45540(s, out),
														_ => false,
													}
												}
												['g', s @ ..] => {
													super::push(out, 39915);
													s.is_empty()
												}
												['n', 'e', 'h', 'c', 's', 'a', 't', 'd', s @ ..] => {
													super::push(out, 39916);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 39917);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 26865);
									match s {
										[] => true,
										['e', 'd', 'e', '\u{1}', s @ ..] => {
											super::push(out, 34734);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 26866);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 34735);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 34736);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 26867);
									match s {
										[] => true,
										['e', 't', 'r', s @ ..] => {
											super::push(out, 34737);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'i', s @ ..] => super::t8::n27963(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => super::t8::n7383(s, out),
				['z', s @ ..] => super::t8::n7384(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t8::n1954(s, out),
		['i', s @ ..] => super::t8::n1955(s, out),
		['o', s @ ..] => super::t8::n1956(s, out),
		['t', s @ ..] => super::t8::n1957(s, out),
		['u', s @ ..] => super::t8::n1958(s, out),
		['ü', s @ ..] => super::t8::n1959(s, out),
		_ => false,
	}
}

/// Words ending -il.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1958);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7321);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 16756);
							s.is_empty()
						}
						['d', 'n', 'a', s @ ..] => {
							super::push(out, 16757);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16758);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 26899);
									match s {
										[] => true,
										['i', 'o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 34767);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 16759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7322);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 16760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7323);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 16761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7324);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16762);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 26900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 16763);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 16764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1959);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7325);
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 16765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1960);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1961);
			match s {
				[] => true,
				['e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7327);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7328);
					match s {
						[] => true,
						['k', 'o', 'r', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26901);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 26902);
									match s {
										[] => true,
										['p', 'm', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 34768);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 26903);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 26904);
									match s {
										[] => true,
										['t', 'i', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 34769);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 7329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16766);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t8::n1964(s, out),
		['f', s @ ..] => {
			super::push(out, 1963);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7340);
					match s {
						[] => true,
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 16795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7341);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26943);
									s.is_empty()
								}
								['n', 'e', 'h', s @ ..] => {
									super::push(out, 26944);
									match s {
										[] => true,
										['c', 'ä', 'l', 'f', 'g', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 34809);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 't', 'f', 'a', 'h', 'c', 's', 'n', 'e', 'g', 'i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 26945);
									s.is_empty()
								}
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
			super::push(out, 1964);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7342);
					match s {
						[] => true,
						['v', 'o', s @ ..] => {
							super::push(out, 16796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1965);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 7343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1966);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 7344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1967);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7345);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7346);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16797);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 16798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1968);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1969);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7348);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7349);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 16799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1970);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7351);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 16800);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 26946);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 26947);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7352);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7353);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7354);
					match s {
						[] => true,
						['o', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26948);
									s.is_empty()
								}
								['n', 'i', s @ ..] => {
									super::push(out, 26949);
									s.is_empty()
								}
								['o', 'd', s @ ..] => {
									super::push(out, 26950);
									s.is_empty()
								}
								['t', 'i', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 26951);
									s.is_empty()
								}
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
			super::push(out, 1971);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 7355);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7356);
					match s {
						[] => true,
						['e', s @ ..] => super::t8::n17222(s, out),
						['i', s @ ..] => super::t8::n17223(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t8::n7438(s, out),
				['s', s @ ..] => super::t8::n7439(s, out),
				['u', s @ ..] => super::t8::n7440(s, out),
				['x', s @ ..] => super::t8::n7441(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t8::n1974(s, out),
		['v', s @ ..] => super::t8::n1975(s, out),
		['x', s @ ..] => super::t8::n1976(s, out),
		['z', s @ ..] => super::t8::n1977(s, out),
		_ => false,
	}
}

/// Words ending -ll.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['a', s @ ..] => super::t8::n1979(s, out),
		['e', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7381);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16884);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 27021);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 27022);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 27023);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7382);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 16885);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 27024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16886);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 34880);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 34881);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 39994);
													match s {
														[] => true,
														['e', 'm', 'a', 'k', '\u{1}', s @ ..] => super::t8::n45597(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 34882);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 34883);
											s.is_empty()
										}
										['r', 'e', 't', s @ ..] => {
											super::push(out, 34884);
											match s {
												[] => true,
												['t', 'e', 'w', '\u{1}', s @ ..] => {
													super::push(out, 39995);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 34885);
											match s {
												[] => true,
												['g', 'e', s @ ..] => {
													super::push(out, 39996);
													match s {
														[] => true,
														['i', 'r', 'k', s @ ..] => super::t8::n45598(s, out),
														_ => false,
													}
												}
												['t', 'f', 'a', 'h', 'c', 's', 'l', 'l', 'e', 's', 'e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 39997);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', s @ ..] => {
											super::push(out, 34886);
											s.is_empty()
										}
										['y', s @ ..] => {
											super::push(out, 34887);
											match s {
												[] => true,
												['d', 'n', 'a', 'h', '\u{1}', s @ ..] => {
													super::push(out, 39998);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7383);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 't', 't', s @ ..] => {
									super::push(out, 27025);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 27026);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 16887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7384);
					match s {
						[] => true,
						['r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 16888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7385);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 7386);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 16889);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16890);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 27027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7387);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7388);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 16892);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 27028);
									match s {
										[] => true,
										['r', 'o', '\u{1}', s @ ..] => {
											super::push(out, 34888);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 27029);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 16893);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 16894);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16895);
							match s {
								[] => true,
								['o', 't', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 27030);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 16896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7389);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 16897);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 16898);
							s.is_empty()
						}
						['r', s @ ..] => super::t8::n17320(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t8::n7471(s, out),
				['s', s @ ..] => super::t8::n7472(s, out),
				['t', s @ ..] => super::t8::n7473(s, out),
				['u', s @ ..] => super::t8::n7474(s, out),
				['w', s @ ..] => super::t8::n7475(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t8::n1981(s, out),
		['o', s @ ..] => super::t8::n1982(s, out),
		['u', s @ ..] => super::t8::n1983(s, out),
		['y', s @ ..] => super::t8::n1984(s, out),
		['ä', s @ ..] => super::t8::n1985(s, out),
		['ö', s @ ..] => super::t8::n1986(s, out),
		['ü', s @ ..] => super::t8::n1987(s, out),
		_ => false,
	}
}

/// Words ending -mbol.
pub(crate) fn n7508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7427);
	match s {
		[] => true,
		['y', 's', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 27067);
					match s {
						[] => true,
						['a', 'n', 'i', 'm', 'r', 'e', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40006);
									s.is_empty()
								}
								['t', 'h', 'c', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 40007);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'u', s @ ..] => {
					super::push(out, 27068);
					match s {
						[] => true,
						['l', 'l', 'a', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 34904);
							s.is_empty()
						}
						['n', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 34905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'r', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 27069);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dol.
pub(crate) fn n1992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1990);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7430);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7431);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 16959);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eol.
pub(crate) fn n1993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1991);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 7432);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7433);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7434);
			match s {
				[] => true,
				['k', 'n', s @ ..] => {
					super::push(out, 16961);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ogol.
pub(crate) fn n7516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7435);
	s.is_empty()
}

/// Words ending -hol.
pub(crate) fn n1995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1993);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 7436);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16962);
					match s {
						[] => true,
						['i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 27070);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7437);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 16963);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							match s {
								[] => true,
								['l', 'y', s @ ..] => {
									match s {
										[] => true,
										['h', 't', 'e', 'm', s @ ..] => {
											super::push(out, 40008);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 40009);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 40010);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 40011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 34906);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 34907);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 34908);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									super::push(out, 34909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 16964);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 27071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7438);
			match s {
				[] => true,
				['h', 'p', 'a', s @ ..] => {
					super::push(out, 16965);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iol.
pub(crate) fn n1996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1994);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 7439);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 't', 'n', s @ ..] => {
			super::push(out, 7440);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7441);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16967);
					s.is_empty()
				}
				['t', 'i', s @ ..] => {
					super::push(out, 16968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kol.
pub(crate) fn n1997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1995);
	s.is_empty()
}

/// Words ending -lol.
pub(crate) fn n1998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1996);
	s.is_empty()
}

/// Words ending -mol.
pub(crate) fn n1999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1997);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7442);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7443);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7444);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 16969);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nol.
pub(crate) fn n2000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1998);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7445);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16970);
					match s {
						[] => true,
						['t', 'e', 'o', s @ ..] => {
							super::push(out, 27072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'o', 'r', 'p', 'o', s @ ..] => {
					super::push(out, 16971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7446);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 16972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7447);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7448);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27073);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 27074);
							match s {
								[] => true,
								['a', 'k', s @ ..] => {
									super::push(out, 34910);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40012);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7449);
			match s {
				[] => true,
				['y', s @ ..] => {
					super::push(out, 16974);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ool.
pub(crate) fn n2001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1999);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 7450);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 16975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7451);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 16976);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 16977);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7452);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pol.
pub(crate) fn n2002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2000);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7453);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 7454);
			match s {
				[] => true,
				['r', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 16979);
					s.is_empty()
				}
				['ü', 's', '\u{1}', s @ ..] => {
					super::push(out, 16980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7455);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 16981);
					match s {
						[] => true,
						['u', 'r', '\u{1}', s @ ..] => {
							super::push(out, 27075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7456);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7457);
			match s {
				[] => true,
				['e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 16982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7458);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7459);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 16983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rol.
pub(crate) fn n2003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2001);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7461);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 16984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7462);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 16985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7463);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7464);
			match s {
				[] => true,
				['s', 'n', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 16986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 7465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sol.
pub(crate) fn n2004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2002);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7466);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 16987);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 27076);
							s.is_empty()
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
				['m', s @ ..] => {
					super::push(out, 16988);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7467);
			match s {
				[] => true,
				['r', 'e', s @ ..] => {
					super::push(out, 16990);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 16991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7468);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tol.
pub(crate) fn n2005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2003);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7469);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7470);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 16992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7471);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 16993);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 27077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uol.
pub(crate) fn n2006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2004);
	s.is_empty()
}

/// Words ending -zol.
pub(crate) fn n2007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2005);
	s.is_empty()
}

/// Words ending -pl.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	match s {
		[] => true,
		['p', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 2006);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rl.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2007);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7472);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7473);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', 'i', 'e', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 27078);
									s.is_empty()
								}
								['n', 'e', 'z', 't', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 27079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 16994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7474);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 40013);
													s.is_empty()
												}
												['h', 'c', 's', 't', 'a', 'p', '\u{1}', s @ ..] => {
													super::push(out, 40014);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', 'f', 'u', 'a', 'h', 'e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 34911);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 'a', s @ ..] => {
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 34912);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 34913);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 34914);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 34915);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 34916);
											s.is_empty()
										}
										['h', '\u{1}', s @ ..] => {
											super::push(out, 34917);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 34918);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', 'm', '\u{1}', s @ ..] => {
							super::push(out, 16995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7475);
					match s {
						[] => true,
						['p', 'ö', s @ ..] => {
							super::push(out, 16996);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 16997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => super::t8::n7558(s, out),
				['h', s @ ..] => super::t8::n7559(s, out),
				['i', s @ ..] => {
					super::push(out, 7478);
					match s {
						[] => true,
						['z', 'n', 'i', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 17000);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => super::t8::n7561(s, out),
				['m', s @ ..] => super::t8::n7562(s, out),
				['p', s @ ..] => super::t8::n7563(s, out),
				['s', s @ ..] => {
					super::push(out, 7482);
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 17017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t8::n7565(s, out),
				['w', s @ ..] => {
					super::push(out, 7484);
					s.is_empty()
				}
				['z', s @ ..] => super::t8::n7567(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t8::n2011(s, out),
		['r', s @ ..] => super::t8::n2012(s, out),
		['u', s @ ..] => super::t8::n2013(s, out),
		['ö', s @ ..] => super::t8::n2014(s, out),
		_ => false,
	}
}

/// Words ending -sl.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['i', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 2013);
			s.is_empty()
		}
		['k', 'c', 'i', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 2014);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2015);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7492);
					s.is_empty()
				}
				['ä', 'h', 'u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 7493);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tl.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2016);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7494);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7495);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7496);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7497);
					match s {
						[] => true,
						['h', 'a', 'n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27109);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 27110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 7498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2017);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 7499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2018);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 7500);
					match s {
						[] => true,
						['e', 't', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 17023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'a', 'h', 'c', 's', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2019);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2020);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7502);
					match s {
						[] => true,
						['o', 'x', 'a', s @ ..] => {
							super::push(out, 17024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2021);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17025);
							s.is_empty()
						}
						['n', 'e', 'h', 'c', 'ü', 'k', '\u{1}', s @ ..] => {
							super::push(out, 17026);
							s.is_empty()
						}
						['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 17027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'k', 'n', 'e', 'z', 't', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7503);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2022);
			match s {
				[] => true,
				['e', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7505);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ul.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2023);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7506);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17028);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7508);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17030);
							s.is_empty()
						}
						['n', 'e', 'w', s @ ..] => {
							super::push(out, 17031);
							s.is_empty()
						}
						['r', 'e', 't', s @ ..] => {
							super::push(out, 17032);
							match s {
								[] => true,
								['s', 'ä', 'l', '\u{1}', s @ ..] => {
									super::push(out, 27111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7509);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 17033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['i', 'x', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7510);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7511);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27112);
									s.is_empty()
								}
								['r', 'a', 'l', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 27113);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['n', 'o', 'i', 't', 'a', 's', 'i', 'n', 'o', 'i', '\u{1}', s @ ..] => {
											super::push(out, 34951);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 34952);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', 'x', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2025);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7513);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7514);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 27114);
									s.is_empty()
								}
								['s', 'g', 'e', 'i', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 27115);
									s.is_empty()
								}
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
			super::push(out, 2026);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7515);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7516);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7517);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2028);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7519);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 2029);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2030);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2031);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2032);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2033);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7523);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2034);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7524);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2035);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7526);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vl.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	s.is_empty()
}

/// Words ending -wl.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2036);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7527);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 7528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2037);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2038);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7529);
					match s {
						[] => true,
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 17037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7531);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xl.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 2039);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yl.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2040);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 7532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2041);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2042);
			match s {
				[] => true,
				['a', 't', 'n', 'e', 'f', 'r', s @ ..] => {
					super::push(out, 7533);
					s.is_empty()
				}
				['e', 'i', s @ ..] => {
					super::push(out, 7534);
					s.is_empty()
				}
				['o', 'b', 'r', 'a', 'c', 'a', s @ ..] => {
					match s {
						[] => true,
						['t', 'n', 'e', 'p', 'n', s @ ..] => {
							super::push(out, 17039);
							s.is_empty()
						}
						['x', 'e', 'h', 'n', s @ ..] => {
							super::push(out, 17040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2043);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2044);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2045);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7535);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7536);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7537);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zl.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['e', 'p', 's', '\u{1}', s @ ..] => {
			super::push(out, 2046);
			s.is_empty()
		}
		['n', 'a', 'l', 'f', 'p', 'h', 'c', 's', 'i', 'e', 'l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 2047);
			s.is_empty()
		}
		['r', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 2048);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7538);
					match s {
						[] => true,
						['k', 'h', 'c', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 17041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 7539);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äl.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2049);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ål.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2050);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 17042);
							s.is_empty()
						}
						['k', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 17043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öl.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2052);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7541);
					match s {
						[] => true,
						['r', 'e', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 17044);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 17045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2053);
			match s {
				[] => true,
				['r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7542);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', s @ ..] => {
			super::push(out, 2054);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2055);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2056);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'a', s @ ..] => {
			super::push(out, 2057);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7544);
					match s {
						[] => true,
						['w', '\u{1}', s @ ..] => {
							super::push(out, 17046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 17047);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17048);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 17049);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 27116);
									s.is_empty()
								}
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
			super::push(out, 2058);
			match s {
				[] => true,
				['a', 's', 'i', s @ ..] => {
					super::push(out, 7545);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7546);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2059);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7548);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 17050);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 17051);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 17052);
							s.is_empty()
						}
						['n', 'o', s @ ..] => {
							super::push(out, 17053);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17054);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 27117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 17055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7549);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 17056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2060);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'i', s @ ..] => {
							super::push(out, 17057);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2061);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7551);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7552);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7553);
					match s {
						[] => true,
						['n', 'i', 'z', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 17059);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 17060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2062);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2063);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 2064);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7554);
					match s {
						[] => true,
						['i', 'm', 'r', 'e', 'f', 'f', 'e', 'f', 'p', '\u{1}', s @ ..] => {
							super::push(out, 17061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7555);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 17062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ül.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2065);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7556);
					match s {
						[] => true,
						['ü', 'b', 'l', 'e', 'b', 'a', 'n', 'h', 'c', 's', 'k', 'n', 'a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 17063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2066);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7557);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7558);
					match s {
						[] => true,
						['d', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 17064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7559);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word gulfstream.
pub(crate) fn n42521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40039);
	s.is_empty()
}

/// Words ending -instream.
pub(crate) fn n42522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40040);
	s.is_empty()
}

/// The word lithiumamalgam.
pub(crate) fn n42523(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40041);
	s.is_empty()
}

/// Words ending -rham.
pub(crate) fn n7659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7571);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 17074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -jam.
pub(crate) fn n2077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2073);
	s.is_empty()
}

/// Words ending -lam.
pub(crate) fn n2078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2074);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7572);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7573);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7574);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 17075);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 17076);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 17077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mam.
pub(crate) fn n2079(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2075);
	match s {
		[] => true,
		['a', 'h', '\u{1}', s @ ..] => {
			super::push(out, 7575);
			s.is_empty()
		}
		['i', '\u{1}', s @ ..] => {
			super::push(out, 7576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nam.
pub(crate) fn n2080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2076);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 7577);
			match s {
				[] => true,
				['c', 'i', 'e', 'l', 'n', s @ ..] => {
					super::push(out, 17078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7578);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 17079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pam.
pub(crate) fn n2081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2077);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7579);
			match s {
				[] => true,
				['t', 't', 'u', '\u{1}', s @ ..] => {
					super::push(out, 17080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7580);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 7581);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 17081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ram.
pub(crate) fn n2082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2078);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7583);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 7584);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17082);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 17083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7585);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 17084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 7586);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17085);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 17086);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 27124);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 27125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 17087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 7587);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 17088);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 34958);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 34959);
									s.is_empty()
								}
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
			super::push(out, 7588);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 17089);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 27126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 17090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 7589);
			match s {
				[] => true,
				['e', 'i', 'r', 'o', 'e', 'h', 't', '\u{1}', s @ ..] => {
					super::push(out, 17091);
					s.is_empty()
				}
				['r', 'e', 'd', 'n', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 17092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7590);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7591);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 17093);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 17094);
					match s {
						[] => true,
						['p', 'm', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 27127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 17095);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sam.
pub(crate) fn n2083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2079);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7592);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 7593);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 17096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7594);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 17097);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 17098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7595);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7596);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 17099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tam.
pub(crate) fn n2084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2080);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7598);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 7599);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7600);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wam.
pub(crate) fn n2085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2081);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yam.
pub(crate) fn n2086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2082);
	s.is_empty()
}

/// Words ending -bm.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 2083);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dm.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	s.is_empty()
}

/// Words ending -em.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2084);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 2085);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7602);
					match s {
						[] => true,
						['l', 'a', 'b', 't', 'e', 'k', 's', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 17100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2086);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7603);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 17101);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27128);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 17102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7604);
					match s {
						[] => true,
						['p', 'l', 'o', 'z', '\u{1}', s @ ..] => {
							super::push(out, 17103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7605);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7606);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 17105);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17106);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 7607);
					match s {
						[] => true,
						['n', 'e', 'g', 'n', 'u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 17108);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 17109);
							match s {
								[] => true,
								['i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 27129);
									s.is_empty()
								}
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
			super::push(out, 2087);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7608);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 17110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7609);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7610);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', s @ ..] => {
					super::push(out, 7611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2088);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7612);
					match s {
						[] => true,
						['p', 'e', 's', 'e', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 17112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2089);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7614);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 17113);
							match s {
								[] => true,
								['u', 'h', 't', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 27130);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 7615);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 17114);
							s.is_empty()
						}
						['o', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 27131);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 27132);
									match s {
										[] => true,
										['l', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 34960);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'e', 'm', s @ ..] => {
									super::push(out, 27133);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 34961);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 27134);
									match s {
										[] => true,
										['g', 'n', 'u', 'r', s @ ..] => {
											super::push(out, 34962);
											match s {
												[] => true,
												['e', 'i', 'd', s @ ..] => {
													super::push(out, 40042);
													match s {
														[] => true,
														['r', 'a', 't', 'e', 'r', '\u{1}', s @ ..] => super::t8::n45605(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 34963);
											match s {
												[] => true,
												['o', 'i', 't', 'k', 'u', s @ ..] => {
													super::push(out, 40043);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 7616);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 17115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7617);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 17116);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2090);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2091);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7618);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 17117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 7619);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7620);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17118);
							match s {
								[] => true,
								['h', 't', '-', s @ ..] => {
									super::push(out, 27135);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2092);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7622);
					match s {
						[] => true,
						['n', 'r', 'e', 'h', 'c', 's', 't', '\u{1}', s @ ..] => {
							super::push(out, 17120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7623);
					match s {
						[] => true,
						['a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 17121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t8::n2097(s, out),
		['x', s @ ..] => {
			super::push(out, 2094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fm.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	s.is_empty()
}

/// Words ending -hm.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2095);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 7628);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17126);
							match s {
								[] => true,
								['e', 'f', 'f', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 27137);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17127);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2096);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7629);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2097);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7630);
					s.is_empty()
				}
				['a', 'g', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7631);
					s.is_empty()
				}
				['g', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7632);
					s.is_empty()
				}
				['o', 'l', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 7633);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 7634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -im.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2099);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2100);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2101);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7636);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7637);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7638);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2102);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2103);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7640);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7641);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17129);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 17130);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 27138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 17131);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17132);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'i', s @ ..] => {
							super::push(out, 17133);
							s.is_empty()
						}
						['s', 'g', 'n', 'u', 'h', s @ ..] => {
							super::push(out, 17134);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 17135);
							match s {
								[] => true,
								['r', 'o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 27140);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 7642);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7643);
					match s {
						[] => true,
						['h', 'c', 's', 'r', s @ ..] => {
							super::push(out, 17136);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 27141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7644);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17137);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17138);
							match s {
								[] => true,
								['e', 't', 't', 'ü', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 27142);
									s.is_empty()
								}
								['h', 'ä', 'z', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 27143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 17139);
							s.is_empty()
						}
						['r', 'e', 'd', s @ ..] => {
							super::push(out, 17140);
							match s {
								[] => true,
								['n', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 27144);
									s.is_empty()
								}
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
			super::push(out, 2104);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2105);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7646);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7647);
					match s {
						[] => true,
						['u', 'm', s @ ..] => {
							match s {
								[] => true,
								['-', 't', 'h', 'c', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 27145);
									s.is_empty()
								}
								['t', 'h', 'c', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 27146);
									s.is_empty()
								}
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
			super::push(out, 2106);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7648);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7649);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2107);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2108);
			match s {
				[] => true,
				['c', 'i', 'v', 'n', 'o', 'i', 'h', 's', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7652);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 7653);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lm.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2109);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7655);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7656);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 17141);
							match s {
								[] => true,
								['o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 27147);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 17142);
							match s {
								[] => true,
								['a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 27148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7657);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7658);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17143);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 17144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17145);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17146);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 17147);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27149);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 27150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7660);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 17148);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27151);
									s.is_empty()
								}
								['k', 'a', 'b', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 27152);
									s.is_empty()
								}
								['n', 'e', 't', 't', 'e', 'r', 'a', 'g', 'i', 'z', '\u{1}', s @ ..] => {
									super::push(out, 27153);
									s.is_empty()
								}
								['s', 'k', 'a', 'b', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 27154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 7661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2110);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17149);
							match s {
								[] => true,
								['e', 'm', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 27155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17150);
							match s {
								[] => true,
								['e', 't', 't', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 27156);
									s.is_empty()
								}
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
			super::push(out, 2111);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 17151);
							match s {
								[] => true,
								['d', 'o', 'o', 'w', 'y', 'l', 'l', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 27157);
									s.is_empty()
								}
								['e', 's', 'e', 's', 'r', 'o', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 27158);
									s.is_empty()
								}
								['s', 't', 'r', 'a', '-', 'l', 'a', 'i', 't', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 27159);
									s.is_empty()
								}
								_ => false,
							}
						}
						['a', 'i', s @ ..] => {
							super::push(out, 17152);
							match s {
								[] => true,
								['p', 'm', 'y', 'l', 'o', '\u{1}', s @ ..] => {
									super::push(out, 27160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 's', s @ ..] => {
							super::push(out, 17153);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							super::push(out, 17154);
							match s {
								[] => true,
								['d', 'n', 'a', 'k', 's', '\u{1}', s @ ..] => {
									super::push(out, 27161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 27162);
									s.is_empty()
								}
								['o', 'i', 't', 'a', s @ ..] => {
									super::push(out, 27163);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'i', s @ ..] => {
							super::push(out, 17155);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 27164);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 27165);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 27166);
									match s {
										[] => true,
										['u', 'e', 't', 'n', 'e', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 34977);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'i', s @ ..] => {
							super::push(out, 17156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2112);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7662);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7663);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7664);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 7665);
					s.is_empty()
				}
				['n', 'e', 't', 't', 'o', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2113);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mm.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2114);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7668);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 17157);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 27167);
									match s {
										[] => true,
										['t', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 34978);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17158);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17159);
							match s {
								[] => true,
								['a', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 34979);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 34980);
											s.is_empty()
										}
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
					super::push(out, 7669);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 7670);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 17160);
							match s {
								[] => true,
								['g', 'r', 'i', 'b', 'e', 'g', 'z', s @ ..] => {
									super::push(out, 27168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17161);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 17162);
							match s {
								[] => true,
								['c', 's', s @ ..] => super::t8::n28305(s, out),
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 17163);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17164);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 27169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 17165);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t8::n7763(s, out),
				['t', s @ ..] => super::t8::n7764(s, out),
				['w', s @ ..] => super::t8::n7765(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t8::n2119(s, out),
		['i', s @ ..] => super::t8::n2120(s, out),
		['u', s @ ..] => super::t8::n2121(s, out),
		_ => false,
	}
}

/// Words ending -om.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2118);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 7683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2119);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7684);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 17174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7685);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7686);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17175);
							match s {
								[] => true,
								['d', 'i', 'l', 'a', 'v', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 27181);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17176);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7687);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7688);
					match s {
						[] => true,
						['e', 's', 'i', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 17177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7689);
					match s {
						[] => true,
						['l', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 17178);
							s.is_empty()
						}
						['r', 'e', 't', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 17179);
							s.is_empty()
						}
						['t', 'i', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 17180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7690);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7691);
					match s {
						[] => true,
						['l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 17181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2120);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2121);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7692);
					s.is_empty()
				}
				['g', 'n', 'a', 'h', s @ ..] => {
					super::push(out, 7693);
					match s {
						[] => true,
						['p', 'm', 'y', 'l', '\u{1}', s @ ..] => {
							super::push(out, 17182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 7694);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							super::push(out, 17183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2122);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7695);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 17184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2123);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7697);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2124);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2125);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7700);
					match s {
						[] => true,
						['l', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 17185);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7701);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7702);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7703);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7704);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 17187);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 17188);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 27182);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 35010);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 35011);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 7705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2126);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7706);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17189);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 17190);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17191);
							match s {
								[] => true,
								['t', 'f', 'a', 'h', 'c', 's', 't', 'r', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 27183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7707);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7708);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 17192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2127);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7709);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7710);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => super::t8::n7802(s, out),
				['e', s @ ..] => {
					super::push(out, 7712);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7713);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 17196);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 17197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7714);
					match s {
						[] => true,
						['c', 'o', s @ ..] => {
							super::push(out, 17198);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 27187);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 27188);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 27189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7715);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7716);
					match s {
						[] => true,
						['e', s @ ..] => super::t8::n17645(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t8::n7808(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t8::n2132(s, out),
		['t', s @ ..] => super::t8::n2133(s, out),
		['y', s @ ..] => super::t8::n2134(s, out),
		['z', s @ ..] => super::t8::n2135(s, out),
		_ => false,
	}
}

/// Words ending -rm.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2132);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7722);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 7723);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 17208);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7724);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17209);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 27207);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 17210);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 27208);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 27209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17211);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 27210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7725);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 17212);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17213);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 27211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7726);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 7727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17214);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 17215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7728);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17216);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 27212);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'u', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 35036);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 35037);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'f', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 27213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7729);
					match s {
						[] => true,
						['e', 'u', s @ ..] => {
							super::push(out, 17217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7730);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17218);
							match s {
								[] => true,
								['u', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 27214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7731);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7732);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 7733);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 17219);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 40090);
													s.is_empty()
												}
												['h', 'c', 'r', s @ ..] => {
													super::push(out, 40091);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 35038);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											super::push(out, 35039);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ß', s @ ..] => {
					super::push(out, 7734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2133);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7735);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7737);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 17221);
							match s {
								[] => true,
								['s', 't', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 27215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7738);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7740);
					match s {
						[] => true,
						['c', 's', 'l', 'l', 'a', 'f', s @ ..] => {
							super::push(out, 17222);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 27216);
									s.is_empty()
								}
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
			super::push(out, 2135);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7741);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 17223);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 17224);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 27217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 17225);
							s.is_empty()
						}
						['n', 'e', 's', s @ ..] => {
							super::push(out, 17226);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 27218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17227);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 27219);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 35040);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 35041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 17228);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 27220);
									match s {
										[] => true,
										['i', 'e', 'k', 'g', s @ ..] => {
											super::push(out, 35042);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 40092);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'a', s @ ..] => {
							super::push(out, 17229);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 27221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7742);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 17230);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t8::n2140(s, out),
		['ä', s @ ..] => super::t8::n2141(s, out),
		['ü', s @ ..] => {
			super::push(out, 2138);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 7747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sm.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2139);
			match s {
				[] => true,
				['t', 'u', 'o', 'b', 'a', 't', 'a', 'h', 'w', '\u{1}', s @ ..] => {
					super::push(out, 7748);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tm.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -um.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2141);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7749);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 17242);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 27234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'i', 't', s @ ..] => {
							super::push(out, 17243);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 17244);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 27235);
									match s {
										[] => true,
										['n', 'r', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 35048);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', 'p', 'a', 'z', s @ ..] => {
											super::push(out, 35049);
											s.is_empty()
										}
										['z', 'r', 'u', 'p', '\u{1}', s @ ..] => {
											super::push(out, 35050);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ö', s @ ..] => {
									super::push(out, 27236);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35051);
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
										['d', 'i', 'u', s @ ..] => {
											super::push(out, 35052);
											s.is_empty()
										}
										['g', 'l', 'a', s @ ..] => {
											super::push(out, 35053);
											s.is_empty()
										}
										['i', 'l', s @ ..] => {
											super::push(out, 35054);
											s.is_empty()
										}
										['n', 'r', s @ ..] => {
											super::push(out, 35055);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 35056);
											match s {
												[] => true,
												['a', s @ ..] => {
													match s {
														[] => true,
														['j', 'a', 'u', 'g', '\u{1}', s @ ..] => super::t8::n45662(s, out),
														['u', s @ ..] => super::t8::n45663(s, out),
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
									super::push(out, 27237);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 35057);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17245);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 35058);
											s.is_empty()
										}
										['f', 'f', s @ ..] => {
											super::push(out, 35059);
											s.is_empty()
										}
										['i', 'n', s @ ..] => {
											super::push(out, 35060);
											match s {
												[] => true,
												['r', 'u', 't', '\u{1}', s @ ..] => {
													super::push(out, 40093);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 't', 'e', s @ ..] => {
											super::push(out, 35061);
											match s {
												[] => true,
												['l', 'k', '\u{1}', s @ ..] => {
													super::push(out, 40094);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ß', 'i', s @ ..] => {
							super::push(out, 17246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7750);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 7751);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => super::t8::n28386(s, out),
								['e', s @ ..] => super::t8::n28387(s, out),
								['h', s @ ..] => super::t8::n28388(s, out),
								['r', 'e', 'i', 'b', s @ ..] => super::t8::n28389(s, out),
								['s', s @ ..] => super::t8::n28390(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t8::n7843(s, out),
				['r', s @ ..] => super::t8::n7844(s, out),
				['s', 'l', s @ ..] => super::t8::n7845(s, out),
				['z', s @ ..] => super::t8::n7846(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t8::n2146(s, out),
		['c', s @ ..] => super::t8::n2147(s, out),
		['d', s @ ..] => super::t8::n2148(s, out),
		['e', s @ ..] => super::t8::n2149(s, out),
		['f', s @ ..] => super::t8::n2150(s, out),
		['g', s @ ..] => super::t8::n2151(s, out),
		['h', s @ ..] => super::t8::n2152(s, out),
		['i', s @ ..] => super::t8::n2153(s, out),
		['k', s @ ..] => super::t8::n2154(s, out),
		['l', s @ ..] => super::t8::n2155(s, out),
		['m', s @ ..] => super::t8::n2156(s, out),
		['n', s @ ..] => super::t8::n2157(s, out),
		['p', s @ ..] => super::t8::n2158(s, out),
		['r', s @ ..] => super::t8::n2159(s, out),
		['s', s @ ..] => super::t8::n2160(s, out),
		['t', s @ ..] => super::t8::n2161(s, out),
		['u', s @ ..] => super::t8::n2162(s, out),
		['v', s @ ..] => super::t8::n2163(s, out),
		['x', s @ ..] => super::t8::n2164(s, out),
		['ä', s @ ..] => super::t8::n2165(s, out),
		_ => false,
	}
}

/// Words ending -wm.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2162);
			match s {
				[] => true,
				['l', 'l', 'a', 'b', 'ß', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ym.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 294);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2163);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 7846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2164);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2165);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2166);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 17447);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17448);
							match s {
								[] => true,
								['h', 't', 'e', '\u{1}', s @ ..] => {
									super::push(out, 27449);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 27450);
									match s {
										[] => true,
										['e', 't', '\u{1}', s @ ..] => {
											super::push(out, 35187);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', 's', '\u{1}', s @ ..] => {
									super::push(out, 27451);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17449);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 27452);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35188);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', s @ ..] => {
									super::push(out, 27453);
									match s {
										[] => true,
										['h', 't', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 35189);
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
									super::push(out, 27454);
									match s {
										[] => true,
										['p', s @ ..] => {
											super::push(out, 35190);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 27455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 17450);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									super::push(out, 27456);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35191);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 27457);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 27458);
									s.is_empty()
								}
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
			super::push(out, 2167);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7848);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 7849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äm.
pub(crate) fn n295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 295);
	s.is_empty()
}

/// Words ending -öm.
pub(crate) fn n296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2169);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -üm.
pub(crate) fn n297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2170);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2171);
			match s {
				[] => true,
				['ü', s @ ..] => {
					super::push(out, 7851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2172);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7852);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17451);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 27459);
									match s {
										[] => true,
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 35192);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17452);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 27460);
									s.is_empty()
								}
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35193);
											s.is_empty()
										}
										['-', 'n', 'a', 'm', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 35194);
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['n', 'i', 'd', 'r', 'a', 'b', 'a', 'g', '\u{1}', s @ ..] => {
													super::push(out, 40165);
													s.is_empty()
												}
												['s', 'i', 'e', 'r', '\u{1}', s @ ..] => {
													super::push(out, 40166);
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
														['d', 'i', 'e', 's', '\u{1}', s @ ..] => super::t8::n45697(s, out),
														['e', 'w', 'o', 'l', 'l', 'a', 'h', '\u{1}', s @ ..] => super::t8::n45698(s, out),
														['f', 'f', 'a', '\u{1}', s @ ..] => super::t8::n45699(s, out),
														['r', 'r', 'a', 'n', 'f', 'o', 'h', '\u{1}', s @ ..] => super::t8::n45700(s, out),
														['u', 'a', 'r', 'f', '\u{1}', s @ ..] => super::t8::n45701(s, out),
														['v', 'r', 'e', 'n', '\u{1}', s @ ..] => super::t8::n45702(s, out),
														['z', 't', 'a', 'k', '\u{1}', s @ ..] => super::t8::n45703(s, out),
														_ => false,
													}
												}
												['n', 'a', 'm', s @ ..] => {
													match s {
														[] => true,
														['n', 'e', 's', 'n', 'e', 's', '\u{1}', s @ ..] => super::t8::n45704(s, out),
														['s', 't', 'h', 'c', 'a', 'n', 'h', 'i', 'e', 'w', '\u{1}', s @ ..] => super::t8::n45705(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['d', 'e', 'l', '\u{1}', s @ ..] => super::t8::n45706(s, out),
														['m', 'm', 'o', 's', '\u{1}', s @ ..] => super::t8::n45707(s, out),
														['n', 'n', 'ä', 'm', '\u{1}', s @ ..] => super::t8::n45708(s, out),
														['t', 'a', 'e', 'h', 't', '\u{1}', s @ ..] => super::t8::n45709(s, out),
														_ => false,
													}
												}
												['i', 'p', 'm', 'a', 'v', '\u{1}', s @ ..] => {
													super::push(out, 40167);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['a', 'v', 'e', '\u{1}', s @ ..] => {
													super::push(out, 40168);
													s.is_empty()
												}
												['f', 'l', 'o', 'w', '\u{1}', s @ ..] => {
													super::push(out, 40169);
													s.is_empty()
												}
												['h', 'c', 'n', 'ö', 'm', '\u{1}', s @ ..] => {
													super::push(out, 40170);
													s.is_empty()
												}
												['l', 'e', 'f', 'u', 'e', 't', '\u{1}', s @ ..] => {
													super::push(out, 40171);
													s.is_empty()
												}
												['m', s @ ..] => {
													super::push(out, 40172);
													s.is_empty()
												}
												['u', 'a', 'l', 'o', 'k', 'i', 'n', '\u{1}', s @ ..] => {
													super::push(out, 40173);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ēm.
pub(crate) fn n298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	s.is_empty()
}

/// Words ending -an.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2173);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7853);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2174);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 7855);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 17453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['a', s @ ..] => {
					super::push(out, 7856);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 17454);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17456);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 17457);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 17458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 7858);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7859);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7860);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17459);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7861);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2175);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7862);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7863);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 17461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7864);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7865);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7866);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 17462);
							s.is_empty()
						}
						['á', s @ ..] => {
							super::push(out, 17463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7867);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7868);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2177);
			match s {
				[] => true,
				['c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7870);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 7871);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7872);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7873);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 7874);
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 17464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2178);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7875);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 17465);
							match s {
								[] => true,
								['a', 'b', s @ ..] => {
									match s {
										[] => true,
										['s', 's', 'u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 35195);
											s.is_empty()
										}
										['ß', 'u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 35196);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 17466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2179);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7876);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7877);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7878);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17467);
							match s {
								[] => true,
								['m', 'o', s @ ..] => {
									super::push(out, 27461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 17468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7879);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 17469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7880);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17470);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 27462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17471);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 27463);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 35197);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', 't', s @ ..] => {
									super::push(out, 27464);
									s.is_empty()
								}
								['l', 'a', 'r', s @ ..] => {
									super::push(out, 27465);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 27466);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 27467);
									match s {
										[] => true,
										['g', 'n', 'u', 's', s @ ..] => {
											super::push(out, 35198);
											s.is_empty()
										}
										['n', 'o', 'i', 't', 'a', 'k', s @ ..] => {
											super::push(out, 35199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 17472);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 27468);
									s.is_empty()
								}
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
			super::push(out, 2180);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7881);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17473);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 17474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 7882);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7883);
					match s {
						[] => true,
						['o', 'k', s @ ..] => {
							super::push(out, 17475);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 17476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7884);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 17477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7885);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17478);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 17479);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 27469);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 27470);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 17480);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 27471);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 27472);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 35200);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 35201);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t8::n2185(s, out),
		['j', s @ ..] => {
			super::push(out, 2182);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7894);
					s.is_empty()
				}
				['r', 'e', 'd', 'e', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7895);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t8::n2187(s, out),
		['l', s @ ..] => super::t8::n2188(s, out),
		['m', s @ ..] => super::t8::n2189(s, out),
		['n', s @ ..] => {
			super::push(out, 2186);
			match s {
				[] => true,
				['a', s @ ..] => super::t8::n8025(s, out),
				['k', s @ ..] => super::t8::n8026(s, out),
				['n', 'i', s @ ..] => super::t8::n8027(s, out),
				['o', s @ ..] => super::t8::n8028(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t8::n2191(s, out),
		['p', s @ ..] => super::t8::n2192(s, out),
		['r', s @ ..] => super::t8::n2193(s, out),
		['s', s @ ..] => super::t8::n2194(s, out),
		['t', s @ ..] => super::t8::n2195(s, out),
		['u', s @ ..] => super::t8::n2196(s, out),
		['v', s @ ..] => super::t8::n2197(s, out),
		['w', s @ ..] => super::t8::n2198(s, out),
		['x', s @ ..] => super::t8::n2199(s, out),
		['y', s @ ..] => super::t8::n2200(s, out),
		['z', s @ ..] => super::t8::n2201(s, out),
		['ō', s @ ..] => super::t8::n2202(s, out),
		_ => false,
	}
}

/// Words ending -en.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2201);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2202);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7989);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 17615);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 27549);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 17616);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 27550);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 27551);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 27552);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 35228);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 27553);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 35229);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 35230);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 35231);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17617);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 35232);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 35233);
											s.is_empty()
										}
										['e', 'b', s @ ..] => {
											super::push(out, 35234);
											s.is_empty()
										}
										['h', 'c', s @ ..] => {
											super::push(out, 35235);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 35236);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 35237);
											match s {
												[] => true,
												['e', 'n', s @ ..] => {
													super::push(out, 40177);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['t', 'n', s @ ..] => {
													super::push(out, 40178);
													s.is_empty()
												}
												['v', s @ ..] => {
													super::push(out, 40179);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'u', s @ ..] => {
											super::push(out, 35238);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 27554);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 17618);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 27555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t8::n8087(s, out),
				['i', s @ ..] => super::t8::n8088(s, out),
				['l', s @ ..] => super::t8::n8089(s, out),
				['n', s @ ..] => super::t8::n8090(s, out),
				['o', s @ ..] => super::t8::n8091(s, out),
				['r', s @ ..] => super::t8::n8092(s, out),
				['u', s @ ..] => super::t8::n8093(s, out),
				['ü', s @ ..] => {
					super::push(out, 7997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t8::n2207(s, out),
		['d', s @ ..] => super::t8::n2208(s, out),
		['e', s @ ..] => super::t8::n2209(s, out),
		['f', s @ ..] => super::t9::n2210(s, out),
		['g', s @ ..] => super::t9::n2211(s, out),
		['h', s @ ..] => super::t9::n2212(s, out),
		['i', s @ ..] => super::t9::n2213(s, out),
		['k', s @ ..] => super::t9::n2214(s, out),
		['l', s @ ..] => super::t9::n2215(s, out),
		['m', s @ ..] => super::t9::n2216(s, out),
		['n', s @ ..] => super::t9::n2217(s, out),
		['p', s @ ..] => super::t9::n2218(s, out),
		['r', s @ ..] => super::t9::n2219(s, out),
		['s', s @ ..] => super::t9::n2220(s, out),
		['t', s @ ..] => super::t9::n2221(s, out),
		['u', s @ ..] => super::t9::n2222(s, out),
		['v', s @ ..] => super::t9::n2223(s, out),
		['w', s @ ..] => super::t9::n2224(s, out),
		['x', s @ ..] => super::t9::n2225(s, out),
		['y', s @ ..] => super::t9::n2226(s, out),
		['z', s @ ..] => super::t9::n2227(s, out),
		['ß', s @ ..] => super::t9::n2228(s, out),
		['ä', s @ ..] => super::t9::n2229(s, out),
		['é', s @ ..] => super::t9::n2230(s, out),
		_ => false,
	}
}

/// Words ending -bahn.
pub(crate) fn n8308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8211);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 18084);
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					super::push(out, 28129);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'e', s @ ..] => {
			super::push(out, 18085);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 18086);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 18087);
			match s {
				[] => true,
				['n', 'i', 'r', 'e', s @ ..] => {
					super::push(out, 28131);
					s.is_empty()
				}
				['r', 'e', 'b', 'f', s @ ..] => {
					super::push(out, 28132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 28133);
					s.is_empty()
				}
				['i', 'e', 's', 'x', s @ ..] => {
					super::push(out, 28134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 28135);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 28136);
					match s {
						[] => true,
						['e', 'u', s @ ..] => {
							super::push(out, 35601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 18088);
			s.is_empty()
		}
		['s', 'h', s @ ..] => {
			super::push(out, 18089);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 18090);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 28137);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nhahn.
pub(crate) fn n18590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18093);
	match s {
		[] => true,
		['e', 'n', 'a', 's', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 28139);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -phahn.
pub(crate) fn n18591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18094);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 28140);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rhahn.
pub(crate) fn n18592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18095);
	match s {
		[] => true,
		['e', 'v', s @ ..] => {
			super::push(out, 28141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -shahn.
pub(crate) fn n18593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18096);
	s.is_empty()
}

/// Words ending -jahn.
pub(crate) fn n8310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8213);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 18097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kahn.
pub(crate) fn n8311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8214);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rahn.
pub(crate) fn n8312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8215);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 18099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wahn.
pub(crate) fn n8313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8216);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18100);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 18101);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 18102);
			s.is_empty()
		}
		['m', 'u', 's', 'n', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 18103);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 18104);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 35602);
							match s {
								[] => true,
								['z', 'o', 't', 'a', 'm', 'r', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 40339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 's', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 42963);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 44197);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 42964);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ö', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 40340);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 35603);
							match s {
								[] => true,
								['ö', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 40341);
									s.is_empty()
								}
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
			super::push(out, 18105);
			match s {
				[] => true,
				['e', 'd', 'n', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 35604);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 35605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 18106);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'u', 's', 'r', 'e', 'f', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 35606);
							s.is_empty()
						}
						['i', 'e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 40342);
									s.is_empty()
								}
								['k', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'i', 'l', 'n', 'n', 'ä', 'm', '\u{1}', s @ ..] => {
											super::push(out, 42965);
											s.is_empty()
										}
										['r', 'e', 'b', 'u', 'a', 's', '\u{1}', s @ ..] => {
											super::push(out, 42966);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zahn.
pub(crate) fn n8314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8217);
	match s {
		[] => true,
		['k', 'c', 'e', '\u{1}', s @ ..] => {
			super::push(out, 18108);
			s.is_empty()
		}
		['l', 'a', s @ ..] => {
			super::push(out, 18109);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 18110);
			match s {
				[] => true,
				['e', 'f', s @ ..] => {
					super::push(out, 28142);
					match s {
						[] => true,
						['f', 'a', '\u{1}', s @ ..] => {
							super::push(out, 35607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			super::push(out, 18111);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 28143);
					s.is_empty()
				}
				['i', 't', 'b', 'u', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 28144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 's', s @ ..] => {
			super::push(out, 18112);
			match s {
				[] => true,
				['o', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 28145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ehn.
pub(crate) fn n2233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2229);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 8218);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 18113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 8219);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 18114);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ihn.
pub(crate) fn n2234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2230);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8220);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ohn.
pub(crate) fn n2235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2231);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 8221);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 8222);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 18115);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 35608);
									s.is_empty()
								}
								['n', 'i', 'm', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 35609);
									s.is_empty()
								}
								['r', 'u', 't', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 35610);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 18116);
					match s {
						[] => true,
						['t', 't', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 35611);
									s.is_empty()
								}
								['u', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 35612);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 18117);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 18118);
					match s {
						[] => true,
						['e', 't', s @ ..] => {
							super::push(out, 28146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18119);
					match s {
						[] => true,
						['s', 'e', 'd', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 28147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8223);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 18120);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 18121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8224);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 18122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8225);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8226);
			match s {
				[] => true,
				['e', 'd', s @ ..] => {
					super::push(out, 18124);
					s.is_empty()
				}
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 28148);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 35613);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 28149);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 28150);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 35614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18125);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 28151);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 28152);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 28153);
							s.is_empty()
						}
						['r', 'e', 'l', s @ ..] => {
							super::push(out, 28154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 8227);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 18126);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uhn.
pub(crate) fn n2236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2232);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 8228);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 18127);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 28155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'i', s @ ..] => {
					super::push(out, 18128);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 18129);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 28156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'c', s @ ..] => {
					super::push(out, 18130);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öhn.
pub(crate) fn n2237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2233);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 8229);
			match s {
				[] => true,
				['e', 's', 'i', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 18131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 8230);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8231);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -in.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2234);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8232);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 18132);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 18133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 2235);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8234);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 18134);
							match s {
								[] => true,
								['d', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 28157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8235);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18135);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 18136);
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 28158);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 18137);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 28159);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 28160);
									s.is_empty()
								}
								['p', 'p', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 28161);
									s.is_empty()
								}
								['t', 't', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 28162);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'a', 'b', 't', 's', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 18138);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 18139);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'i', s @ ..] => {
											super::push(out, 35615);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 35616);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 35617);
											s.is_empty()
										}
										['m', 'l', 'u', s @ ..] => {
											super::push(out, 35618);
											s.is_empty()
										}
										['n', 'o', s @ ..] => {
											super::push(out, 35619);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'g', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 18140);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 18141);
							match s {
								[] => true,
								['s', 'u', 'n', 'l', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 28163);
									s.is_empty()
								}
								['u', 't', 'p', 'y', 'l', 'a', 'k', 'u', 'e', '\u{1}', s @ ..] => {
									super::push(out, 28164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 18142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8236);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 18143);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8237);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 18144);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 28165);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 35620);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 28166);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 35621);
											s.is_empty()
										}
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
					super::push(out, 8238);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18145);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 18146);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 28167);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 28168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 18147);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 18148);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 18149);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 35622);
											match s {
												[] => true,
												['e', 'f', '\u{1}', s @ ..] => {
													super::push(out, 40343);
													s.is_empty()
												}
												_ => false,
											}
										}
										['k', s @ ..] => {
											super::push(out, 35623);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 35624);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 40344);
													match s {
														[] => true,
														['o', 's', '\u{1}', s @ ..] => super::t9::n45784(s, out),
														_ => false,
													}
												}
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
							super::push(out, 18150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8239);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 18151);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 18152);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 18153);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8240);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18154);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 18155);
							s.is_empty()
						}
						['u', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 18156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8241);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8242);
					match s {
						[] => true,
						['i', 'h', 'o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 18157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8243);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 18158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8244);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8245);
					match s {
						[] => true,
						['r', s @ ..] => super::t9::n18658(s, out),
						['ä', s @ ..] => {
							super::push(out, 18160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => super::t9::n8343(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t9::n2241(s, out),
		['d', s @ ..] => super::t9::n2242(s, out),
		['e', s @ ..] => super::t9::n2243(s, out),
		['f', s @ ..] => super::t9::n2244(s, out),
		['g', s @ ..] => super::t9::n2245(s, out),
		['h', s @ ..] => super::t9::n2246(s, out),
		['i', s @ ..] => super::t9::n2247(s, out),
		['j', s @ ..] => super::t9::n2248(s, out),
		['k', s @ ..] => super::t9::n2249(s, out),
		['l', s @ ..] => super::t9::n2250(s, out),
		['m', s @ ..] => super::t9::n2251(s, out),
		['n', s @ ..] => super::t9::n2252(s, out),
		['o', s @ ..] => super::t9::n2253(s, out),
		['p', s @ ..] => super::t9::n2254(s, out),
		['r', s @ ..] => super::t9::n2255(s, out),
		['s', s @ ..] => super::t9::n2256(s, out),
		['t', s @ ..] => super::t9::n2257(s, out),
		['u', s @ ..] => super::t9::n2258(s, out),
		['v', s @ ..] => super::t9::n2259(s, out),
		['w', s @ ..] => super::t9::n2260(s, out),
		['x', s @ ..] => super::t9::n2261(s, out),
		['y', s @ ..] => super::t9::n2262(s, out),
		['z', s @ ..] => super::t9::n2263(s, out),
		['ß', s @ ..] => super::t9::n2264(s, out),
		_ => false,
	}
}

/// Words ending -ln.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8407);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8408);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 18502);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 18503);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 28448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 8409);
					match s {
						[] => true,
						['f', 'o', s @ ..] => {
							super::push(out, 18504);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 28449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 18505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8410);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8411);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 28450);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 28451);
									match s {
										[] => true,
										['l', s @ ..] => {
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 40419);
													s.is_empty()
												}
												['a', s @ ..] => {
													super::push(out, 40420);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 8412);
					match s {
						[] => true,
						['c', 'i', 'w', 't', s @ ..] => {
							super::push(out, 18506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8413);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8414);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8415);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 18507);
							match s {
								[] => true,
								['i', '-', s @ ..] => {
									super::push(out, 28452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8416);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 18508);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 18509);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 28453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8417);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 18510);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 28454);
									s.is_empty()
								}
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
			super::push(out, 2262);
			match s {
				[] => true,
				['e', 'k', s @ ..] => {
					super::push(out, 8418);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nn.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2263);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8419);
					s.is_empty()
				}
				['m', s @ ..] => super::t9::n8518(s, out),
				['p', s @ ..] => {
					super::push(out, 8421);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 18525);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 28480);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 28481);
									match s {
										[] => true,
										['g', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 40444);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 40445);
													match s {
														[] => true,
														['e', s @ ..] => super::t9::n45842(s, out),
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
									super::push(out, 28482);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 28483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8422);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 18526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 8424);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 18528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2264);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8425);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8426);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8427);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 8428);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8429);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t9::n2269(s, out),
		_ => false,
	}
}

/// Words ending -on.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2266);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8437);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 2267);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 8439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2268);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8440);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8441);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8442);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 18544);
							s.is_empty()
						}
						['o', 'b', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28506);
									match s {
										[] => true,
										['d', 'a', 'l', 'o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 35846);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 28507);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 28508);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 28509);
									match s {
										[] => true,
										['e', 'd', s @ ..] => {
											super::push(out, 35847);
											match s {
												[] => true,
												['a', 'l', 'o', 'k', 'o', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 40460);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 28510);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 35848);
											s.is_empty()
										}
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
					super::push(out, 8443);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 18545);
							match s {
								[] => true,
								['c', 'r', s @ ..] => {
									super::push(out, 28511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 18546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2269);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8444);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18547);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 18548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8445);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 18549);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 18550);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28512);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 35849);
											s.is_empty()
										}
										['m', 'e', '\u{1}', s @ ..] => {
											super::push(out, 35850);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2270);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8446);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8447);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8448);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 18552);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 18553);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 18554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8449);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 18555);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 18556);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28513);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 18557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8450);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8451);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 18558);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 18559);
							match s {
								[] => true,
								['i', 'm', 's', '\u{1}', s @ ..] => {
									super::push(out, 28514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 18560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8452);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 18561);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 28515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 18562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2271);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8454);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8455);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8456);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8457);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 18563);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 18564);
							match s {
								[] => true,
								['p', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 28516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 18565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2272);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8459);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8460);
					match s {
						[] => true,
						['l', 'e', 't', 'l', s @ ..] => {
							super::push(out, 18566);
							match s {
								[] => true,
								['i', 'b', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 28517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 8461);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 18567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8462);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 18568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8463);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => super::t9::n2277(s, out),
		['h', s @ ..] => super::t9::n2278(s, out),
		['i', s @ ..] => super::t9::n2279(s, out),
		['j', s @ ..] => {
			super::push(out, 2276);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8495);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 18657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8496);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t9::n2281(s, out),
		['l', s @ ..] => super::t9::n2282(s, out),
		['m', s @ ..] => super::t9::n2283(s, out),
		['n', s @ ..] => super::t9::n2284(s, out),
		['o', s @ ..] => {
			super::push(out, 2281);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8530);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8531);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8532);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8533);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8534);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8535);
					match s {
						[] => true,
						['r', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 18716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8536);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 18717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t9::n2286(s, out),
		['r', s @ ..] => super::t9::n2287(s, out),
		['s', s @ ..] => super::t9::n2288(s, out),
		['t', s @ ..] => super::t9::n2289(s, out),
		['u', s @ ..] => {
			super::push(out, 2286);
			s.is_empty()
		}
		['v', s @ ..] => super::t9::n2291(s, out),
		['w', s @ ..] => super::t9::n2292(s, out),
		['x', s @ ..] => super::t9::n2293(s, out),
		['y', s @ ..] => super::t9::n2294(s, out),
		['z', s @ ..] => super::t9::n2295(s, out),
		['ä', s @ ..] => {
			super::push(out, 2292);
			s.is_empty()
		}
		['ç', s @ ..] => super::t9::n2297(s, out),
		_ => false,
	}
}

/// Words ending -rn.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2294);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8607);
					match s {
						[] => true,
						['m', 'u', s @ ..] => {
							super::push(out, 18845);
							match s {
								[] => true,
								['a', 's', s @ ..] => {
									super::push(out, 28780);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 35996);
											s.is_empty()
										}
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
					super::push(out, 8608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18846);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 18847);
							match s {
								[] => true,
								['e', 'i', s @ ..] => {
									super::push(out, 28781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8609);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18848);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 18849);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 8610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8611);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 8612);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8613);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8614);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 18850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 8615);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8616);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 18851);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 28782);
									match s {
										[] => true,
										['h', 'c', 'ö', 'm', '\u{1}', s @ ..] => {
											super::push(out, 35997);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 18852);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8617);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8618);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8619);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18853);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 18854);
							s.is_empty()
						}
						['d', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 18855);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 18856);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 18857);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 18858);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'b', '\u{1}', s @ ..] => super::t9::n30042(s, out),
								['t', 'n', 'a', '\u{1}', s @ ..] => super::t9::n30043(s, out),
								_ => false,
							}
						}
						['k', 'i', 'f', 'a', 'r', 'g', '\u{1}', s @ ..] => super::t9::n19389(s, out),
						['l', s @ ..] => super::t9::n19390(s, out),
						['m', s @ ..] => super::t9::n19391(s, out),
						['n', s @ ..] => super::t9::n19392(s, out),
						['r', s @ ..] => super::t9::n19393(s, out),
						['s', s @ ..] => super::t9::n19394(s, out),
						['t', s @ ..] => super::t9::n19395(s, out),
						['ß', 'u', 'n', s @ ..] => super::t9::n19396(s, out),
						['ö', s @ ..] => super::t9::n19397(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t9::n8718(s, out),
				['m', s @ ..] => super::t9::n8719(s, out),
				['n', s @ ..] => super::t9::n8720(s, out),
				['p', s @ ..] => super::t9::n8721(s, out),
				['s', s @ ..] => super::t9::n8722(s, out),
				['t', s @ ..] => super::t9::n8723(s, out),
				['u', s @ ..] => super::t9::n8724(s, out),
				['z', s @ ..] => super::t9::n8725(s, out),
				['ß', s @ ..] => super::t9::n8726(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t9::n2300(s, out),
		['o', s @ ..] => super::t9::n2301(s, out),
		['r', s @ ..] => super::t9::n2302(s, out),
		['u', s @ ..] => super::t9::n2303(s, out),
		['ö', s @ ..] => super::t9::n2304(s, out),
		_ => false,
	}
}

/// Words ending -sn.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['e', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 2301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -un.
pub(crate) fn n311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 311);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2302);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2303);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8650);
					match s {
						[] => true,
						['c', 'e', 'r', 'p', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 18917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8651);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 18918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8652);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 18919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8653);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18920);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28874);
									s.is_empty()
								}
								['t', 's', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 28875);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 8654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18921);
							s.is_empty()
						}
						['n', 'e', 't', 't', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 18922);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 18923);
							match s {
								[] => true,
								['f', 'o', 'h', 'd', 'e', 'i', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 28876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 18924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2304);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8655);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2305);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2306);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2307);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 2308);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2309);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8660);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8661);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8662);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8663);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8664);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8665);
					s.is_empty()
				}
				['ō', s @ ..] => {
					super::push(out, 8666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2310);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 2311);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2312);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2313);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2314);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2315);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8669);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 18925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2316);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2317);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 8671);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8672);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 18926);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 28877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wn.
pub(crate) fn n312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 312);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2318);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8673);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 28878);
									s.is_empty()
								}
								['u', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 28879);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'o', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 18927);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8674);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8675);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 18928);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 18929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yn.
pub(crate) fn n313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 313);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2319);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2320);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 8677);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2321);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 2322);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 2323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zn.
pub(crate) fn n314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 314);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -án.
pub(crate) fn n315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 315);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -än.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2326);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2327);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8678);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8679);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2328);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8680);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8681);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 8682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2329);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8683);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 18930);
							s.is_empty()
						}
						['g', 'i', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 18931);
							s.is_empty()
						}
						['t', 's', 'i', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 18932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ën.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ón.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 318);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2331);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 2332);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2333);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ön.
pub(crate) fn n319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 319);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2335);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2336);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 18933);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 18934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ün.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 2337);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 18935);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 18936);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 18937);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28880);
									s.is_empty()
								}
								['i', 's', '\u{1}', s @ ..] => {
									super::push(out, 28881);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 28882);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 28883);
									match s {
										[] => true,
										['i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 36051);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 18938);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 36052);
											s.is_empty()
										}
										['t', 'n', 'i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 36053);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 18939);
							match s {
								[] => true,
								['s', 'a', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 28884);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 18940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -erado.
pub(crate) fn n19488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18951);
	s.is_empty()
}

/// Words ending -orado.
pub(crate) fn n19489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18952);
	s.is_empty()
}

/// Words ending -prado.
pub(crate) fn n19490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18953);
	s.is_empty()
}

/// Words ending -sado.
pub(crate) fn n8815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8715);
	s.is_empty()
}

/// Words ending -tado.
pub(crate) fn n8816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8716);
	s.is_empty()
}

/// The word cruzado.
pub(crate) fn n8817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8717);
	s.is_empty()
}

/// Words ending -edo.
pub(crate) fn n2366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2362);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8718);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 8719);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8720);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 8721);
			match s {
				[] => true,
				['o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 18954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8722);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 18955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ido.
pub(crate) fn n2367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2363);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8724);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 8725);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 8726);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 8727);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 8728);
			match s {
				[] => true,
				['i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 18956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8729);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ldo.
pub(crate) fn n2368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2364);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8730);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8731);
			match s {
				[] => true,
				['d', 'i', 'm', 'm', 'u', 'g', '\u{1}', s @ ..] => {
					super::push(out, 18957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8732);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 18958);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ndo.
pub(crate) fn n2369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2365);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8733);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 18959);
					match s {
						[] => true,
						['m', 'o', 'k', 'l', s @ ..] => {
							super::push(out, 28885);
							match s {
								[] => true,
								['l', 'a', s @ ..] => {
									super::push(out, 36054);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 18960);
					match s {
						[] => true,
						['s', 'i', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 28886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 8734);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 18961);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 18962);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 18963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8735);
			match s {
				[] => true,
				['h', 's', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 18964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 8736);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 18965);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 18966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -odo.
pub(crate) fn n2370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2366);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 8737);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 8738);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rdo.
pub(crate) fn n2371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2367);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18968);
					s.is_empty()
				}
				['a', 'r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 18969);
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 18970);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udo.
pub(crate) fn n2372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2368);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 8739);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 8740);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 8741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 8742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eo.
pub(crate) fn n326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 326);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2369);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 2370);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2371);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8744);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8745);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 18972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2372);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 2373);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2374);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8746);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8747);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fo.
pub(crate) fn n327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 327);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2375);
			s.is_empty()
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 2376);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2377);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2378);
			match s {
				[] => true,
				['i', 'w', s @ ..] => {
					super::push(out, 8748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2379);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -go.
pub(crate) fn n328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 328);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2381);
			match s {
				[] => true,
				['d', 'n', 'a', '-', 'p', 'o', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 8749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 2382);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8750);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8751);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 18973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2383);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8752);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 8753);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 8754);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18974);
							s.is_empty()
						}
						['e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 18975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2384);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8755);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 28887);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 28888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8756);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18976);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 18977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8757);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8758);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 18978);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 18979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2385);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2386);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2387);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8760);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 18980);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 18981);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 18982);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 18983);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 28889);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 28890);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 18984);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 18985);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 18986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8761);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8762);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 18989);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 18990);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 18991);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 18992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2388);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 8764);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8765);
					match s {
						[] => true,
						['n', 'o', s @ ..] => {
							super::push(out, 18993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2389);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8767);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 18994);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 18995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8768);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 18996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2390);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 8769);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 2391);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ho.
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 329);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2392);
			match s {
				[] => true,
				['v', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 8770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2393);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8771);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 18998);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 18999);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 19000);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 19001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8772);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8773);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 19002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8774);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 8775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2394);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2395);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2396);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8776);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8777);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -io.
pub(crate) fn n330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 330);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2397);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8778);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8779);
					match s {
						[] => true,
						['b', 'i', '\u{1}', s @ ..] => {
							super::push(out, 19003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2398);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8780);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 8781);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 28893);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 28894);
									s.is_empty()
								}
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
			super::push(out, 2399);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8782);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 28895);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 28896);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 28897);
									match s {
										[] => true,
										['a', 't', s @ ..] => {
											super::push(out, 36055);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 28898);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['h', 'c', 'ü', s @ ..] => {
													super::push(out, 40557);
													s.is_empty()
												}
												['r', 'h', 'ö', s @ ..] => {
													super::push(out, 40558);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									super::push(out, 28899);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 28900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8783);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 19004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8784);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 19005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2400);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8786);
					match s {
						[] => true,
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 19006);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8787);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 19007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8788);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2401);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8790);
					match s {
						[] => true,
						['c', 'i', 'd', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 19008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2402);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8791);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19009);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 19010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8792);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 19011);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28901);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 19012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2403);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8794);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 19013);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 19014);
							match s {
								[] => true,
								['e', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 28902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 8795);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8797);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 19016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8798);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8799);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 19017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2405);
			match s {
				[] => true,
				['a', 'r', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8800);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2406);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8802);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 19018);
							match s {
								[] => true,
								['l', 'e', 'f', 'o', s @ ..] => {
									super::push(out, 28903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 19019);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 19020);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 28904);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 28905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8803);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8804);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 19021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8805);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8806);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -jo.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 2407);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['a', s @ ..] => {
			super::push(out, 2408);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8808);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8809);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8810);
					match s {
						[] => true,
						['a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 19022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2409);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2410);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8811);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2411);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 8813);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 8814);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ko.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2412);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8815);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8816);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 8817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2413);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2414);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2415);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2416);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2417);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8820);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8821);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 19025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8822);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8823);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 19027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8824);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 36056);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 36057);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 36058);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 28906);
									match s {
										[] => true,
										['e', 'i', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 36059);
											s.is_empty()
										}
										['l', 'a', 'f', s @ ..] => {
											match s {
												[] => true,
												['n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 40559);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 40560);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 28907);
									match s {
										[] => true,
										['e', 'p', s @ ..] => {
											super::push(out, 36060);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 28908);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['b', 'e', 'r', 'k', '\u{1}', s @ ..] => {
											super::push(out, 36061);
											s.is_empty()
										}
										['g', s @ ..] => {
											super::push(out, 36062);
											match s {
												[] => true,
												['n', 'u', s @ ..] => {
													match s {
														[] => true,
														['d', 'r', 'h', 'ä', 'f', 'e', 'g', '\u{1}', s @ ..] => super::t9::n45906(s, out),
														['k', s @ ..] => super::t9::n45907(s, out),
														['r', s @ ..] => super::t9::n45908(s, out),
														['t', s @ ..] => super::t9::n45909(s, out),
														['z', 't', 'e', 'l', 'r', 'e', 'v', '\u{1}', s @ ..] => super::t9::n45910(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 36063);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 40561);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 40562);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 36064);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 36065);
											match s {
												[] => true,
												['u', 'm', 'r', 'a', '\u{1}', s @ ..] => {
													super::push(out, 40563);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'i', s @ ..] => {
									super::push(out, 28909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8825);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 19028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2418);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8826);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8827);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 19029);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 19030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2419);
			match s {
				[] => true,
				['e', 't', '\u{1}', s @ ..] => {
					super::push(out, 8828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2420);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8829);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8830);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 19031);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 19032);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2421);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8831);
					s.is_empty()
				}
				['h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 8832);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8833);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8834);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8835);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19033);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 19034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2422);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2423);
			match s {
				[] => true,
				['a', s @ ..] => super::t9::n8938(s, out),
				['e', s @ ..] => super::t9::n8939(s, out),
				['i', s @ ..] => super::t9::n8940(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t9::n2428(s, out),
		['ö', s @ ..] => super::t9::n2429(s, out),
		_ => false,
	}
}

/// Words ending -lo.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2426);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8840);
					match s {
						[] => true,
						['m', 'e', 'c', 'i', 'v', 'a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 19037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8841);
					match s {
						[] => true,
						['ó', 'z', '\u{1}', s @ ..] => {
							super::push(out, 19038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8842);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8843);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8844);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 19039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2427);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8846);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8847);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8848);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'g', 'a', '\u{1}', s @ ..] => {
			super::push(out, 2428);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2429);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8850);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 19040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 8851);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2430);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8854);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 19042);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2431);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8855);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 19043);
							match s {
								[] => true,
								['a', 'p', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 28910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 19044);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 19045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8856);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 19046);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 19047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28911);
									s.is_empty()
								}
								['n', 'o', s @ ..] => {
									match s {
										[] => true,
										['l', 'o', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 36066);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 36067);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 28912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 19048);
							match s {
								[] => true,
								['i', 'c', 'i', 'l', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 28913);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 19049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8857);
					match s {
						[] => true,
						['d', 'u', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 19050);
							s.is_empty()
						}
						['r', 'a', 'g', s @ ..] => {
							super::push(out, 19051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8858);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 19052);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 19053);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 28914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 19054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2432);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8859);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 8860);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8861);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 8862);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8863);
					match s {
						[] => true,
						['e', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 19055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8864);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 19056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8865);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8866);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2433);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mo.
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2434);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2435);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 8870);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8871);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2436);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 8873);
					match s {
						[] => true,
						['s', 'e', '\u{1}', s @ ..] => {
							super::push(out, 19057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8874);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8875);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8876);
					match s {
						[] => true,
						['q', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 19058);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2437);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 8877);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8878);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 19059);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2438);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8879);
					match s {
						[] => true,
						['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 19060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2439);
			s.is_empty()
		}
		_ => false,
	}
}

