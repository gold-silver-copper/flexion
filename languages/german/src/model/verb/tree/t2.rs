use flexion::Path;

/// Words ending -hsen.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1875);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 3903);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 3904);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5812);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 5813);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 5814);
									match s {
										[] => true,
										['c', s @ ..] => {
											match s {
												[] => true,
												['a', 'n', '\u{1}', s @ ..] => {
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
								['n', s @ ..] => {
									super::push(out, 5815);
									match s {
										[] => true,
										['a', s @ ..] => {
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
										['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5816);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 7086);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
											super::push(out, 7087);
											match s {
												[] => true,
												['p', 'm', 'e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 5817);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 5818);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 7088);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 7089);
											match s {
												[] => true,
												['e', 'f', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 5819);
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
				['i', s @ ..] => {
					super::push(out, 1876);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 5820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1878);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 3905);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -isen.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 678);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1879);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1880);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1881);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 3907);
							match s {
								[] => true,
								['f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1883);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 3908);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5823);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5824);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 5825);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5826);
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
				['r', s @ ..] => {
					super::push(out, 1884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3909);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3910);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5827);
									match s {
										[] => true,
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
							super::push(out, 3911);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 3912);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3913);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7090);
											match s {
												[] => true,
												['n', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => super::t4::n11672(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['k', s @ ..] => super::t4::n5042(s, out),
						['m', s @ ..] => super::t4::n5043(s, out),
						['n', s @ ..] => super::t4::n5044(s, out),
						['p', s @ ..] => super::t4::n5045(s, out),
						['r', s @ ..] => super::t4::n5046(s, out),
						['s', s @ ..] => super::t4::n5047(s, out),
						['t', s @ ..] => super::t4::n5048(s, out),
						['u', s @ ..] => super::t4::n5049(s, out),
						_ => false,
					}
				}
				['s', s @ ..] => super::t4::n2173(s, out),
				['t', s @ ..] => super::t4::n2174(s, out),
				['w', s @ ..] => super::t4::n2175(s, out),
				['z', '\u{1}', s @ ..] => {
					super::push(out, 1888);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ksen.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 680);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['a', 'n', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				['i', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3935);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1890);
					match s {
						[] => true,
						['i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['o', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1891);
					s.is_empty()
				}
				['u', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3937);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -msen.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['a', 'n', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 683);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['r', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3939);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3940);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['e', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3941);
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
		['u', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 3942);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nsen.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['r', 'f', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 3943);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 3944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 688);
			s.is_empty()
		}
		['i', 'z', s @ ..] => {
			super::push(out, 689);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -osen.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 690);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 691);
			match s {
				[] => true,
				['n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 693);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1894);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1896);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3945);
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
						['h', s @ ..] => {
							super::push(out, 3946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1897);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -psen.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 5850);
									s.is_empty()
								}
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
			super::push(out, 695);
			match s {
				[] => true,
				['n', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3948);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3949);
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
		['m', s @ ..] => {
			super::push(out, 696);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 3950);
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

/// Words ending -ssen.
pub(crate) fn n192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 192);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1898);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3951);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3952);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3953);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3954);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3955);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 5851);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3956);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5852);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 5853);
									match s {
										[] => true,
										['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 5854);
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
							super::push(out, 3957);
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
							super::push(out, 3958);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 5855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 3959);
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
					super::push(out, 1899);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t4::n2201(s, out),
				['m', s @ ..] => {
					super::push(out, 1901);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 3974);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 3975);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 3976);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5870);
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
								['i', 'e', 'n', 'i', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3977);
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
					super::push(out, 1903);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t4::n8669(s, out),
								['r', s @ ..] => super::t4::n8670(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n738(s, out),
		['i', s @ ..] => super::t4::n739(s, out),
		['o', s @ ..] => super::t4::n740(s, out),
		['u', s @ ..] => super::t4::n741(s, out),
		['y', s @ ..] => super::t4::n742(s, out),
		['ä', s @ ..] => super::t4::n743(s, out),
		['ü', s @ ..] => super::t4::n744(s, out),
		_ => false,
	}
}

/// Words ending -usen.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4018);
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
				['m', s @ ..] => {
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
				['p', s @ ..] => {
					super::push(out, 1932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4019);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4020);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1933);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 5900);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7133);
											s.is_empty()
										}
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
					super::push(out, 1934);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4021);
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
		['e', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['l', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4022);
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
		['m', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 709);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äsen.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1936);
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

