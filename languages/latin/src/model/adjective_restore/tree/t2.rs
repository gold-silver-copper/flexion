use flexion::Path;

/// Words ending -pens.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 930);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 2684);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2685);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 5419);
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
							super::push(out, 5420);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5421);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5422);
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
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5423);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 12362);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
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
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['l', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['u', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5424);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 932);
			match s {
				[] => true,
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5425);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8900);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8901);
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
						['i', s @ ..] => {
							super::push(out, 5426);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5427);
							match s {
								[] => true,
								['r', s @ ..] => {
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
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5428);
							match s {
								[] => true,
								['e', s @ ..] => {
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
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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
		['r', s @ ..] => {
			super::push(out, 933);
			match s {
				[] => true,
				['e', 'c', 'e', s @ ..] => {
					super::push(out, 2687);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5429);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5430);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5431);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5432);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8902);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 8903);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rens.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 294);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
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
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5433);
							match s {
								[] => true,
								['n', 'm', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5434);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 5435);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5436);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8904);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8905);
									match s {
										[] => true,
										['a', 'r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 936);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2689);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'q', s @ ..] => {
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
				['f', s @ ..] => {
					super::push(out, 2690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5437);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 8906);
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
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => super::t4::n7357(s, out),
						['i', s @ ..] => {
							super::push(out, 5439);
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => super::t4::n7360(s, out),
						['o', s @ ..] => super::t4::n7361(s, out),
						['r', 'e', 'p', '\u{1}', s @ ..] => super::t4::n7362(s, out),
						['s', s @ ..] => super::t4::n7363(s, out),
						['u', 'a', '\u{1}', s @ ..] => super::t4::n7364(s, out),
						_ => false,
					}
				}
				['g', s @ ..] => super::t4::n3126(s, out),
				['h', s @ ..] => super::t4::n3127(s, out),
				['m', s @ ..] => super::t4::n3128(s, out),
				['s', s @ ..] => super::t4::n3129(s, out),
				['t', s @ ..] => super::t4::n3130(s, out),
				['u', s @ ..] => super::t4::n3131(s, out),
				['v', s @ ..] => super::t4::n3132(s, out),
				['x', 'e', '\u{1}', s @ ..] => super::t4::n3133(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t4::n1044(s, out),
		['i', s @ ..] => super::t4::n1045(s, out),
		['o', s @ ..] => super::t4::n1046(s, out),
		['r', s @ ..] => super::t4::n1047(s, out),
		['u', s @ ..] => super::t4::n1048(s, out),
		_ => false,
	}
}

/// Words ending -sens.
pub(crate) fn n319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 295);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 943);
			match s {
				[] => true,
				['a', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'n', 'm', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2704);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 5462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 944);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2705);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2706);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2707);
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
		['p', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 947);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 948);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tens.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 949);
			match s {
				[] => true,
				['l', s @ ..] => {
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
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'b', '\u{1}', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 950);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5463);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 8918);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 12368);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 15033);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n31953(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 15034);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n31954(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 12369);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 12370);
											match s {
												[] => true,
												['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 12371);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 12372);
											match s {
												[] => true,
												['n', 'e', 'g', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 8919);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 12373);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
						['n', s @ ..] => {
							super::push(out, 5464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
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
								['o', s @ ..] => {
									super::push(out, 8920);
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 951);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2708);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 8921);
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
				['b', s @ ..] => {
					super::push(out, 2709);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2710);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5465);
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
				['p', s @ ..] => super::t4::n3168(s, out),
				['r', s @ ..] => {
					super::push(out, 2713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n1060(s, out),
		['o', s @ ..] => super::t4::n1061(s, out),
		['r', s @ ..] => super::t4::n1062(s, out),
		['s', s @ ..] => super::t4::n1063(s, out),
		['t', s @ ..] => super::t4::n1064(s, out),
		['u', s @ ..] => {
			super::push(out, 957);
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
				['e', 'd', '\u{1}', s @ ..] => super::t4::n3182(s, out),
				['o', 'c', '\u{1}', s @ ..] => super::t4::n3183(s, out),
				['p', '\u{1}', s @ ..] => super::t4::n3184(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uens.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 297);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5498);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8939);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8940);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5499);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'r', '\u{1}', s @ ..] => {
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
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 959);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['d', s @ ..] => {
			super::push(out, 960);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
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
		['g', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
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
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 962);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2720);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2721);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2722);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5500);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => super::t4::n13711(s, out),
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => super::t4::n7509(s, out),
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => super::t4::n7510(s, out),
						['n', s @ ..] => super::t4::n7511(s, out),
						['o', s @ ..] => super::t4::n7512(s, out),
						['r', s @ ..] => super::t4::n7513(s, out),
						['s', s @ ..] => super::t4::n7514(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t4::n3193(s, out),
				['l', s @ ..] => super::t4::n3194(s, out),
				['p', s @ ..] => super::t4::n3195(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t4::n1071(s, out),
		['p', s @ ..] => super::t4::n1072(s, out),
		['q', s @ ..] => super::t4::n1073(s, out),
		['r', s @ ..] => super::t4::n1074(s, out),
		['s', s @ ..] => super::t4::n1075(s, out),
		['t', s @ ..] => super::t4::n1076(s, out),
		['x', 'e', '\u{1}', s @ ..] => super::t4::n1077(s, out),
		_ => false,
	}
}

/// Words ending -vens.
pub(crate) fn n322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 298);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2745);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 971);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2746);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['l', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5515);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
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
						['v', s @ ..] => {
							super::push(out, 5516);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8970);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 12393);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 12394);
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
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8971);
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
									super::push(out, 8972);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 2748);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5517);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['m', s @ ..] => {
					super::push(out, 2749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 5518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5519);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => super::t5::n7571(s, out),
						['i', s @ ..] => {
							super::push(out, 5521);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5522);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => super::t5::n13810(s, out),
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => super::t5::n7574(s, out),
						['o', s @ ..] => super::t5::n7575(s, out),
						['r', s @ ..] => super::t5::n7576(s, out),
						['s', s @ ..] => super::t5::n7577(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t5::n3236(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t5::n1083(s, out),
		_ => false,
	}
}

/// Words ending -xens.
pub(crate) fn n323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 299);
	match s {
		[] => true,
		['e', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
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
				['t', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -icus.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 330);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2778);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2779);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5557);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2780);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2781);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8988);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5560);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2783);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5562);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2784);
					match s {
						[] => true,
						['e', 'r', 'y', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8990);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8991);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2785);
					match s {
						[] => true,
						['e', 'l', 'o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 2787);
					match s {
						[] => true,
						['a', 's', 's', 'o', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8992);
									s.is_empty()
								}
								['r', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8993);
									s.is_empty()
								}
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
			super::push(out, 1001);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2788);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 5567);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5568);
							match s {
								[] => true,
								['o', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 8994);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 5569);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1002);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2790);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2791);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 5570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2792);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5571);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 5572);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8995);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8996);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 5573);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 5574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2793);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5575);
							match s {
								[] => true,
								['l', 's', s @ ..] => {
									super::push(out, 8997);
									match s {
										[] => true,
										['n', 'e', 'e', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 12396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5576);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5577);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 2794);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5578);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 8998);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8999);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
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
						['n', s @ ..] => {
							super::push(out, 5580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2795);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2796);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['a', 'd', 'u', s @ ..] => {
					super::push(out, 2797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t5::n1121(s, out),
		['g', s @ ..] => super::t5::n1122(s, out),
		['h', s @ ..] => {
			super::push(out, 1006);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2803);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5598);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5599);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 5600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t5::n3307(s, out),
				['t', s @ ..] => super::t5::n3308(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t5::n1124(s, out),
		['m', s @ ..] => super::t5::n1125(s, out),
		['n', s @ ..] => super::t5::n1126(s, out),
		['o', s @ ..] => super::t5::n1127(s, out),
		['p', s @ ..] => super::t5::n1128(s, out),
		['r', s @ ..] => super::t5::n1129(s, out),
		['s', s @ ..] => super::t5::n1130(s, out),
		['t', s @ ..] => super::t5::n1131(s, out),
		['v', s @ ..] => super::t5::n1132(s, out),
		['x', s @ ..] => super::t5::n1133(s, out),
		_ => false,
	}
}

/// The word promiscus.
pub(crate) fn n7856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5761);
	s.is_empty()
}

/// Words ending -riscus.
pub(crate) fn n3379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2867);
	match s {
		[] => true,
		['a', 's', 's', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5762);
			s.is_empty()
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 5763);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5764);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uscus.
pub(crate) fn n1141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1022);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'o', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2868);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 5765);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5766);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucus.
pub(crate) fn n364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['l', 'g', s @ ..] => {
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
				['p', s @ ..] => {
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
		['d', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ycus.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	s.is_empty()
}

/// Words ending -ubidus.
pub(crate) fn n3391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2874);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 5771);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5772);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cidus.
pub(crate) fn n1150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1031);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'l', 'u', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 2876);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2877);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 2878);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2879);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2880);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5773);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2881);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9202);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 9203);
							match s {
								[] => true,
								['e', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9204);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9205);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5775);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5776);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -didus.
pub(crate) fn n1151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1032);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2882);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9206);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fidus.
pub(crate) fn n1152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1033);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2883);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2884);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5777);
					match s {
						[] => true,
						['d', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5778);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9207);
							match s {
								[] => true,
								['n', 'n', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 12466);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'u', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5779);
					match s {
						[] => true,
						['q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2885);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2886);
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

/// Words ending -gidus.
pub(crate) fn n1153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1034);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 2887);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9208);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 9209);
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
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lidus.
pub(crate) fn n1154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1035);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2889);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 5781);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 2890);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9210);
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
		['l', s @ ..] => {
			super::push(out, 2891);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
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
								['e', s @ ..] => {
									super::push(out, 12467);
									match s {
										[] => true,
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2892);
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
						['n', s @ ..] => {
							super::push(out, 9211);
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
				['t', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -midus.
pub(crate) fn n1155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1036);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2893);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2894);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5783);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 5785);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5786);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5787);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pidus.
pub(crate) fn n1157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1038);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 2895);
			s.is_empty()
		}
		['u', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ridus.
pub(crate) fn n1158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1039);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2896);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 2897);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2898);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2899);
			match s {
				[] => true,
				['o', 'h', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 2900);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2901);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tidus.
pub(crate) fn n1159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1040);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 2902);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5789);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 5790);
					s.is_empty()
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2903);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2904);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uidus.
pub(crate) fn n1160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1041);
	s.is_empty()
}

/// Words ending -vidus.
pub(crate) fn n1161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1042);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2905);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5792);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5793);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2906);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 5794);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5795);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5796);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2907);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2908);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2909);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ldus.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 341);
	s.is_empty()
}

/// Words ending -ndus.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 342);
	match s {
		[] => true,
		['a', s @ ..] => super::t5::n1162(s, out),
		['e', s @ ..] => super::t5::n1163(s, out),
		['i', s @ ..] => {
			super::push(out, 1045);
			s.is_empty()
		}
		['u', s @ ..] => super::t5::n1165(s, out),
		_ => false,
	}
}

/// Words ending -odus.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 343);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 2960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['m', 'o', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
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
		['p', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['o', 'l', 'i', s @ ..] => {
					super::push(out, 2961);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdus.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 344);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2962);
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
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