/// Words ending -ösen.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 710);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1937);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1938);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1939);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1940);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1941);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4023);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4024);
							match s {
								[] => true,
								['a', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üsen.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eten.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 721);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1952);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1953);
					match s {
						[] => true,
						['n', 'u', 's', 'e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1954);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1955);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1956);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 4035);
							s.is_empty()
						}
						['o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1957);
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
		['e', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['w', s @ ..] => {
					super::push(out, 1958);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5902);
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 5903);
									s.is_empty()
								}
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
			super::push(out, 723);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4036);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4037);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4038);
							match s {
								[] => true,
								['i', 'e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4039);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4040);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 5904);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 5905);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
						['t', 'n', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1960);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4041);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4042);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7136);
											match s {
												[] => true,
												['e', 't', 'n', 'u', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
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
				['n', s @ ..] => {
					super::push(out, 1961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 724);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 725);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 726);
			match s {
				[] => true,
				['t', s @ ..] => super::t4::n2271(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -aften.
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 727);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4058);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4059);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7139);
									match s {
										[] => true,
										['n', 'u', 'k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 7790);
													match s {
														[] => true,
														['u', 'a', '\u{1}', s @ ..] => super::t4::n14314(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 'r', 'i', 'w', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7630);
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 7631);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 7791);
													s.is_empty()
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 7632);
											match s {
												[] => true,
												['b', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 7633);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n14315(s, out),
														['t', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => super::t4::n14316(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4060);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 5916);
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
		['r', 'c', s @ ..] => {
			super::push(out, 1962);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4061);
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

/// Words ending -heften.
pub(crate) fn n2275(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4062);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5918);
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
		_ => false,
	}
}

/// Words ending -iften.
pub(crate) fn n770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 729);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1963);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4063);
					s.is_empty()
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
		['h', s @ ..] => {
			super::push(out, 1964);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1965);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4064);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5919);
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
				['h', s @ ..] => {
					super::push(out, 4065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1966);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5920);
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

/// Words ending -uften.
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 730);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4066);
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
		['h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üften.
pub(crate) fn n773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 732);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4068);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4069);
					match s {
						[] => true,
						['e', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 5921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'o', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 4070);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 4071);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hten.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['a', 'r', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1968);
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
		['c', s @ ..] => {
			super::push(out, 733);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1969);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4072);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4073);
							match s {
								[] => true,
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 7140);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 7141);
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
							super::push(out, 4074);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 5922);
									match s {
										[] => true,
										['c', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7634);
													s.is_empty()
												}
												['r', s @ ..] => super::t4::n13522(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t4::n5266(s, out),
						['p', s @ ..] => super::t4::n5267(s, out),
						['r', s @ ..] => super::t4::n5268(s, out),
						['s', 's', 'i', 'm', '\u{1}', s @ ..] => super::t4::n5269(s, out),
						['t', 'u', 'g', s @ ..] => super::t4::n5270(s, out),
						['ß', 'i', 'm', '\u{1}', s @ ..] => super::t4::n5271(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t4::n2286(s, out),
				['i', s @ ..] => super::t4::n2287(s, out),
				['r', s @ ..] => super::t4::n2288(s, out),
				['u', s @ ..] => super::t4::n2289(s, out),
				['y', 'e', 'b', '\u{1}', s @ ..] => super::t4::n2290(s, out),
				['ä', s @ ..] => super::t4::n2291(s, out),
				['ü', s @ ..] => super::t4::n2292(s, out),
				_ => false,
			}
		}
		['g', 'i', 'f', '\u{1}', s @ ..] => super::t4::n776(s, out),
		_ => false,
	}
}

/// Words ending -iten.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 735);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 736);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1977);
					match s {
						[] => true,
						['r', 'a', s @ ..] => super::t4::n5293(s, out),
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1978);
					s.is_empty()
				}
				['l', s @ ..] => super::t4::n2295(s, out),
				['r', s @ ..] => super::t4::n2296(s, out),
				['w', s @ ..] => {
					super::push(out, 1981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4115);
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

/// Words ending -kten.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 737);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'a', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1982);
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

/// Words ending -lten.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 738);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4116);
							match s {
								[] => true,
								['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4117);
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
				['h', s @ ..] => super::t4::n2302(s, out),
				['k', s @ ..] => {
					super::push(out, 1985);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4130);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1986);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4131);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5995);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 5996);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 5997);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 5998);
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
				['r', s @ ..] => {
					super::push(out, 1987);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1988);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4133);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 7207);
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
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1989);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4134);
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4135);
							s.is_empty()
						}
						['m', 'u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => super::t4::n5346(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t4::n782(s, out),
		['i', 'g', '\u{1}', s @ ..] => super::t4::n783(s, out),
		['ä', s @ ..] => super::t4::n784(s, out),
		_ => false,
	}
}

/// Words ending -mten.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['a', 'e', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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

/// Words ending -nten.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 742);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 743);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 744);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4139);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4140);
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
		_ => false,
	}
}