/// Words ending -udus.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 345);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['c', 'i', 'v', 'a', s @ ..] => {
					super::push(out, 2963);
					match s {
						[] => true,
						['l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6069);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2964);
					match s {
						[] => true,
						['m', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1056);
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

/// Words ending -ceus.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 348);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1072);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2990);
					match s {
						[] => true,
						['a', 'c', 'a', s @ ..] => {
							super::push(out, 6084);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 9825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2991);
					match s {
						[] => true,
						['i', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2992);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2993);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2994);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6086);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6087);
							match s {
								[] => true,
								['i', 'l', s @ ..] => {
									super::push(out, 9826);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6088);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
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
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2995);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6089);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 9827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2996);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2997);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6090);
							match s {
								[] => true,
								['r', 'b', 'm', 'e', 'm', s @ ..] => {
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
						['e', s @ ..] => {
							super::push(out, 6091);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['i', s @ ..] => {
							super::push(out, 6092);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9828);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 13265);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 9829);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 9830);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 9831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2998);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6094);
							match s {
								[] => true,
								['l', 'o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['h', s @ ..] => {
							super::push(out, 6095);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2999);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3000);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6097);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6098);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1073);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1074);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6100);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1075);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3002);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6102);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3003);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3004);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 6104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3005);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6105);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6106);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6108);
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
					super::push(out, 3007);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3008);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6109);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1076);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1077);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3009);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1078);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1079);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1080);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1081);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ideus.
pub(crate) fn n1204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1084);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 3011);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6111);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6112);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6113);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6114);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9833);
							match s {
								[] => true,
								['t', 'y', 'r', 'a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 15634);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 15635);
											match s {
												[] => true,
												['e', 'r', 'y', 'h', 't', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 9834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6115);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6116);
					match s {
						[] => true,
						['l', 'e', 'd', s @ ..] => {
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
				['y', s @ ..] => {
					super::push(out, 6117);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'y', 'h', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 15636);
											s.is_empty()
										}
										['l', 'y', 't', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 15637);
											match s {
												[] => true,
												['o', '\u{1}', s @ ..] => {
													super::push(out, 17118);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3012);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6118);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6119);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6120);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -odeus.
pub(crate) fn n1206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1086);
	s.is_empty()
}

/// Words ending -udeus.
pub(crate) fn n1207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1087);
	s.is_empty()
}

/// Words ending -geus.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1088);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1089);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1090);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3013);
					s.is_empty()
				}
				['y', 'r', 'a', 'h', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', 'y', 't', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9835);
									s.is_empty()
								}
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
			super::push(out, 1092);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1093);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -heus.
pub(crate) fn n379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3014);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3016);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1097);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3018);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6121);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6122);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3019);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ieus.
pub(crate) fn n380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	s.is_empty()
}

/// Words ending -leus.
pub(crate) fn n381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1098);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1099);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1100);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3020);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1101);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3022);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3023);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -meus.
pub(crate) fn n382(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 3024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1103);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1104);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1105);
			match s {
				[] => true,
				['m', 'a', 'n', 'n', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 3025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1106);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -neus.
pub(crate) fn n383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1107);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3026);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3027);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6123);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6124);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3028);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6126);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3029);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3030);
					match s {
						[] => true,
						['n', 'i', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 9836);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3031);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3032);
					match s {
						[] => true,
						['a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6128);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3033);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9838);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 13266);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'm', 'e', 't', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3034);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6129);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9839);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 9840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6130);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 9841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6131);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9842);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 9843);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 13267);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 13268);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 13269);
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 15638);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 9844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3035);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9845);
									s.is_empty()
								}
								['o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9846);
									s.is_empty()
								}
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
			super::push(out, 1108);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1109);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6133);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6134);
							match s {
								[] => true,
								['r', 'g', 'i', 'n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 3037);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3038);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6135);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3039);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1111);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3041);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
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
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3042);
					s.is_empty()
				}
				['d', s @ ..] => super::t5::n3575(s, out),
				['g', s @ ..] => super::t5::n3576(s, out),
				['l', s @ ..] => super::t5::n3577(s, out),
				['m', s @ ..] => super::t5::n3578(s, out),
				['p', s @ ..] => super::t5::n3579(s, out),
				['t', s @ ..] => {
					super::push(out, 3048);
					s.is_empty()
				}
				['u', s @ ..] => super::t5::n3581(s, out),
				['v', s @ ..] => {
					super::push(out, 3050);
					s.is_empty()
				}
				['x', s @ ..] => super::t5::n3583(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t5::n1232(s, out),
		['n', s @ ..] => super::t5::n1233(s, out),
		['o', s @ ..] => super::t5::n1234(s, out),
		['r', s @ ..] => super::t5::n1235(s, out),
		['u', s @ ..] => super::t5::n1236(s, out),
		_ => false,
	}
}

/// Words ending -oeus.
pub(crate) fn n384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	s.is_empty()
}

/// Words ending -peus.
pub(crate) fn n385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1117);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1118);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1119);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1120);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -reus.
pub(crate) fn n386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 358);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3056);
					match s {
						[] => true,
						['l', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3057);
					match s {
						[] => true,
						['p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1122);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3060);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3061);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3062);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3063);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6164);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6165);
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
				['p', s @ ..] => {
					super::push(out, 3064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1123);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 3066);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3067);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3068);
					match s {
						[] => true,
						['r', 'o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 3069);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3070);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6166);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'u', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1124);
			match s {
				[] => true,
				['a', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
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
				['u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1125);
			match s {
				[] => true,
				['i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1126);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'l', 'u', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3071);
					match s {
						[] => true,
						['r', 'u', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'r', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 13271);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -seus.
pub(crate) fn n387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 359);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1127);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6167);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1129);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3072);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3073);
					match s {
						[] => true,
						['g', s @ ..] => {
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['r', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6169);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
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
		['y', s @ ..] => {
			super::push(out, 1132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -teus.
pub(crate) fn n388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 360);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1133);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['h', 'p', 'u', 'e', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 6170);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1134);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3076);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3077);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3078);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 6172);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1137);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1138);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1139);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1140);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 3079);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ueus.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 361);
	s.is_empty()
}

/// Words ending -veus.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 362);
	match s {
		[] => true,
		['a', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1141);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xeus.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 363);
	s.is_empty()
}

/// Words ending -alicius.
pub(crate) fn n8393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6192);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 9860);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9861);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13274);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9862);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9863);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13276);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 13277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 9864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -enicius.
pub(crate) fn n8397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6196);
	s.is_empty()
}

/// Words ending -unicius.
pub(crate) fn n8398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6197);
	s.is_empty()
}

/// Words ending -picius.
pub(crate) fn n3663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3098);
	s.is_empty()
}

/// Words ending -ricius.
pub(crate) fn n3664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3099);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6198);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6199);
			match s {
				[] => true,
				['n', 'i', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6200);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13278);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13279);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13280);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15639);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15640);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 13281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6201);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9865);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9866);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9867);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sicius.
pub(crate) fn n3665(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3100);
	s.is_empty()
}

/// Words ending -ticius.
pub(crate) fn n3666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3101);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6202);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9868);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9869);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13282);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 13283);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13284);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 15641);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 15642);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9870);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13285);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9871);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 13287);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6203);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9872);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9873);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15643);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 15644);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6204);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6205);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9875);
					match s {
						[] => true,
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
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6206);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 6207);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 9877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6208);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 6209);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9878);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vicius.
pub(crate) fn n3667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3102);
	s.is_empty()
}

/// Words ending -ncius.
pub(crate) fn n1284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1157);
	s.is_empty()
}

/// Words ending -ocius.
pub(crate) fn n1285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1158);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6211);
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

/// Words ending -rcius.
pub(crate) fn n1286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1159);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3103);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -scius.
pub(crate) fn n1287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1160);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3105);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3106);
			match s {
				[] => true,
				['n', 'm', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3107);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6212);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6213);
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

/// Words ending -ucius.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1161);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3108);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ycius.
pub(crate) fn n1289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1162);
	s.is_empty()
}

/// Words ending -dius.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1163);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1164);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1165);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6214);
							match s {
								[] => true,
								['h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['p', s @ ..] => {
					super::push(out, 3111);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6215);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 1166);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3114);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1167);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eius.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1170);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3120);
					match s {
						[] => true,
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1171);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 1172);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1173);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1174);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1175);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1176);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3122);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3124);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6217);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6218);
							match s {
								[] => true,
								['p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1177);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1178);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1179);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					super::push(out, 3125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1180);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gius.
pub(crate) fn n408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 379);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1181);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1182);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6220);
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
		['r', s @ ..] => {
			super::push(out, 1183);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1184);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hius.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 380);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1185);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3129);
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
		['t', s @ ..] => {
			super::push(out, 1186);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3131);
					match s {
						[] => true,
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lius.
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 381);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1187);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1188);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6222);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3136);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3137);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6224);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6226);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 6227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1189);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1190);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6228);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6229);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9881);
									match s {
										[] => true,
										['l', 'l', 'i', 'h', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 13289);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'n', 'o', 'l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 9882);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 13290);
											s.is_empty()
										}
										['l', 'e', s @ ..] => {
											match s {
												[] => true,
												['b', s @ ..] => {
													super::push(out, 15646);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 15647);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 9883);
									match s {
										[] => true,
										['u', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 13291);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 9884);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9885);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 13292);
											match s {
												[] => true,
												['n', 'i', 'm', 'u', 'c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 15648);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 13293);
											match s {
												[] => true,
												['v', '\u{1}', s @ ..] => {
													super::push(out, 15649);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 13294);
											match s {
												[] => true,
												['e', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
													super::push(out, 15650);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 13295);
											match s {
												[] => true,
												['c', 'a', '\u{1}', s @ ..] => {
													super::push(out, 15651);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1191);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3139);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mius.
pub(crate) fn n411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 382);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1193);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1194);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1195);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nius.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 383);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3142);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3144);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3145);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 9886);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9887);
									s.is_empty()
								}
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
			super::push(out, 1197);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1198);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3149);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3150);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3151);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6230);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3152);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3153);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 6232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3154);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6233);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 6234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1199);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1200);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1201);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3158);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6235);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6236);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6237);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6238);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3159);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3160);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3161);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3162);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3163);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3164);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3165);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6240);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 6241);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6242);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3167);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1202);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6245);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9888);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 9889);
									s.is_empty()
								}
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
			super::push(out, 1203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oius.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 384);
	s.is_empty()
}

/// Words ending -pius.
pub(crate) fn n414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 385);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1204);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'u', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', s @ ..] => {
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
		['s', 'a', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1205);
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