/// Words ending -oten.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['b', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 745);
			s.is_empty()
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1993);
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1995);
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
		['m', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 747);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4141);
							s.is_empty()
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
		['o', s @ ..] => {
			super::push(out, 749);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4142);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4143);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1996);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pten.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 750);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 751);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rten.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1998);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1999);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2000);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2001);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2002);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4145);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4146);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6002);
									s.is_empty()
								}
								['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4147);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4148);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4149);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4150);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4151);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4152);
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
			super::push(out, 753);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4153);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4154);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6003);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4155);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											match s {
												[] => true,
												['b', 'ü', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['t', 'n', 'u', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4156);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6004);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4157);
							match s {
								[] => true,
								['e', 'v', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7208);
											match s {
												[] => true,
												['e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 4158);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6005);
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
		['h', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 756);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2006);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 2007);
					s.is_empty()
				}
				['r', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 2008);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2009);
					match s {
						[] => true,
						['t', 'n', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6006);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
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
		['u', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 2010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 758);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4159);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4160);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4161);
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
							super::push(out, 4162);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6008);
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
		['ü', s @ ..] => {
			super::push(out, 759);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sten.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2011);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2012);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4163);
							s.is_empty()
						}
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4164);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6009);
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
					super::push(out, 2013);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4165);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4166);
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
				['t', s @ ..] => {
					super::push(out, 2014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4167);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4168);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4169);
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
			super::push(out, 761);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4170);
							match s {
								[] => true,
								['e', 'r', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 762);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2015);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6010);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 6011);
									match s {
										[] => true,
										['c', 'a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6012);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 7209);
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
						['r', s @ ..] => {
							super::push(out, 4171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 2016);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2017);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4172);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4173);
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
				['m', s @ ..] => {
					super::push(out, 2018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4174);
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
				['n', s @ ..] => {
					super::push(out, 2019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4175);
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
				['r', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4176);
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
		['n', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['u', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4177);
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
		['o', s @ ..] => {
			super::push(out, 764);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2020);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2021);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4178);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4179);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4180);
							match s {
								[] => true,
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => super::t4::n5419(s, out),
						['p', s @ ..] => super::t4::n5420(s, out),
						['r', s @ ..] => super::t4::n5421(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t4::n811(s, out),
		['u', s @ ..] => super::t4::n812(s, out),
		['ö', s @ ..] => super::t4::n813(s, out),
		['ü', s @ ..] => super::t4::n814(s, out),
		_ => false,
	}
}

/// Words ending -tten.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2032);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2033);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6025);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6026);
									match s {
										[] => true,
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
				['m', s @ ..] => {
					super::push(out, 2034);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2035);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 6027);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6028);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6029);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['k', s @ ..] => {
													super::push(out, 7664);
													match s {
														[] => true,
														['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => super::t4::n14205(s, out),
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
									super::push(out, 6030);
									s.is_empty()
								}
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
			super::push(out, 770);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4200);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4201);
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
				['f', s @ ..] => {
					super::push(out, 2037);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4202);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4203);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4204);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4205);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4206);
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
				['j', s @ ..] => {
					super::push(out, 2038);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2039);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4207);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4208);
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4209);
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
				['r', s @ ..] => {
					super::push(out, 2040);
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
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4210);
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
				['s', s @ ..] => {
					super::push(out, 2041);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2042);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4211);
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
		['i', s @ ..] => {
			super::push(out, 771);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2043);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4212);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4213);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4214);
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
							super::push(out, 4215);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t4::n818(s, out),
		['ä', s @ ..] => {
			super::push(out, 773);
			s.is_empty()
		}
		['ü', s @ ..] => super::t4::n820(s, out),
		_ => false,
	}
}

/// Words ending -uten.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4224);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4225);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4226);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4227);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4228);
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
		['e', s @ ..] => {
			super::push(out, 776);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4229);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4230);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4231);
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
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4232);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4233);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4234);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4235);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6036);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 6037);
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
						['s', s @ ..] => {
							super::push(out, 4236);
							match s {
								[] => true,
								['s', 'i', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6038);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ß', s @ ..] => {
							super::push(out, 4237);
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
				['r', s @ ..] => {
					super::push(out, 2049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4238);
							s.is_empty()
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4239);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4240);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4241);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4242);
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
				['f', s @ ..] => {
					super::push(out, 2050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4243);
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
		['m', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2051);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2052);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 779);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 781);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2055);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4244);
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
				['l', s @ ..] => {
					super::push(out, 2056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4245);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6040);
									s.is_empty()
								}
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
		_ => false,
	}
}

/// Words ending -xten.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['e', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2057);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äten.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 783);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öten.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2058);
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
			super::push(out, 784);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2059);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2060);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2061);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2062);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üten.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2063);
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
		['r', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4247);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4248);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4249);
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
		['t', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 789);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rbauen.
pub(crate) fn n2419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2072);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4254);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6041);
					match s {
						[] => true,
						['n', 'a', 'n', 'i', 'e', 's', 'u', 'a', '\u{1}', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 4255);
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

/// Words ending -sbauen.
pub(crate) fn n2420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2073);
	match s {
		[] => true,
		['u', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ubauen.
pub(crate) fn n2421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2074);
	match s {
		[] => true,
		['z', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word außbauen.
pub(crate) fn n2422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2075);
	s.is_empty()
}

/// Words ending -dauen.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 791);
	s.is_empty()
}