/// Words ending -rius.
pub(crate) fn n415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 386);
	match s {
		[] => true,
		['a', s @ ..] => super::t5::n1338(s, out),
		['b', s @ ..] => {
			super::push(out, 1207);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3187);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3188);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1208);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3190);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3191);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 6318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 3192);
					s.is_empty()
				}
				['l', 'c', 'u', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 3193);
					s.is_empty()
				}
				['m', 'o', s @ ..] => {
					super::push(out, 3194);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3195);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3196);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1209);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1210);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3198);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3199);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3200);
					match s {
						[] => true,
						['e', 'm', 'e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3201);
					s.is_empty()
				}
				['s', s @ ..] => super::t5::n3781(s, out),
				['t', s @ ..] => super::t5::n3782(s, out),
				['x', s @ ..] => {
					super::push(out, 3204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1211);
			match s {
				[] => true,
				['o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1212);
			match s {
				[] => true,
				['a', s @ ..] => super::t5::n3785(s, out),
				['s', s @ ..] => super::t5::n3786(s, out),
				['u', s @ ..] => super::t5::n3787(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t5::n1345(s, out),
		['y', s @ ..] => super::t5::n1346(s, out),
		_ => false,
	}
}

/// Words ending -sius.
pub(crate) fn n416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 387);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1215);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 1216);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3213);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 3214);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1217);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1218);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3216);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1220);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6337);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tius.
pub(crate) fn n417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 388);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1222);
			match s {
				[] => true,
				['c', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3218);
					s.is_empty()
				}
				['f', 'i', 'l', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 3219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1223);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6339);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6340);
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
				['n', s @ ..] => {
					super::push(out, 3221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1224);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3222);
					match s {
						[] => true,
						['a', 't', 'r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 6341);
							s.is_empty()
						}
						['i', 't', 'n', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3223);
					match s {
						[] => true,
						['u', 'b', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 6343);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3225);
					match s {
						[] => true,
						['t', 'u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 6344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6345);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6346);
							match s {
								[] => true,
								['d', 'n', 'e', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10057);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 6347);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6348);
							match s {
								[] => true,
								['d', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10058);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6349);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 10059);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6350);
							match s {
								[] => true,
								['m', 'e', '\u{1}', s @ ..] => {
									super::push(out, 10060);
									s.is_empty()
								}
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
			super::push(out, 1226);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1227);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1228);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1229);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1230);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1231);
			match s {
				[] => true,
				['r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vius.
pub(crate) fn n418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1232);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10061);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 10062);
									match s {
										[] => true,
										['t', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 15695);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 15696);
													s.is_empty()
												}
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
							super::push(out, 6351);
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
			super::push(out, 1233);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3232);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1234);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 3235);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3237);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3238);
					s.is_empty()
				}
				['x', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 3239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1235);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1236);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1237);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xius.
pub(crate) fn n419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 1238);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1239);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
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

/// Words ending -icapillus.
pub(crate) fn n8602(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 10070);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sillus.
pub(crate) fn n3842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3252);
	match s {
		[] => true,
		['u', 'p', s @ ..] => {
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

/// Words ending -uillus.
pub(crate) fn n3844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3254);
	match s {
		[] => true,
		['q', s @ ..] => {
			super::push(out, 6365);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -villus.
pub(crate) fn n3845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3255);
	match s {
		[] => true,
		['o', s @ ..] => {
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

/// Words ending -ullus.
pub(crate) fn n1387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1255);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3257);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yllus.
pub(crate) fn n1388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1256);
	match s {
		[] => true,
		['h', 'p', 'o', s @ ..] => {
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 6367);
					s.is_empty()
				}
				['r', 'c', 'i', s @ ..] => {
					super::push(out, 6368);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 10072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -olus.
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 1257);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1258);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3258);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							super::push(out, 6369);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 10073);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'u', s @ ..] => {
							super::push(out, 6370);
							match s {
								[] => true,
								['l', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 10074);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'u', s @ ..] => {
							super::push(out, 6371);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 10075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6372);
							match s {
								[] => true,
								['r', 'e', 's', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 10076);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1259);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 3260);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3261);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6373);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6374);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3262);
					match s {
						[] => true,
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1260);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1261);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3264);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6377);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 6378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1262);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1263);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6379);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 10077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plus.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1264);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1265);
			match s {
				[] => true,
				['t', 'l', 'u', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'q', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 6381);
							s.is_empty()
						}
						['s', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 6382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 1266);
			match s {
				[] => true,
				['a', s @ ..] => {
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
				['i', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1267);
			match s {
				[] => true,
				['t', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 3266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1268);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3267);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6383);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 13394);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 10078);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 10079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6384);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3268);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 10080);
									match s {
										[] => true,
										['u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 13395);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ulus.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1269);
			match s {
				[] => true,
				['i', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1270);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3270);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3271);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6387);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10081);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 15697);
													match s {
														[] => true,
														['d', '\u{1}', s @ ..] => super::t5::n33192(s, out),
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
								['n', s @ ..] => {
									super::push(out, 10082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 6388);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10083);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 13396);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 6389);
							match s {
								[] => true,
								['r', 'u', 't', s @ ..] => {
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
							super::push(out, 6390);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10084);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 10085);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10086);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6391);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10087);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10088);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10089);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 13397);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 13398);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 6392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3272);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6393);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6394);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10090);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3273);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10091);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 13399);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 15698);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 13400);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 10092);
									s.is_empty()
								}
								['n', 'i', 'm', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1271);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3274);
					match s {
						[] => true,
						['r', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 3275);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6395);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10093);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 10094);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 6396);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 10095);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 6397);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 6398);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10096);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 6399);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6400);
							s.is_empty()
						}
						['r', s @ ..] => super::t5::n8648(s, out),
						['v', s @ ..] => {
							super::push(out, 6402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t5::n3875(s, out),
				['u', s @ ..] => super::t5::n3876(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t5::n1404(s, out),
		['g', s @ ..] => super::t5::n1405(s, out),
		['l', s @ ..] => super::t5::n1406(s, out),
		['m', s @ ..] => super::t5::n1407(s, out),
		['n', s @ ..] => super::t5::n1408(s, out),
		['p', s @ ..] => super::t5::n1409(s, out),
		['r', s @ ..] => super::t5::n1410(s, out),
		['s', s @ ..] => super::t5::n1411(s, out),
		['t', s @ ..] => super::t5::n1412(s, out),
		['v', s @ ..] => super::t5::n1413(s, out),
		_ => false,
	}
}

/// Words ending -ylus.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['t', 'c', 'a', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -imus.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1282);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1284);
			match s {
				[] => true,
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1285);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3298);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 3299);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3300);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10116);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 10117);
									match s {
										[] => true,
										['i', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['i', s @ ..] => {
													super::push(out, 15700);
													match s {
														[] => true,
														['r', 'e', 'v', '\u{1}', s @ ..] => super::t5::n33193(s, out),
														_ => false,
													}
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
								['t', s @ ..] => {
									super::push(out, 10118);
									s.is_empty()
								}
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
			super::push(out, 1286);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'm', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'd', 'i', 'c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1287);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1288);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 3301);
					match s {
						[] => true,
						['a', 'u', 'q', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3303);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6419);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10120);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 13405);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 13406);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 15701);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 15702);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 13407);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 13408);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['l', 'a', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 15703);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n33194(s, out),
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
									super::push(out, 10121);
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 15704);
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
								['g', s @ ..] => {
									super::push(out, 10122);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 10123);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10124);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 13409);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 13410);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 15705);
													s.is_empty()
												}
												['d', s @ ..] => super::t5::n28639(s, out),
												['t', s @ ..] => super::t5::n28640(s, out),
												['v', s @ ..] => super::t5::n28641(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => super::t5::n8688(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t5::n3913(s, out),
				['u', s @ ..] => super::t5::n3914(s, out),
				_ => false,
			}
		}
		['s', s @ ..] => super::t5::n1423(s, out),
		['t', s @ ..] => super::t5::n1424(s, out),
		['x', s @ ..] => super::t5::n1425(s, out),
		_ => false,
	}
}