/// Words ending -hauen.
pub(crate) fn n840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 792);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2076);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2077);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4256);
					match s {
						[] => true,
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
			super::push(out, 2078);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6042);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6043);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6044);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 6045);
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 6046);
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
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6047);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['a', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'l', 'n', 'h', 'ä', '\u{1}', s @ ..] => {
											super::push(out, 7665);
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
						['i', 'e', 'b', 'r', 'o', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6048);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6049);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7219);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7220);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['h', '\u{1}', s @ ..] => {
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
							super::push(out, 6050);
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
										['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 7666);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 7667);
											match s {
												[] => true,
												['n', 'u', s @ ..] => {
													match s {
														[] => true,
														['n', 'i', 'h', '\u{1}', s @ ..] => super::t4::n14324(s, out),
														['r', '\u{1}', s @ ..] => super::t4::n14325(s, out),
														_ => false,
													}
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
							super::push(out, 6051);
							match s {
								[] => true,
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
						['u', s @ ..] => {
							super::push(out, 6052);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 7221);
									match s {
										[] => true,
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
			super::push(out, 2079);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', 'u', 'a', 'r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 2080);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 2081);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4257);
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
			super::push(out, 2082);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 4258);
					match s {
						[] => true,
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
			super::push(out, 2083);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4259);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4260);
					match s {
						[] => true,
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4261);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 6053);
							match s {
								[] => true,
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
			super::push(out, 2084);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4262);
					match s {
						[] => true,
						['d', 'e', 'i', 'n', '\u{1}', s @ ..] => super::t4::n9050(s, out),
						['v', '\u{1}', s @ ..] => super::t4::n9051(s, out),
						['z', '\u{1}', s @ ..] => super::t4::n9052(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t4::n2432(s, out),
		['u', s @ ..] => super::t4::n2433(s, out),
		_ => false,
	}
}

/// Words ending -iauen.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 793);
	s.is_empty()
}

/// Words ending -kauen.
pub(crate) fn n842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 794);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2087);
			match s {
				[] => true,
				['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2088);
			match s {
				[] => true,
				['e', 'z', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 4265);
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

/// Words ending -lauen.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 795);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 2089);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word mauen.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 796);
	s.is_empty()
}

/// Words ending -rauen.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 797);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2090);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4266);
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
		['f', s @ ..] => {
			super::push(out, 2091);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2092);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4268);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4269);
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
					super::push(out, 4270);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2093);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4271);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4272);
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
				['m', s @ ..] => {
					super::push(out, 4273);
					match s {
						[] => true,
						['i', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6056);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4274);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7222);
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
				['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 4275);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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

/// Words ending -sauen.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 798);
	s.is_empty()
}

/// Words ending -tauen.
pub(crate) fn n847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 799);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2094);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2095);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2096);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4276);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4277);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4278);
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

/// Words ending -euen.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2097);
					s.is_empty()
				}
				['n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2099);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2100);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4281);
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2101);
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6057);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6058);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7223);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6059);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 7224);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6060);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -äuen.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2102);
					s.is_empty()
				}
				['r', 'e', 'd', 'e', 'i', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4282);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4283);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'd', '\u{1}', s @ ..] => {
			super::push(out, 804);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 805);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -walzen.
pub(crate) fn n2471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2112);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4297);
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

/// Words ending -elzen.
pub(crate) fn n873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 818);
	match s {
		[] => true,
		['m', 'h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4298);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4299);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4300);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7225);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 7226);
									match s {
										[] => true,
										['a', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 4301);
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
		_ => false,
	}
}

/// Words ending -ilzen.
pub(crate) fn n874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 819);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4302);
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

/// Words ending -olzen.
pub(crate) fn n875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 820);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 2113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -älzen.
pub(crate) fn n876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4303);
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