/// Words ending -anus.
pub(crate) fn n437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1306);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3316);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3317);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 10146);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13474);
											s.is_empty()
										}
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
					super::push(out, 3319);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
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
				['u', s @ ..] => {
					super::push(out, 3320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1307);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3321);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3322);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6433);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10147);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 10148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6434);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10149);
									s.is_empty()
								}
								['e', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 10150);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 10151);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10152);
									match s {
										[] => true,
										['m', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 15814);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 15815);
													s.is_empty()
												}
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
							super::push(out, 6435);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6436);
							match s {
								[] => true,
								['a', s @ ..] => {
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
						['v', s @ ..] => {
							super::push(out, 6437);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 6438);
							match s {
								[] => true,
								['e', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 13475);
											s.is_empty()
										}
										['o', 'e', 'n', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3323);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3324);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3325);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10155);
									match s {
										[] => true,
										['e', 'p', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3326);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3327);
					match s {
						[] => true,
						['a', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6440);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1308);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3328);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'r', s @ ..] => {
					super::push(out, 3329);
					match s {
						[] => true,
						['o', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3330);
					match s {
						[] => true,
						['u', 'm', 'a', s @ ..] => {
							super::push(out, 6443);
							match s {
								[] => true,
								['r', 't', 'l', 'u', '\u{1}', s @ ..] => {
									super::push(out, 10156);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1309);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3332);
					match s {
						[] => true,
						['p', 'o', 'r', 'u', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6445);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 6446);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 6447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1310);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3333);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3334);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1312);
			match s {
				[] => true,
				['p', 'a', 'i', 'd', s @ ..] => {
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
		['i', s @ ..] => super::t5::n1447(s, out),
		['l', s @ ..] => super::t5::n1448(s, out),
		['m', s @ ..] => super::t5::n1449(s, out),
		['n', s @ ..] => super::t5::n1450(s, out),
		['o', s @ ..] => {
			super::push(out, 1317);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t5::n1452(s, out),
		['r', s @ ..] => super::t5::n1453(s, out),
		['s', s @ ..] => super::t5::n1454(s, out),
		['t', s @ ..] => super::t5::n1455(s, out),
		['u', s @ ..] => super::t5::n1456(s, out),
		['v', s @ ..] => super::t5::n1457(s, out),
		['y', s @ ..] => super::t5::n1458(s, out),
		['z', s @ ..] => super::t5::n1459(s, out),
		_ => false,
	}
}

/// Words ending -inus.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 411);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1343);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 3408);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 1344);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6533);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3410);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 3411);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3413);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6535);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6536);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6537);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6538);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 10207);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10208);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 3414);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6540);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6541);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6542);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 10209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 3415);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6543);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 6544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3416);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6545);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6546);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6547);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 10210);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10211);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 10212);
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
						['m', s @ ..] => {
							super::push(out, 6548);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6549);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6550);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6551);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 6552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3417);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3418);
					match s {
						[] => true,
						['o', s @ ..] => {
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
				['u', s @ ..] => {
					super::push(out, 3419);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6553);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 3420);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6555);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3421);
					s.is_empty()
				}
				['g', 'a', 'r', 'a', 'm', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 6557);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 6558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3422);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6559);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6560);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6562);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 3423);
					s.is_empty()
				}
				['u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 3424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3425);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3426);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['c', 'u', s @ ..] => {
					super::push(out, 3428);
					match s {
						[] => true,
						['p', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6564);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1350);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3429);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6565);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6566);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6567);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 6568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t5::n4056(s, out),
				['g', s @ ..] => {
					super::push(out, 3431);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3432);
					match s {
						[] => true,
						['b', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6574);
							s.is_empty()
						}
						['l', s @ ..] => super::t5::n8884(s, out),
						['u', s @ ..] => super::t5::n8885(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t5::n4059(s, out),
				['o', s @ ..] => super::t5::n4060(s, out),
				['u', s @ ..] => super::t5::n4061(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t5::n1485(s, out),
		['n', s @ ..] => super::t5::n1486(s, out),
		['p', s @ ..] => super::t5::n1487(s, out),
		['r', s @ ..] => super::t5::n1488(s, out),
		['s', s @ ..] => super::t5::n1489(s, out),
		['t', s @ ..] => super::t5::n1490(s, out),
		['u', s @ ..] => super::t5::n1491(s, out),
		['v', s @ ..] => super::t5::n1492(s, out),
		['x', s @ ..] => super::t5::n1493(s, out),
		['y', s @ ..] => super::t5::n1494(s, out),
		_ => false,
	}
}

/// Words ending -mnus.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 412);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['t', 'i', s @ ..] => {
					super::push(out, 3479);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nnus.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 413);
	match s {
		[] => true,
		['i', 'c', 'n', 'o', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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

/// Words ending -onus.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 414);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1363);
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
		['c', 'o', 'r', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 1364);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 1365);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1366);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 3480);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 3481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3482);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6653);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 10245);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10246);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6654);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10247);
									match s {
										[] => true,
										['l', 'c', '\u{1}', s @ ..] => {
											super::push(out, 13495);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6655);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 10248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 6656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3483);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6657);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 1369);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnus.
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 415);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1370);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 3484);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3485);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 3486);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6659);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 10249);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 10250);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10251);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6660);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 10252);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 10253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3487);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6661);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6662);
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
		['o', s @ ..] => {
			super::push(out, 1371);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1372);
			match s {
				[] => true,
				['i', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6663);
							match s {
								[] => true,
								['u', 'q', 's', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 10254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'u', s @ ..] => {
					super::push(out, 3488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -unus.
pub(crate) fn n445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 416);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', 'r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'r', 'o', 'p', 'p', 'o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
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

/// The word acrocarpus.
pub(crate) fn n28786(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word macrocarpus.
pub(crate) fn n28787(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -erus.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 435);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1396);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 1397);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3502);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3503);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10257);
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3504);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 3506);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 3507);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6680);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10258);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 6681);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6682);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10260);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6683);
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 10262);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3508);
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
		['g', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6684);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6685);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1400);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 1401);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3509);
					match s {
						[] => true,
						['h', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'n', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1402);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1403);
			match s {
				[] => true,
				['o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3510);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 6688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1404);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1405);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3511);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6689);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3512);
					match s {
						[] => true,
						['c', 'i', 'r', 'o', 't', 'e', 'h', 'r', '\u{1}', s @ ..] => {
							super::push(out, 6691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['c', 'i', s @ ..] => {
											super::push(out, 13498);
											match s {
												[] => true,
												['m', '\u{1}', s @ ..] => {
													super::push(out, 15826);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 13499);
											match s {
												[] => true,
												['l', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 15827);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 3514);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 6692);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6693);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1406);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 1407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orus.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 1410);
			match s {
				[] => true,
				['e', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6694);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 10263);
									s.is_empty()
								}
								_ => false,
							}
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
				['i', s @ ..] => {
					super::push(out, 3515);
					match s {
						[] => true,
						['t', 'e', 'b', 'a', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1411);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'n', 'e', 'b', '\u{1}', s @ ..] => {
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
		['f', s @ ..] => {
			super::push(out, 1412);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1413);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1414);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3516);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['c', 'u', 'a', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['i', 'r', 'i', 'v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 10264);
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 13500);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 10265);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											super::push(out, 13501);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10266);
									match s {
										[] => true,
										['n', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 13502);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10267);
									match s {
										[] => true,
										['a', 'e', 'l', 'u', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 13503);
											s.is_empty()
										}
										['l', 'u', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 13504);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'n', 'e', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'a', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3517);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10268);
									match s {
										[] => true,
										['s', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1415);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 1416);
			match s {
				[] => true,
				['o', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		['p', s @ ..] => {
			super::push(out, 1417);
			match s {
				[] => true,
				['i', 'n', 'i', 'm', 'u', 'l', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 3518);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3519);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3520);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 1418);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 6697);
							match s {
								[] => true,
								['u', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 10269);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6698);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10270);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 13505);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6699);
							match s {
								[] => true,
								['e', 'v', 'a', 'd', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6700);
							match s {
								[] => true,
								['c', 'e', 's', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 10272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urus.
pub(crate) fn n470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1421);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1422);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3522);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3523);
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
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
		['d', s @ ..] => {
			super::push(out, 1423);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1424);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1425);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t5::n1568(s, out),
		['t', s @ ..] => super::t5::n1569(s, out),
		['x', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3547);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6825);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 10659);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 14224);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 16533);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n34758(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 16534);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n34759(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 14225);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 14226);
											match s {
												[] => true,
												['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 14227);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 14228);
											match s {
												[] => true,
												['n', 'e', 'g', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 10660);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
						['n', s @ ..] => {
							super::push(out, 6826);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
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
								['o', s @ ..] => {
									super::push(out, 10661);
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
				['i', s @ ..] => super::t5::n4215(s, out),
				['u', s @ ..] => super::t5::n4216(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nfisus.
pub(crate) fn n4245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3560);
	match s {
		[] => true,
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lisus.
pub(crate) fn n1587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1445);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3561);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3562);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nisus.
pub(crate) fn n1588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1446);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3563);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3564);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6831);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6832);
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
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3565);
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

/// Words ending -risus.
pub(crate) fn n1589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1447);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', '\u{1}', s @ ..] => {
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

/// Words ending -visus.
pub(crate) fn n1590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1448);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 3566);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 3567);
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
		['i', s @ ..] => {
			super::push(out, 3568);
			match s {
				[] => true,
				['d', s @ ..] => {
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
		['n', s @ ..] => {
			super::push(out, 3569);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3570);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 3571);
			match s {
				[] => true,
				['e', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lsus.
pub(crate) fn n475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1449);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1450);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', s @ ..] => {
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1451);
			match s {
				[] => true,
				['v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 3572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1452);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 3573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10672);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10673);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6834);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10674);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 10675);
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
							super::push(out, 6835);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6836);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 3575);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 3576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10676);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10677);
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
							super::push(out, 6839);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6840);
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