/// Words ending -nzen.
pub(crate) fn n238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 235);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2114);
					match s {
						[] => true,
						['c', 's', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2115);
					match s {
						[] => true,
						['f', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6066);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 6067);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6068);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6069);
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
				['r', s @ ..] => {
					super::push(out, 2116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4305);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4306);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4307);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6070);
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
				['w', s @ ..] => {
					super::push(out, 2118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2119);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2120);
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
				['r', s @ ..] => {
					super::push(out, 2121);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6071);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 6072);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6073);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7227);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 7228);
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
								['s', s @ ..] => {
									super::push(out, 6074);
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
						['t', s @ ..] => {
							super::push(out, 4308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2122);
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2123);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6076);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7229);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 825);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2124);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6077);
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
				['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 2125);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rzen.
pub(crate) fn n239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 236);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 826);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6078);
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
				['m', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6079);
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
						['s', s @ ..] => {
							super::push(out, 4310);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6080);
									s.is_empty()
								}
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
			super::push(out, 828);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['w', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4311);
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
		['ü', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2126);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4312);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4313);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4314);
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
					super::push(out, 2128);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6081);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7230);
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
									super::push(out, 6082);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 6083);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6084);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 7231);
											match s {
												[] => true,
												['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 7232);
											match s {
												[] => true,
												['e', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
									super::push(out, 6085);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 7668);
													match s {
														[] => true,
														['ü', '\u{1}', s @ ..] => super::t4::n14207(s, out),
														_ => false,
													}
												}
												['d', s @ ..] => {
													super::push(out, 7669);
													match s {
														[] => true,
														['e', 'i', 'n', '\u{1}', s @ ..] => super::t4::n14208(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2129);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4315);
							match s {
								[] => true,
								['c', 'a', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4316);
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

/// Words ending -tzen.
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 237);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4317);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6086);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6087);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 6088);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6089);
									match s {
										[] => true,
										['i', 'e', 'r', s @ ..] => {
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
								['r', s @ ..] => {
									super::push(out, 6090);
									match s {
										[] => true,
										['e', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6091);
									match s {
										[] => true,
										['u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4318);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6092);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6093);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6094);
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
				['w', s @ ..] => {
					super::push(out, 2132);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6095);
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 833);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4320);
							s.is_empty()
						}
						['r', 'e', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4321);
							s.is_empty()
						}
						['f', s @ ..] => super::t4::n5644(s, out),
						['r', s @ ..] => super::t4::n5645(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t4::n2503(s, out),
				['n', s @ ..] => super::t4::n2504(s, out),
				['p', s @ ..] => super::t4::n2505(s, out),
				['s', s @ ..] => super::t4::n2506(s, out),
				['w', s @ ..] => super::t4::n2507(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n889(s, out),
		['o', s @ ..] => super::t4::n890(s, out),
		['u', s @ ..] => super::t4::n891(s, out),
		['ä', s @ ..] => super::t4::n892(s, out),
		['ö', 'g', 'r', 'e', '\u{1}', s @ ..] => super::t4::n893(s, out),
		['ü', s @ ..] => super::t4::n894(s, out),
		_ => false,
	}
}

/// Words ending -uzen.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 238);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['n', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['r', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4385);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4386);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4387);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6127);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4388);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4389);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zzen.
pub(crate) fn n242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 239);
	match s {
		[] => true,
		['a', 'j', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 842);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ießen.
pub(crate) fn n901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 845);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 2172);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4390);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4391);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4392);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4393);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4394);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6128);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6129);
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
					super::push(out, 4395);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'ü', '\u{1}', s @ ..] => {
									super::push(out, 7251);
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
					super::push(out, 4396);
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
		['h', s @ ..] => {
			super::push(out, 2173);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6130);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6131);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6132);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 6133);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 6134);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6135);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7252);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 7253);
									match s {
										[] => true,
										['b', 'e', 'n', 'a', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7254);
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
							super::push(out, 6136);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7255);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'ü', '\u{1}', s @ ..] => {
											super::push(out, 7673);
											s.is_empty()
										}
										['d', 'e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 7674);
											s.is_empty()
										}
										['u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 7675);
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
									super::push(out, 7256);
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
							super::push(out, 6137);
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6138);
							match s {
								[] => true,
								['o', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6139);
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
		['l', s @ ..] => {
			super::push(out, 2174);
			match s {
				[] => true,
				['f', s @ ..] => super::t4::n5742(s, out),
				['h', s @ ..] => super::t4::n5743(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2175);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 4399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2177);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4400);
					s.is_empty()
				}
				['p', s @ ..] => super::t4::n5746(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ißen.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4402);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4403);
							match s {
								[] => true,
								['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4404);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4405);
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
						['t', s @ ..] => {
							super::push(out, 4406);
							match s {
								[] => true,
								['s', 'e', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4407);
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
					super::push(out, 2182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4408);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4409);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7269);
											s.is_empty()
										}
										['e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 7270);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 7271);
											match s {
												[] => true,
												['u', 'a', 'r', 'd', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 7272);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', 'm', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', 'e', 'r', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 7273);
											match s {
												[] => true,
												['e', 'v', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 7274);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 7275);
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
						['r', s @ ..] => {
							super::push(out, 4410);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4411);
							match s {
								[] => true,
								['u', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t4::n2556(s, out),
				['m', s @ ..] => super::t4::n2557(s, out),
				['r', s @ ..] => super::t4::n2558(s, out),
				['w', s @ ..] => super::t4::n2559(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stoßen.
pub(crate) fn n2561(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4428);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4429);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4430);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 4431);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4432);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4433);
			match s {
				[] => true,
				['u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4434);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6164);
					match s {
						[] => true,
						['m', 'm', 'a', 's', 'u', 'z', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6165);
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
			super::push(out, 4435);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6166);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7289);
							match s {
								[] => true,
								['e', 'i', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'a', 'n', 'i', 'e', 'f', 'u', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7290);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 7291);
							match s {
								[] => true,
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
					super::push(out, 6167);
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
			super::push(out, 4436);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4437);
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

/// Words ending -eyßen.
pub(crate) fn n908(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 2189);
			s.is_empty()
		}
		['h', '\u{1}', s @ ..] => {
			super::push(out, 2190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ößen.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['f', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -üßen.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2192);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2193);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2194);
					match s {
						[] => true,
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2195);
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
		['r', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2196);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2197);
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
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2198);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2199);
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

/// The word rumkrabbeln.
pub(crate) fn n13746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7682);
	s.is_empty()
}

/// The word herumkrabbeln.
pub(crate) fn n13747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7683);
	s.is_empty()
}