/// Words ending -nsus.
pub(crate) fn n476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1453);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
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
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1454);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 3577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6841);
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
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6842);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6843);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10678);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 10679);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 14235);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 14236);
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
									super::push(out, 10680);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 14237);
											match s {
												[] => true,
												['r', 't', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 14238);
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
				['d', s @ ..] => {
					super::push(out, 3578);
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
				['f', s @ ..] => {
					super::push(out, 3579);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6844);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['f', s @ ..] => super::t5::n9276(s, out),
						['n', s @ ..] => {
							super::push(out, 6846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => super::t5::n4280(s, out),
				['m', s @ ..] => super::t5::n4281(s, out),
				['p', s @ ..] => super::t5::n4282(s, out),
				['r', s @ ..] => super::t5::n4283(s, out),
				['s', s @ ..] => super::t5::n4284(s, out),
				['t', s @ ..] => super::t5::n4285(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1455);
			s.is_empty()
		}
		['o', s @ ..] => super::t5::n1598(s, out),
		['u', s @ ..] => super::t5::n1599(s, out),
		_ => false,
	}
}

/// Words ending -osus.
pub(crate) fn n477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 1458);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'l', 'g', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3589);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6856);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10689);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10690);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 6857);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6858);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10691);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 10692);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6859);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10694);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 10695);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 14243);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 14244);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3590);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3591);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 3592);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6861);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 3593);
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
		['d', s @ ..] => {
			super::push(out, 1460);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 3594);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3595);
					match s {
						[] => true,
						['o', 'r', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 3596);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1461);
			match s {
				[] => true,
				['b', 'a', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3598);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6864);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3599);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6866);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3600);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6868);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6869);
							match s {
								[] => true,
								['f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 14245);
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
					super::push(out, 3601);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 6870);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6871);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3602);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 3603);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 3604);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 6873);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 1463);
			s.is_empty()
		}
		['i', s @ ..] => super::t5::n1607(s, out),
		['l', s @ ..] => super::t5::n1608(s, out),
		['m', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 3624);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10755);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											super::push(out, 14266);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'q', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 10756);
									match s {
										[] => true,
										['t', 'u', 'c', 'a', '\u{1}', s @ ..] => {
											super::push(out, 14267);
											s.is_empty()
										}
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
					super::push(out, 3625);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 3626);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6923);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6924);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10757);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 10758);
									match s {
										[] => true,
										['a', 'l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3627);
					s.is_empty()
				}
				['o', s @ ..] => super::t5::n4333(s, out),
				['r', s @ ..] => super::t5::n4334(s, out),
				['u', s @ ..] => super::t5::n4335(s, out),
				['y', s @ ..] => super::t5::n4336(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t5::n1610(s, out),
		['p', s @ ..] => super::t5::n1611(s, out),
		['r', s @ ..] => super::t5::n1612(s, out),
		['t', s @ ..] => super::t5::n1613(s, out),
		['u', s @ ..] => super::t5::n1614(s, out),
		['v', s @ ..] => super::t5::n1615(s, out),
		['x', s @ ..] => super::t5::n1616(s, out),
		_ => false,
	}
}

/// Words ending -psus.
pub(crate) fn n478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 6968);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6969);
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
					super::push(out, 3664);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 3665);
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
				['o', s @ ..] => {
					super::push(out, 3666);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 3667);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rsus.
pub(crate) fn n479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1474);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
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
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10800);
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1475);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 3668);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 3669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6970);
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
						['m', s @ ..] => {
							super::push(out, 6971);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
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
				['p', s @ ..] => {
					super::push(out, 3670);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 6972);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 10801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6973);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10802);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 10803);
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
						['s', 'b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 3672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 6974);
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
						['b', s @ ..] => {
							super::push(out, 6975);
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
							super::push(out, 6976);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 6977);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10804);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 10805);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 10806);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 10807);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10808);
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
						['i', s @ ..] => {
							super::push(out, 6978);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 10810);
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
						['n', s @ ..] => {
							super::push(out, 6979);
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
							super::push(out, 6980);
							s.is_empty()
						}
						['r', s @ ..] => super::t5::n9500(s, out),
						['s', s @ ..] => {
							super::push(out, 6982);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t5::n1620(s, out),
		['u', s @ ..] => super::t5::n1621(s, out),
		_ => false,
	}
}

/// Words ending -ssus.
pub(crate) fn n480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 451);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
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
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 1479);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3679);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 6986);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'i', '\u{1}', s @ ..] => {
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
							super::push(out, 6987);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 10812);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10813);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6988);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['o', s @ ..] => {
							super::push(out, 6989);
							match s {
								[] => true,
								['r', s @ ..] => {
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
								_ => false,
							}
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6990);
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
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 3681);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6991);
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['f', s @ ..] => {
							super::push(out, 6992);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6993);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6994);
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
				['m', s @ ..] => super::t5::n4409(s, out),
				['p', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => super::t5::n4411(s, out),
				['s', s @ ..] => super::t5::n4412(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t5::n1624(s, out),
		['o', s @ ..] => super::t5::n1625(s, out),
		['u', s @ ..] => super::t5::n1626(s, out),
		_ => false,
	}
}

/// Words ending -usus.
pub(crate) fn n481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 1483);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
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
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 1484);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 3693);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
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
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1485);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 3694);
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
				['f', s @ ..] => {
					super::push(out, 3695);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
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
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 3696);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7018);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7019);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 3697);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 3698);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 1486);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 3699);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7020);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10836);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 10837);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 10838);
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
					super::push(out, 3700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 7021);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 7022);
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
					super::push(out, 3701);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1487);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 3702);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 3703);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => super::t5::n9609(s, out),
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7024);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'b', 'a', '\u{1}', s @ ..] => super::t5::n9612(s, out),
						['x', 'e', '\u{1}', s @ ..] => super::t5::n9613(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t5::n1637(s, out),
		_ => false,
	}
}

/// Words ending -ysus.
pub(crate) fn n482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	s.is_empty()
}