/// Words ending -irbeln.
pub(crate) fn n2586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2204);
	match s {
		[] => true,
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6173);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6174);
					match s {
						[] => true,
						['u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6175);
					match s {
						[] => true,
						['o', 'p', 'm', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
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
		_ => false,
	}
}

/// Words ending -urbeln.
pub(crate) fn n2587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2205);
	match s {
		[] => true,
		['k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6176);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6177);
					match s {
						[] => true,
						['e', 't', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ubeln.
pub(crate) fn n921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 861);
	match s {
		[] => true,
		['j', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4445);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['t', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 6178);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 6179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4446);
					match s {
						[] => true,
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -öbeln.
pub(crate) fn n923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 863);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 2206);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4447);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 6181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -deln.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 250);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2207);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4449);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6183);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6184);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6185);
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
						['h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 't', s @ ..] => {
					super::push(out, 2208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 4451);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4452);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6186);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6187);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 6188);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 6189);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6190);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6191);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 7684);
													match s {
														[] => true,
														['ü', '\u{1}', s @ ..] => super::t4::n14222(s, out),
														_ => false,
													}
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
								['s', s @ ..] => {
									super::push(out, 6192);
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
				['r', s @ ..] => {
					super::push(out, 2209);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 2210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['a', s @ ..] => super::t5::n2600(s, out),
				['e', s @ ..] => {
					super::push(out, 2212);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2213);
					match s {
						[] => true,
						['w', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6208);
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
				['o', s @ ..] => {
					super::push(out, 2214);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2215);
					match s {
						[] => true,
						['b', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4458);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6209);
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
				['ü', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t5::n930(s, out),
		['ä', s @ ..] => super::t5::n931(s, out),
		['ö', s @ ..] => super::t5::n932(s, out),
		['ü', 't', s @ ..] => super::t5::n933(s, out),
		_ => false,
	}
}

/// Words ending -feln.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
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
				['ö', s @ ..] => {
					super::push(out, 2225);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6212);
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
				['ü', 'n', s @ ..] => {
					super::push(out, 2226);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6213);
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
		['i', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['e', 'w', 'z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4470);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4471);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4472);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 6214);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4473);
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
		['r', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['ü', 'w', 'r', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['e', 'd', 'n', 'a', 'n', 'i', 'e', 'h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2228);
					s.is_empty()
				}
				['ä', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4475);
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
		_ => false,
	}
}

/// Words ending -geln.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 877);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2229);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4476);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4478);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4479);
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
				['r', s @ ..] => {
					match s {
						[] => true,
						['k', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4480);
							s.is_empty()
						}
						['p', 's', 'r', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 878);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4482);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7293);
											match s {
												[] => true,
												['e', 'd', s @ ..] => {
													match s {
														[] => true,
														['e', 'i', 'w', '\u{1}', s @ ..] => super::t5::n14227(s, out),
														['i', 'w', '\u{1}', s @ ..] => super::t5::n14228(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 7294);
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
							super::push(out, 4483);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6215);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6216);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7295);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 7296);
											match s {
												[] => true,
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
							super::push(out, 4484);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6217);
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
						['w', s @ ..] => {
							super::push(out, 4485);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6218);
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
				['l', 'f', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 879);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6219);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 7297);
											s.is_empty()
										}
										_ => false,
									}
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
				['i', s @ ..] => {
					super::push(out, 2233);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4486);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7298);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 4487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4488);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2234);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4489);
							s.is_empty()
						}
						['r', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6221);
									match s {
										[] => true,
										['o', 'v', '\u{1}', s @ ..] => {
											super::push(out, 7299);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', 'l', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 881);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 882);
			s.is_empty()
		}
		['u', s @ ..] => super::t5::n944(s, out),
		['ä', s @ ..] => {
			super::push(out, 884);
			s.is_empty()
		}
		['ö', s @ ..] => super::t5::n946(s, out),
		['ü', s @ ..] => super::t5::n947(s, out),
		_ => false,
	}
}

/// Words ending -heln.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6229);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6230);
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
				['e', s @ ..] => {
					super::push(out, 2242);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 6231);
									match s {
										[] => true,
										['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
											super::push(out, 7300);
											s.is_empty()
										}
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
					super::push(out, 2243);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 6232);
									match s {
										[] => true,
										['h', 'c', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['m', s @ ..] => {
													super::push(out, 7692);
													match s {
														[] => true,
														['u', '\u{1}', s @ ..] => super::t5::n14231(s, out),
														_ => false,
													}
												}
												['n', s @ ..] => {
													super::push(out, 7693);
													match s {
														[] => true,
														['i', 'e', '\u{1}', s @ ..] => super::t5::n14232(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 7301);
											match s {
												[] => true,
												['i', 'e', '\u{1}', s @ ..] => {
													super::push(out, 7694);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2244);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4497);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 6233);
									match s {
										[] => true,
										['r', 'f', 's', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7302);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ä', 'h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7303);
											match s {
												[] => true,
												['e', 'v', '\u{1}', s @ ..] => {
													super::push(out, 7695);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2245);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2246);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4499);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6234);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 6235);
									match s {
										[] => true,
										['c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6236);
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
				['ö', s @ ..] => {
					super::push(out, 2247);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4500);
							match s {
								[] => true,
								['k', 'r', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', 'ü', '\u{1}', s @ ..] => {
											super::push(out, 7304);
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											super::push(out, 7305);
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6237);
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
		_ => false,
	}
}

/// Words ending -keln.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['r', 'o', s @ ..] => {
					super::push(out, 2248);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4501);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4502);
							match s {
								[] => true,
								['u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6238);
									s.is_empty()
								}
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
			super::push(out, 889);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2249);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4503);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 4504);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6239);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2250);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4505);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4506);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 4507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6240);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6241);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 6242);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6243);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6244);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 6245);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6246);
									match s {
										[] => true,
										['n', 'e', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 7696);
													match s {
														[] => true,
														['e', 't', 'i', 'e', 'w', '\u{1}', s @ ..] => super::t5::n14233(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2251);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => {
					super::push(out, 2252);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4509);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 7306);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 7307);
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
							super::push(out, 4510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4511);
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
		['e', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2253);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2254);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4512);
							s.is_empty()
						}
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 891);
			match s {
				[] => true,
				['e', 'r', 'p', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4514);
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
				['i', s @ ..] => {
					super::push(out, 2255);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 4515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4516);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6247);
									s.is_empty()
								}
								['r', s @ ..] => super::t5::n9445(s, out),
								['t', 'n', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => super::t5::n5933(s, out),
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t5::n953(s, out),
		['u', s @ ..] => super::t5::n954(s, out),
		['ä', s @ ..] => super::t5::n955(s, out),
		_ => false,
	}
}

/// Words ending -meln.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['e', 'i', 'r', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2257);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 6252);
									match s {
										[] => true,
										['u', 'r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7697);
													s.is_empty()
												}
												['e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 7698);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6253);
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
							super::push(out, 4521);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', 'c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6254);
									match s {
										[] => true,
										['e', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'z', '\u{1}', s @ ..] => {
									super::push(out, 6255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6256);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6257);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 7308);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 7309);
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
									super::push(out, 6258);
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
				['e', s @ ..] => {
					super::push(out, 2258);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 6259);
									match s {
										[] => true,
										['e', 'b', 'r', 'o', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2259);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7310);
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
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6262);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2260);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6263);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 7311);
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 2261);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4523);
							match s {
								[] => true,
								['m', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 6264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['r', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4525);
							match s {
								[] => true,
								['e', s @ ..] => super::t5::n9480(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2262);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tunneln.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 897);
			match s {
				[] => true,
				['e', 't', 'n', 'u', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -peln.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 4526);
							s.is_empty()
						}
						['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 899);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2263);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6265);
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
				['e', s @ ..] => {
					super::push(out, 2264);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4528);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 7312);
											match s {
												[] => true,
												['u', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 7313);
											match s {
												[] => true,
												['c', 'o', 'h', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 7314);
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
						['t', s @ ..] => {
							super::push(out, 4529);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', s @ ..] => {
											super::push(out, 7315);
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
					super::push(out, 2265);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4530);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', 'c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6266);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6267);
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
				['ü', 'r', s @ ..] => {
					super::push(out, 2266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 900);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2267);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 6268);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 6269);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7316);
											s.is_empty()
										}
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
					super::push(out, 2268);
					match s {
						[] => true,
						['n', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4531);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2269);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4534);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4535);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6271);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 6272);
									match s {
										[] => true,
										['c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6273);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 6274);
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
						['t', 's', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2270);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6275);
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6276);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => super::t5::n9502(s, out),
								['t', 'n', 'e', '\u{1}', s @ ..] => super::t5::n9503(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => super::t5::n2687(s, out),
				['ü', 'r', s @ ..] => super::t5::n2688(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t5::n964(s, out),
		_ => false,
	}
}

/// Words ending -seln.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	match s {
		[] => true,
		['a', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 902);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 903);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4539);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4540);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6279);
									match s {
										[] => true,
										['a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											super::push(out, 7317);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'b', '\u{1}', s @ ..] => {
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
		['h', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['c', 'e', 'w', s @ ..] => {
					super::push(out, 2273);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 4541);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 4542);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4543);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4544);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 7318);
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
							super::push(out, 4545);
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
			super::push(out, 905);
			match s {
				[] => true,
				['a', 'g', 'f', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2275);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6280);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 6281);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6282);
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
				['ö', s @ ..] => {
					super::push(out, 2276);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 6283);
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
		['s', s @ ..] => {
			super::push(out, 907);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2277);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4546);
							s.is_empty()
						}
						['m', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4547);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4548);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 6284);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4549);
							match s {
								[] => true,
								['q', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 7319);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6285);
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
						['k', s @ ..] => {
							super::push(out, 4550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2279);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2280);
					match s {
						[] => true,
						['r', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6286);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7320);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6287);
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
				['u', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t5::n5988(s, out),
						['b', 'a', '\u{1}', s @ ..] => super::t5::n5989(s, out),
						_ => false,
					}
				}
				['ü', s @ ..] => super::t5::n2700(s, out),
				_ => false,
			}
		}
		['t', 'ä', 'r', s @ ..] => super::t5::n971(s, out),
		['u', s @ ..] => super::t5::n972(s, out),
		['ö', s @ ..] => super::t5::n973(s, out),
		_ => false,
	}
}

/// Words ending -teln.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['f', 'ü', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
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
										['r', s @ ..] => {
											super::push(out, 7322);
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
								['p', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 7323);
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
						['u', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6292);
									s.is_empty()
								}
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
			super::push(out, 911);
			match s {
				[] => true,
				['e', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4557);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['ä', 'z', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4558);
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
		['s', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['r', 'u', 'w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 2287);
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6293);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 6294);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 6295);
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
				['e', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6296);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 7324);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 6297);
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
						['z', s @ ..] => {
							super::push(out, 4561);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6298);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 6299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2290);
					match s {
						[] => true,
						['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4562);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4563);
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
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6300);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ö', s @ ..] => super::t5::n2718(s, out),
				['ü', s @ ..] => super::t5::n2719(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t5::n981(s, out),
		_ => false,
	}
}

/// Words ending -veln.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['e', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2293);
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

/// Words ending -zeln.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['e', 'r', 'b', 'f', 'u', 'a', '\u{1}', s @ ..] => {
			super::push(out, 917);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2294);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 6306);
									s.is_empty()
								}
								['r', 'e', 't', 'n', 'u', 'r', 'e', 'h', '\u{1}', s @ ..] => {
									super::push(out, 6307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'e', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4567);
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
		['r', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4568);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 6308);
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
		['t', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2296);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4569);
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7329);
											s.is_empty()
										}
										['f', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 7330);
											s.is_empty()
										}
										['g', 'e', 'w', '\u{1}', s @ ..] => {
											super::push(out, 7331);
											s.is_empty()
										}
										['m', 'u', 'r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7699);
													s.is_empty()
												}
												['e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 7700);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 't', 'n', 'u', 'r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7701);
													s.is_empty()
												}
												['e', 'h', '\u{1}', s @ ..] => {
													super::push(out, 7702);
													s.is_empty()
												}
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
							super::push(out, 4570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6309);
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
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2297);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6310);
									match s {
										[] => true,
										['u', 'a', 'r', 'e', 'h', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 6311);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 7332);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ßeln.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['a', 'h', 'c', 's', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 921);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4572);
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
		['ü', s @ ..] => {
			super::push(out, 923);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dern.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				['n', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 2303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2304);
					match s {
						[] => true,
						['h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4579);
							s.is_empty()
						}
						['l', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'z', '\u{1}', s @ ..] => {
									super::push(out, 6317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 932);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4580);
							s.is_empty()
						}
						['l', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6318);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6319);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 7334);
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
									super::push(out, 6320);
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 933);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['n', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 's', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6321);
									s.is_empty()
								}
								['ß', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 6322);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 2309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4581);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4582);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4583);
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
		['l', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4584);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4585);
							match s {
								[] => true,
								['c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 7335);
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
							super::push(out, 4586);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6323);
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
						['w', s @ ..] => {
							super::push(out, 4587);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 6324);
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
		['n', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2310);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6325);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 6326);
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
								['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 6327);
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 6328);
									match s {
										[] => true,
										['u', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => super::t5::n9612(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t5::n2748(s, out),
				['i', s @ ..] => super::t5::n2749(s, out),
				['o', s @ ..] => super::t5::n2750(s, out),
				['u', s @ ..] => super::t5::n2751(s, out),
				['ä', s @ ..] => super::t5::n2752(s, out),
				['ü', s @ ..] => super::t5::n2753(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t5::n1003(s, out),
		['r', s @ ..] => super::t5::n1004(s, out),
		['u', s @ ..] => super::t5::n1005(s, out),
		['ä', s @ ..] => super::t5::n1006(s, out),
		['ö', s @ ..] => super::t5::n1007(s, out),
		['ü', s @ ..] => super::t5::n1008(s, out),
		_ => false,
	}
}

/// Words ending -hiefern.
pub(crate) fn n6084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4602);
	s.is_empty()
}

/// Words ending -liefern.
pub(crate) fn n6085(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6349);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6350);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6351);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6352);
			match s {
				[] => true,
				['i', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6353);
			match s {
				[] => true,
				['e', 'b', 'ü', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6354);
			match s {
				[] => true,
				['u', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6355);
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

/// Words ending -ffern.
pub(crate) fn n1010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 943);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2324);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gfern.
pub(crate) fn n1011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 944);
	s.is_empty()
}

/// Words ending -ifern.
pub(crate) fn n1012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 945);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4603);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4604);
					match s {
						[] => true,
						['c', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 6356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4605);
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

/// Words ending -pfern.
pub(crate) fn n1013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 946);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4606);
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
		['u', s @ ..] => {
			super::push(out, 2327);
			match s {
				[] => true,
				['k', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4607);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ufern.
pub(crate) fn n1014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 947);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 2328);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -äfern.
pub(crate) fn n1015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['l', 'h', 'c', 's', 'n', 'i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 2329);
			s.is_empty()
		}
		_ => false,
	}
}

