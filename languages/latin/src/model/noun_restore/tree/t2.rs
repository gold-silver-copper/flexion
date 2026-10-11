use flexion::Path;

/// Words ending -pula.
pub(crate) fn n908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 903);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2553);
			match s {
				[] => true,
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2554);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6590);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6591);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8126);
									s.is_empty()
								}
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
			super::push(out, 2555);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2556);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rula.
pub(crate) fn n909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 904);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 2557);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2558);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 4659);
					s.is_empty()
				}
				['h', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 4660);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 4661);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sula.
pub(crate) fn n910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 905);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 2559);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4662);
					match s {
						[] => true,
						['n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 2560);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2561);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tula.
pub(crate) fn n911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 906);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2562);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 4663);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4664);
					match s {
						[] => true,
						['i', 'v', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6593);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2563);
			match s {
				[] => true,
				['n', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4665);
					s.is_empty()
				}
				['r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2564);
			match s {
				[] => true,
				['l', 'l', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2565);
			match s {
				[] => true,
				['a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 4668);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2566);
			match s {
				[] => true,
				['d', 'r', 'e', 'c', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2567);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 2568);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 2569);
			s.is_empty()
		}
		['x', s @ ..] => {
			super::push(out, 2570);
			match s {
				[] => true,
				['e', 's', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 4671);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vula.
pub(crate) fn n912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 907);
	match s {
		[] => true,
		['a', 'l', 'c', '\u{1}', s @ ..] => {
			super::push(out, 2571);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2572);
			s.is_empty()
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 2573);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xula.
pub(crate) fn n913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 908);
	s.is_empty()
}

/// Words ending -dium.
pub(crate) fn n1314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1300);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2956);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 2957);
			match s {
				[] => true,
				['u', 'q', 'n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 4860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2958);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4861);
					s.is_empty()
				}
				['r', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2959);
			match s {
				[] => true,
				['b', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4863);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4864);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6654);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 8147);
									match s {
										[] => true,
										['l', 'i', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 9010);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8148);
									match s {
										[] => true,
										['i', 'm', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9011);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8149);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 9012);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 9013);
											match s {
												[] => true,
												['a', 'p', '\u{1}', s @ ..] => {
													super::push(out, 9375);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 8150);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9014);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 9015);
											match s {
												[] => true,
												['a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9376);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 6655);
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6656);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'y', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4865);
					s.is_empty()
				}
				['m', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4866);
					s.is_empty()
				}
				['n', 'o', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4867);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4868);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 6658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4869);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4870);
					match s {
						[] => true,
						['s', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 4871);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4872);
					match s {
						[] => true,
						['p', 'i', s @ ..] => {
							super::push(out, 6660);
							match s {
								[] => true,
								['l', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 4873);
					match s {
						[] => true,
						['f', 'i', 't', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2961);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 4874);
					s.is_empty()
				}
				['l', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4875);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 4876);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2962);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 'i', 'v', 'a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6662);
							s.is_empty()
						}
						['m', 'i', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6663);
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2963);
			match s {
				[] => true,
				['a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4878);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4879);
					match s {
						[] => true,
						['i', 'd', s @ ..] => {
							super::push(out, 6665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4880);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 4881);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gium.
pub(crate) fn n1315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1301);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2964);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4882);
					s.is_empty()
				}
				['l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4883);
					s.is_empty()
				}
				['n', 'i', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 4884);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4885);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6667);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 6668);
							match s {
								[] => true,
								['f', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4886);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6670);
							s.is_empty()
						}
						['i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 4887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2965);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6672);
							match s {
								[] => true,
								['c', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 8155);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8156);
									match s {
										[] => true,
										['o', 'l', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9016);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'r', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 8157);
									s.is_empty()
								}
								['v', 'i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8158);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 4888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2966);
			match s {
				[] => true,
				['d', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4889);
					s.is_empty()
				}
				['l', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4890);
					s.is_empty()
				}
				['m', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4891);
					s.is_empty()
				}
				['r', 'f', 'i', 'r', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4892);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4893);
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6674);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2967);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2968);
			match s {
				[] => true,
				['g', 'a', 'd', 'e', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4895);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4896);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 6676);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6677);
							match s {
								[] => true,
								['r', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8159);
									s.is_empty()
								}
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
			super::push(out, 2969);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 4897);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2970);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 4898);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 4899);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rachium.
pub(crate) fn n6783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6680);
	match s {
		[] => true,
		['b', '\u{1}', s @ ..] => {
			super::push(out, 8160);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word hemistichium.
pub(crate) fn n4964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4901);
	s.is_empty()
}

/// The word archium.
pub(crate) fn n4965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4902);
	s.is_empty()
}

/// Words ending -ychium.
pub(crate) fn n4966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4903);
	s.is_empty()
}

/// Words ending -phium.
pub(crate) fn n3003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2972);
	match s {
		[] => true,
		['a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 4904);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -thium.
pub(crate) fn n3004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2973);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4905);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 6681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lium.
pub(crate) fn n1317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1303);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2974);
			match s {
				[] => true,
				['b', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 4907);
					s.is_empty()
				}
				['d', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 4908);
					s.is_empty()
				}
				['h', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 4909);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 4910);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4911);
					s.is_empty()
				}
				['r', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'y', 'c', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 2975);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2976);
			match s {
				[] => true,
				['b', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4913);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 4914);
					s.is_empty()
				}
				['t', 'n', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2977);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4916);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4917);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 4918);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 4919);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 6682);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4920);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 4921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2978);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4922);
					s.is_empty()
				}
				['y', 'r', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4923);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2979);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 4924);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6684);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 6685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 4925);
					match s {
						[] => true,
						['i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 4926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6687);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6688);
							match s {
								[] => true,
								['i', 'l', 'b', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 6689);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4927);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4928);
					match s {
						[] => true,
						['s', 'i', 'p', s @ ..] => {
							super::push(out, 6690);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8162);
									s.is_empty()
								}
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
			super::push(out, 2980);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 2981);
			match s {
				[] => true,
				['d', 'n', 'o', 'h', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 4929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mium.
pub(crate) fn n1318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1304);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2982);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 4930);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2983);
			match s {
				[] => true,
				['h', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4932);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2984);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4933);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6691);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6692);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4934);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4935);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 2985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nium.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1305);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2986);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 4936);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6693);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 6694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 4937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2987);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4938);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 4939);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 4940);
					s.is_empty()
				}
				['h', 'r', '\u{1}', s @ ..] => {
					super::push(out, 4941);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 6695);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2988);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4942);
					match s {
						[] => true,
						['c', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6697);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6698);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 8163);
									match s {
										[] => true,
										['a', 'v', '\u{1}', s @ ..] => {
											super::push(out, 9017);
											s.is_empty()
										}
										['c', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 9018);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6699);
							match s {
								[] => true,
								['n', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8164);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8165);
									match s {
										[] => true,
										['i', 't', '\u{1}', s @ ..] => {
											super::push(out, 9019);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 9020);
											s.is_empty()
										}
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
					super::push(out, 4943);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6700);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4944);
					match s {
						[] => true,
						['c', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 6702);
							s.is_empty()
						}
						['i', 'u', 'q', 'r', 'e', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 6703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4945);
					match s {
						[] => true,
						['a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6704);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 6705);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'l', 't', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6706);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 6707);
							s.is_empty()
						}
						['u', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 6708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 4946);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4947);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6709);
							match s {
								[] => true,
								['a', 't', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 6710);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2989);
			match s {
				[] => true,
				['e', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 4948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2990);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6711);
							match s {
								[] => true,
								['e', 'd', 'e', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 8168);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6712);
							s.is_empty()
						}
						['l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6713);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6714);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2991);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4949);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6715);
							s.is_empty()
						}
						['s', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 6716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 4950);
					match s {
						[] => true,
						['i', 't', 't', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 6717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 4951);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 6718);
							match s {
								[] => true,
								['e', 'l', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 8170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4952);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 4953);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6719);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 8171);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 6720);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 8172);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 8173);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											super::push(out, 9021);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8174);
									match s {
										[] => true,
										['t', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9022);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 's', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8175);
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 'u', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 4954);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 4955);
					match s {
						[] => true,
						['i', 'p', 'e', '\u{1}', s @ ..] => {
							super::push(out, 6722);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 6723);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 8177);
									s.is_empty()
								}
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
			super::push(out, 2992);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2993);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 4956);
					match s {
						[] => true,
						['i', 'n', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 4957);
					match s {
						[] => true,
						['r', 'o', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 6725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pium.
pub(crate) fn n1320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1306);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2994);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 4958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2995);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6726);
							match s {
								[] => true,
								['n', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6727);
							match s {
								[] => true,
								['i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8179);
									s.is_empty()
								}
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
			super::push(out, 2996);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2997);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 4959);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 8180);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9023);
											s.is_empty()
										}
										['t', 'e', 'n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9024);
											s.is_empty()
										}
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
					super::push(out, 4960);
					match s {
						[] => true,
						['t', 'o', 'i', 'l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2998);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2999);
			match s {
				[] => true,
				['s', 'r', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4961);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rium.
pub(crate) fn n1321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1307);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3000);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4962);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 4963);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6730);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 8181);
									s.is_empty()
								}
								['p', 's', '\u{1}', s @ ..] => {
									super::push(out, 8182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4964);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 6731);
							match s {
								[] => true,
								['g', 'i', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 6732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 4965);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4966);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 6733);
							match s {
								[] => true,
								['i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8184);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6734);
							match s {
								[] => true,
								['s', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4967);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6735);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									super::push(out, 8186);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6736);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6737);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6738);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8187);
									s.is_empty()
								}
								['i', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 8188);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8189);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9025);
											s.is_empty()
										}
										['r', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9026);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6739);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 8190);
									match s {
										[] => true,
										['a', 't', 'c', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9027);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 6740);
							match s {
								[] => true,
								['b', 'a', 'c', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 4968);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 6741);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 6742);
							s.is_empty()
						}
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6743);
							s.is_empty()
						}
						['u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 6744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4969);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 6745);
							s.is_empty()
						}
						['e', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 6746);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6747);
							match s {
								[] => true,
								['d', 'u', 't', 'e', 'l', 'a', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8193);
									s.is_empty()
								}
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8194);
									s.is_empty()
								}
								['p', 'm', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8195);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6748);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8196);
									match s {
										[] => true,
										['t', 'c', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 9028);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'i', 'v', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8197);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6749);
							s.is_empty()
						}
						['u', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 6750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4970);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 4971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6751);
							s.is_empty()
						}
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6752);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 6753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4972);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6754);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6755);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 8198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 6756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4973);
					match s {
						[] => true,
						['a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 6757);
							s.is_empty()
						}
						['n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6758);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 6759);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8199);
									s.is_empty()
								}
								['i', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8200);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 8201);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9029);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t2::n5040(s, out),
				['u', s @ ..] => {
					super::push(out, 4975);
					match s {
						[] => true,
						['t', s @ ..] => super::t2::n6869(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t2::n5042(s, out),
				_ => false,
			}
		}
		['b', s @ ..] => super::t2::n3032(s, out),
		['e', s @ ..] => super::t2::n3033(s, out),
		['i', s @ ..] => super::t2::n3034(s, out),
		['o', s @ ..] => super::t2::n3035(s, out),
		['p', s @ ..] => super::t2::n3036(s, out),
		['t', s @ ..] => super::t2::n3037(s, out),
		['u', s @ ..] => super::t2::n3038(s, out),
		['y', s @ ..] => super::t2::n3039(s, out),
		_ => false,
	}
}

/// Words ending -sium.
pub(crate) fn n1322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1308);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3009);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 5009);
					s.is_empty()
				}
				['l', 'p', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 5010);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3010);
			match s {
				[] => true,
				['h', 'c', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5012);
					s.is_empty()
				}
				['n', 'g', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tium.
pub(crate) fn n1323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1309);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3011);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5014);
					match s {
						[] => true,
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 6803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5015);
					match s {
						[] => true,
						['y', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5016);
					s.is_empty()
				}
				['r', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5017);
					s.is_empty()
				}
				['v', 'l', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3012);
			match s {
				[] => true,
				['n', 'i', 'c', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 5019);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5020);
					match s {
						[] => true,
						['n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 6805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3013);
			match s {
				[] => true,
				['n', 'h', 'c', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 5021);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5022);
					match s {
						[] => true,
						['p', 'u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 6806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3014);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5023);
					s.is_empty()
				}
				['d', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5024);
					s.is_empty()
				}
				['g', 'a', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5025);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5026);
					match s {
						[] => true,
						['i', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 5027);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5028);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8237);
									s.is_empty()
								}
								['l', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 8238);
									s.is_empty()
								}
								['u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8239);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 5029);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6808);
							s.is_empty()
						}
						['r', 'e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3015);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3016);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5030);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 6810);
							match s {
								[] => true,
								['o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5031);
					match s {
						[] => true,
						['p', 'n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 6811);
							s.is_empty()
						}
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'c', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 5032);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5033);
					s.is_empty()
				}
				['u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3017);
			s.is_empty()
		}
		['p', 'e', 's', 'n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 3018);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3019);
			match s {
				[] => true,
				['e', 't', 's', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 5035);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5036);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6813);
							s.is_empty()
						}
						['v', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 's', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 5037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3020);
			match s {
				[] => true,
				['o', '\u{1}', s @ ..] => {
					super::push(out, 5038);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3021);
			match s {
				[] => true,
				['l', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 5040);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5041);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uium.
pub(crate) fn n1324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1310);
	match s {
		[] => true,
		['q', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5042);
					s.is_empty()
				}
				['i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5043);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5044);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8241);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9042);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 8242);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 9043);
											match s {
												[] => true,
												['a', 'd', 'n', 'e', 'm', '\u{1}', s @ ..] => {
													super::push(out, 9381);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 9382);
													s.is_empty()
												}
												_ => false,
											}
										}
										['l', s @ ..] => {
											super::push(out, 9044);
											match s {
												[] => true,
												['o', 's', '\u{1}', s @ ..] => {
													super::push(out, 9383);
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 9045);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 9384);
													s.is_empty()
												}
												['e', 'i', 'l', 'a', '\u{1}', s @ ..] => {
													super::push(out, 9385);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 9046);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 9047);
											match s {
												[] => true,
												['i', 'r', '\u{1}', s @ ..] => {
													super::push(out, 9386);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 9048);
											match s {
												[] => true,
												['a', 'f', '\u{1}', s @ ..] => {
													super::push(out, 9387);
													s.is_empty()
												}
												['l', s @ ..] => {
													super::push(out, 9388);
													s.is_empty()
												}
												['u', 'n', 'i', 'm', '\u{1}', s @ ..] => {
													super::push(out, 9389);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											super::push(out, 9049);
											match s {
												[] => true,
												['a', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 9390);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 8243);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8244);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vium.
pub(crate) fn n1325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1311);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3022);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5045);
					s.is_empty()
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'l', 'e', 'd', 'n', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 3023);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3024);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5047);
					s.is_empty()
				}
				['l', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 5048);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5049);
					s.is_empty()
				}
				['v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3025);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3026);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5051);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 6815);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8245);
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8246);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6816);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 6817);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xium.
pub(crate) fn n1326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1312);
	s.is_empty()
}

/// Words ending -iolum.
pub(crate) fn n3087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3055);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5074);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6831);
					s.is_empty()
				}
				['i', 'v', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5075);
			match s {
				[] => true,
				['c', 'a', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5076);
			match s {
				[] => true,
				['a', 'r', 't', 'x', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6834);
					s.is_empty()
				}
				['o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5077);
			match s {
				[] => true,
				['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 6836);
					s.is_empty()
				}
				['o', 'm', 'i', 'r', 't', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6837);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5078);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6838);
					match s {
						[] => true,
						['m', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8254);
							s.is_empty()
						}
						['n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8255);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8256);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 's', 'a', 'n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6839);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6840);
					match s {
						[] => true,
						['s', 'r', 'e', 'v', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8257);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5079);
			match s {
				[] => true,
				['o', 'g', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 6842);
					s.is_empty()
				}
				['s', 'o', '\u{1}', s @ ..] => {
					super::push(out, 6843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', 'a', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 5080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulum.
pub(crate) fn n1335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1321);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3057);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3058);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5081);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 6844);
							match s {
								[] => true,
								['o', 'v', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6845);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8260);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8261);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6846);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 6847);
							match s {
								[] => true,
								['e', 'c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8262);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8263);
									match s {
										[] => true,
										['n', 'g', 'i', '\u{1}', s @ ..] => {
											super::push(out, 9051);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5082);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6848);
							match s {
								[] => true,
								['n', 'u', 'f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 6849);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6850);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 8266);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6851);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8267);
									match s {
										[] => true,
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9052);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3059);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5084);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 6852);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 6853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8268);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 8269);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 8270);
									s.is_empty()
								}
								['t', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6854);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8272);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 9053);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 8273);
									match s {
										[] => true,
										['u', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9054);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8274);
									match s {
										[] => true,
										['e', 'b', 'i', 'h', '\u{1}', s @ ..] => {
											super::push(out, 9055);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 6855);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8275);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 8276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5085);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 6857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5086);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 6858);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8277);
									match s {
										[] => true,
										['c', 'e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9056);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8278);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 6859);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 6860);
							match s {
								[] => true,
								['i', 'd', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 6861);
							match s {
								[] => true,
								['e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8280);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 6862);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 6863);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8281);
									match s {
										[] => true,
										['l', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 9057);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 9058);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 8282);
									match s {
										[] => true,
										['e', 'v', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9059);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 6864);
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8283);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8284);
									s.is_empty()
								}
								['r', s @ ..] => super::t2::n8424(s, out),
								['s', 'o', 'p', '\u{1}', s @ ..] => super::t2::n8425(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t2::n5153(s, out),
				['n', s @ ..] => super::t2::n5154(s, out),
				['o', s @ ..] => super::t2::n5155(s, out),
				['r', s @ ..] => super::t2::n5156(s, out),
				['s', s @ ..] => super::t2::n5157(s, out),
				['u', s @ ..] => super::t2::n5158(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t2::n3092(s, out),
		['n', s @ ..] => super::t2::n3093(s, out),
		['p', s @ ..] => super::t2::n3094(s, out),
		['s', s @ ..] => super::t2::n3095(s, out),
		['t', s @ ..] => super::t2::n3096(s, out),
		_ => false,
	}
}

/// The word flustrum.
pub(crate) fn n8442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8303);
	s.is_empty()
}

/// Words ending -utrum.
pub(crate) fn n3183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3150);
	s.is_empty()
}

/// Words ending -urum.
pub(crate) fn n1369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3151);
			s.is_empty()
		}
		['c', 's', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 3152);
			s.is_empty()
		}
		['t', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 3153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yrum.
pub(crate) fn n1370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1356);
	match s {
		[] => true,
		['t', 'u', 'b', '\u{1}', s @ ..] => {
			super::push(out, 3154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -netum.
pub(crate) fn n3222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3189);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5195);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5196);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6921);
					s.is_empty()
				}
				['m', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 6922);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6923);
					s.is_empty()
				}
				['t', 'o', 'p', 'm', 'u', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6924);
					s.is_empty()
				}
				['x', 'a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 6925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5197);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -retum.
pub(crate) fn n3225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3192);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5200);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5201);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5202);
			match s {
				[] => true,
				['n', 'g', 'a', 't', 's', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6927);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 5203);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5204);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6928);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uetum.
pub(crate) fn n3227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3194);
	match s {
		[] => true,
		['q', 's', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xetum.
pub(crate) fn n3229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3196);
	s.is_empty()
}

/// Words ending -itum.
pub(crate) fn n1384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1370);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3197);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3198);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 5208);
					match s {
						[] => true,
						['i', 'b', 'e', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3199);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 3200);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3201);
			match s {
				[] => true,
				['i', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3202);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3203);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5211);
					s.is_empty()
				}
				['o', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3204);
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 6930);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 8308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ltum.
pub(crate) fn n1385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1371);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3205);
			match s {
				[] => true,
				['b', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3206);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 5214);
					match s {
						[] => true,
						['n', 'o', 'c', 's', 'u', 't', 'a', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 6932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntum.
pub(crate) fn n1386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1372);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3207);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3208);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5215);
					match s {
						[] => true,
						['i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 6933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t2::n5285(s, out),
				['t', s @ ..] => {
					super::push(out, 5217);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5218);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 5219);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 6943);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 6944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otum.
pub(crate) fn n1387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1373);
	match s {
		[] => true,
		['b', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 3209);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3210);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptum.
pub(crate) fn n1388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1374);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3212);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5220);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 6945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3213);
			match s {
				[] => true,
				['r', 'c', 's', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtum.
pub(crate) fn n1389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1375);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3214);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 5223);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 6946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3215);
			match s {
				[] => true,
				['g', 'o', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stum.
pub(crate) fn n1390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1376);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3216);
			match s {
				[] => true,
				['l', 'p', 'o', 'r', 'o', 'l', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5225);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 5226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3217);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 5227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3218);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 5228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utum.
pub(crate) fn n1391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1377);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3219);
			s.is_empty()
		}
		['b', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 3220);
			s.is_empty()
		}
		['l', '\u{1}', s @ ..] => {
			super::push(out, 3221);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3222);
			match s {
				[] => true,
				['f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5229);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 5230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3223);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ytum.
pub(crate) fn n1392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1378);
	s.is_empty()
}

/// Words ending -ssio.
pub(crate) fn n1558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1541);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3395);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3396);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5333);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7014);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7015);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8371);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8372);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7016);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 5334);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5335);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 7019);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8373);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9122);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8374);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8375);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8376);
									s.is_empty()
								}
								['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 8377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 7020);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8378);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5336);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3397);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5337);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 5338);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7023);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7024);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 8379);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7025);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7026);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 7027);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7028);
							s.is_empty()
						}
						['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 7029);
							s.is_empty()
						}
						['u', 'n', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3398);
			match s {
				[] => true,
				['f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3399);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5340);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5341);
					match s {
						[] => true,
						['e', 'd', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 7031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -usio.
pub(crate) fn n1559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1542);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 3400);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5342);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5343);
					s.is_empty()
				}
				['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 5344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3401);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5345);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5346);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5347);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5348);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3402);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3403);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3404);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 5349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -atio.
pub(crate) fn n1560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1543);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3405);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 5350);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 7034);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7035);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5351);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5352);
					match s {
						[] => true,
						['u', 't', 's', s @ ..] => {
							super::push(out, 7036);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8381);
									s.is_empty()
								}
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
			super::push(out, 3406);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5353);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							super::push(out, 7037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 5354);
					match s {
						[] => true,
						['i', 's', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5355);
					match s {
						[] => true,
						['a', 'f', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7039);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7040);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9123);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 9124);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7041);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5356);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7042);
							match s {
								[] => true,
								['a', 'r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8382);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8383);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9125);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													super::push(out, 9405);
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9126);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 9127);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8384);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8385);
									match s {
										[] => true,
										['a', 'l', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9128);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 9129);
											match s {
												[] => true,
												['i', 'd', '\u{1}', s @ ..] => {
													super::push(out, 9406);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => super::t2::n7154(s, out),
						['l', s @ ..] => {
							super::push(out, 7044);
							match s {
								[] => true,
								['b', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8386);
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8387);
									s.is_empty()
								}
								['i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8388);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7045);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7046);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 8390);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 's', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7047);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7048);
							match s {
								[] => true,
								['a', 'v', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8391);
									s.is_empty()
								}
								['b', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8392);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 8393);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9139);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7049);
							match s {
								[] => true,
								['s', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8395);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t2::n5429(s, out),
				['o', s @ ..] => super::t2::n5430(s, out),
				['r', s @ ..] => super::t2::n5431(s, out),
				['s', s @ ..] => super::t2::n5432(s, out),
				['u', s @ ..] => super::t2::n5433(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t2::n3441(s, out),
		['e', s @ ..] => super::t2::n3442(s, out),
		['f', s @ ..] => super::t2::n3443(s, out),
		['g', s @ ..] => super::t2::n3444(s, out),
		['h', s @ ..] => super::t2::n3445(s, out),
		['i', s @ ..] => super::t2::n3446(s, out),
		['l', s @ ..] => super::t2::n3447(s, out),
		['m', s @ ..] => super::t2::n3448(s, out),
		['n', s @ ..] => super::t2::n3449(s, out),
		['p', s @ ..] => super::t2::n3450(s, out),
		['r', s @ ..] => super::t2::n3451(s, out),
		['s', s @ ..] => super::t2::n3452(s, out),
		['t', s @ ..] => super::t2::n3453(s, out),
		['u', s @ ..] => super::t2::n3454(s, out),
		['v', s @ ..] => super::t2::n3455(s, out),
		['x', s @ ..] => super::t2::n3456(s, out),
		['z', s @ ..] => super::t2::n3457(s, out),
		_ => false,
	}
}

/// Words ending -spectio.
pub(crate) fn n7390(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 8580);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 9213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 8581);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rectio.
pub(crate) fn n5555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5482);
	match s {
		[] => true,
		['e', '\u{1}', s @ ..] => {
			super::push(out, 7275);
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 7276);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7277);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 8582);
					s.is_empty()
				}
				['u', 's', 'n', s @ ..] => {
					super::push(out, 8583);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tectio.
pub(crate) fn n5557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5484);
	match s {
		[] => true,
		['e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 7279);
			s.is_empty()
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7280);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vectio.
pub(crate) fn n5558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5485);
	match s {
		[] => true,
		['a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 7281);
			s.is_empty()
		}
		['e', '\u{1}', s @ ..] => {
			super::push(out, 7282);
			s.is_empty()
		}
		['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 7283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ictio.
pub(crate) fn n3460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3426);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5486);
			match s {
				[] => true,
				['a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7284);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7285);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7286);
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7287);
					s.is_empty()
				}
				['s', 'i', 'r', 'u', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5487);
			match s {
				[] => true,
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5488);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7290);
					match s {
						[] => true,
						['r', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7291);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8586);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 5489);
			s.is_empty()
		}
		['p', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5490);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5491);
			match s {
				[] => true,
				['t', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 5492);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 7293);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nctio.
pub(crate) fn n3461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3427);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5493);
			match s {
				[] => true,
				['t', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7294);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5494);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7295);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7296);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -octio.
pub(crate) fn n3462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3428);
	match s {
		[] => true,
		['c', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uctio.
pub(crate) fn n3463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3429);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5496);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 7297);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 7298);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7299);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 8589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7300);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7301);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7302);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5497);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5498);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8590);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 9214);
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9215);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8591);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5499);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -etio.
pub(crate) fn n1562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1545);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3430);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5500);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 5501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3431);
			match s {
				[] => true,
				['c', 'e', s @ ..] => {
					super::push(out, 5502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -itio.
pub(crate) fn n1563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1546);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3432);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5503);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 7304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', 'r', 'e', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3433);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3434);
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 5505);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 5506);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5507);
					match s {
						[] => true,
						['b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 7305);
							s.is_empty()
						}
						['o', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 7306);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 7307);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 7308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5508);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5509);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7310);
							s.is_empty()
						}
						['t', 's', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7311);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5510);
					match s {
						[] => true,
						['e', 'p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5511);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7313);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8593);
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 8594);
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7314);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3435);
			match s {
				[] => true,
				['r', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3436);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5513);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'u', 'b', 'e', '\u{1}', s @ ..] => {
					super::push(out, 5514);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5515);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 7316);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8596);
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8597);
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8598);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 7317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3437);
			match s {
				[] => true,
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3438);
			match s {
				[] => true,
				['a', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5517);
					s.is_empty()
				}
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5518);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 5519);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 7318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5520);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 7319);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8599);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9217);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8600);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 7320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5521);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 's', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 3439);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3440);
			match s {
				[] => true,
				['a', 'p', 'p', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5523);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5524);
					match s {
						[] => true,
						['p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 5525);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5526);
					match s {
						[] => true,
						['e', 't', '\u{1}', s @ ..] => {
							super::push(out, 7322);
							s.is_empty()
						}
						['u', 'g', 'i', 'l', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 5527);
					match s {
						[] => true,
						['u', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5528);
					match s {
						[] => true,
						['g', 'i', 'l', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3441);
			match s {
				[] => true,
				['e', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 5529);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5530);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8601);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8602);
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5531);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5532);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8604);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8605);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9218);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8606);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8607);
									s.is_empty()
								}
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
			super::push(out, 3442);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5533);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5534);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltio.
pub(crate) fn n1564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1547);
	s.is_empty()
}

/// Words ending -ntio.
pub(crate) fn n1565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1548);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3444);
			match s {
				[] => true,
				['t', 'e', s @ ..] => {
					super::push(out, 5536);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 7326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 3445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -otio.
pub(crate) fn n1566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1549);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 3446);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 5537);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 5538);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3447);
			s.is_empty()
		}
		['v', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 3448);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptio.
pub(crate) fn n1567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1550);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3449);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3450);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5540);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7328);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8608);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5541);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 7329);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3451);
			match s {
				[] => true,
				['r', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7331);
							match s {
								[] => true,
								['r', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7332);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7333);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8610);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7334);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7335);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3452);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5542);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7337);
							s.is_empty()
						}
						['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5543);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3453);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3454);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 7340);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7341);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtio.
pub(crate) fn n1568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1551);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3455);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 5544);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5545);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7343);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3456);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 5546);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5547);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stio.
pub(crate) fn n1569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1552);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3457);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3458);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 7346);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3459);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5549);
					s.is_empty()
				}
				['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3460);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utio.
pub(crate) fn n1570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1553);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3461);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3462);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3463);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5551);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5552);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8611);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9219);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 8612);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8613);
									s.is_empty()
								}
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
			super::push(out, 3464);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7349);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7350);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 8614);
									s.is_empty()
								}
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
			super::push(out, 3465);
			match s {
				[] => true,
				['i', 'm', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7351);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7352);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3466);
			match s {
				[] => true,
				['i', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7354);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8615);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7355);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7356);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3467);
			match s {
				[] => true,
				['q', 'e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5553);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xtio.
pub(crate) fn n1571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1554);
	s.is_empty()
}

/// Words ending -ator.
pub(crate) fn n1684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1666);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3541);
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 5594);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 5595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3542);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5596);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 7372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 5597);
					match s {
						[] => true,
						['r', 'p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5598);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 7374);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8619);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8620);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9220);
											match s {
												[] => true,
												['p', '\u{1}', s @ ..] => {
													super::push(out, 9441);
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9221);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 9222);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 8621);
									s.is_empty()
								}
								['u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 7375);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9223);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 9224);
											match s {
												[] => true,
												['n', 'e', 's', '\u{1}', s @ ..] => {
													super::push(out, 9442);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 9225);
											match s {
												[] => true,
												['s', 'u', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9443);
													s.is_empty()
												}
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
							super::push(out, 7376);
							match s {
								[] => true,
								['b', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8623);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 8624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7377);
							match s {
								[] => true,
								['u', 'm', 'm', 'o', 'c', 'x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 8625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7378);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8626);
									s.is_empty()
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
				['o', s @ ..] => {
					super::push(out, 5600);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8627);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9226);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5601);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5602);
					match s {
						[] => true,
						['d', 'e', '\u{1}', s @ ..] => {
							super::push(out, 7379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3543);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5603);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5604);
					s.is_empty()
				}
				['i', 'l', 'o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5605);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5606);
					match s {
						[] => true,
						['e', s @ ..] => {
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
				['u', s @ ..] => {
					super::push(out, 5607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3544);
			match s {
				[] => true,
				['c', 'u', 'd', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5608);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 5609);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5610);
					match s {
						[] => true,
						['e', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7383);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5611);
					match s {
						[] => true,
						['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3545);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5612);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 5613);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 7385);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5614);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 7387);
							match s {
								[] => true,
								['i', 'l', 'i', 't', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8629);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8630);
									match s {
										[] => true,
										['n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 9227);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5615);
					match s {
						[] => true,
						['u', 'v', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7389);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5616);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8631);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8632);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8633);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5617);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 7390);
							s.is_empty()
						}
						['r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t2::n3584(s, out),
		['l', s @ ..] => super::t2::n3585(s, out),
		['m', s @ ..] => {
			super::push(out, 3548);
			match s {
				[] => true,
				['a', s @ ..] => super::t2::n5720(s, out),
				['g', 'e', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 5639);
					s.is_empty()
				}
				['i', s @ ..] => super::t2::n5722(s, out),
				['m', 'u', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5641);
					s.is_empty()
				}
				['r', s @ ..] => super::t2::n5724(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t2::n3587(s, out),
		['p', s @ ..] => super::t2::n3588(s, out),
		['r', s @ ..] => super::t2::n3589(s, out),
		['s', s @ ..] => super::t2::n3590(s, out),
		['t', s @ ..] => super::t2::n3591(s, out),
		['u', s @ ..] => super::t2::n3592(s, out),
		['v', s @ ..] => super::t2::n3593(s, out),
		['z', s @ ..] => super::t2::n3594(s, out),
		_ => false,
	}
}

/// Words ending -ditor.
pub(crate) fn n3606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3568);
	match s {
		[] => true,
		['a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 5708);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5709);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5710);
			match s {
				[] => true,
				['a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7489);
					s.is_empty()
				}
				['e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7490);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5711);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5712);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7492);
					s.is_empty()
				}
				['r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7493);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gitor.
pub(crate) fn n3607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3569);
	match s {
		[] => true,
		['r', 'a', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5713);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5714);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 7494);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 7495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nitor.
pub(crate) fn n3609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3571);
	match s {
		[] => true,
		['a', 'i', '\u{1}', s @ ..] => {
			super::push(out, 5715);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5716);
			match s {
				[] => true,
				['g', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5717);
			match s {
				[] => true,
				['i', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5718);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 7498);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 7499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5719);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5720);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word cupitor.
pub(crate) fn n3610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3572);
	s.is_empty()
}

/// Words ending -ritor.
pub(crate) fn n3611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3573);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5721);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5722);
			match s {
				[] => true,
				['u', 'g', 'i', 'l', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sitor.
pub(crate) fn n3612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3574);
	match s {
		[] => true,
		['e', 'a', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 5723);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5724);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5725);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5726);
			s.is_empty()
		}
		['r', 'e', 'v', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5727);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -titor.
pub(crate) fn n3613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3575);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5728);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5729);
			match s {
				[] => true,
				['o', 's', '\u{1}', s @ ..] => {
					super::push(out, 7501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 5730);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vitor.
pub(crate) fn n3614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3576);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5731);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltor.
pub(crate) fn n1688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1670);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5732);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5733);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7502);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8678);
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7503);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntor.
pub(crate) fn n1689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1671);
	s.is_empty()
}

/// Words ending -otor.
pub(crate) fn n1690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1672);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 3577);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3578);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptor.
pub(crate) fn n1691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1673);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3579);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3580);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 5735);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7504);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3581);
			match s {
				[] => true,
				['r', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 7505);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7506);
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7507);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3582);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7509);
							s.is_empty()
						}
						['t', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3583);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3584);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtor.
pub(crate) fn n1692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1674);
	match s {
		[] => true,
		['e', 's', s @ ..] => {
			super::push(out, 3585);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5738);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stor.
pub(crate) fn n1693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1675);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3586);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 5739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3587);
			match s {
				[] => true,
				['m', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 3588);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3589);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utor.
pub(crate) fn n1694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1676);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3590);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 3591);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3592);
			match s {
				[] => true,
				['o', 'l', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 5741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3593);
			match s {
				[] => true,
				['i', 't', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 5742);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -itas.
pub(crate) fn n1730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1711);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3616);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3617);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5751);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7513);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 7514);
							match s {
								[] => true,
								['u', 'n', '\u{1}', s @ ..] => {
									super::push(out, 8682);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7515);
							match s {
								[] => true,
								['e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7516);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5752);
					match s {
						[] => true,
						['d', 'n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7517);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7518);
							match s {
								[] => true,
								['b', 'u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8684);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 8685);
									match s {
										[] => true,
										['f', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 9239);
											s.is_empty()
										}
										['o', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9240);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7519);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 7520);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7521);
							match s {
								[] => true,
								['b', 'u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8686);
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8687);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8688);
									match s {
										[] => true,
										['c', 'e', 'l', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9241);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7522);
							match s {
								[] => true,
								['s', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 8689);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5753);
					match s {
						[] => true,
						['l', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7523);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 5754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3618);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 5755);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 7525);
							match s {
								[] => true,
								['e', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8690);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 5756);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7526);
							match s {
								[] => true,
								['u', 'l', '\u{1}', s @ ..] => {
									super::push(out, 8691);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 7527);
							match s {
								[] => true,
								['i', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8692);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7528);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 7529);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9242);
											s.is_empty()
										}
										['h', '\u{1}', s @ ..] => {
											super::push(out, 9243);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 7530);
							match s {
								[] => true,
								['i', 's', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7531);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5757);
					match s {
						[] => true,
						['u', 'c', s @ ..] => {
							super::push(out, 7532);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8695);
									match s {
										[] => true,
										['f', 'n', 'i', '\u{1}', s @ ..] => {
											super::push(out, 9244);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 8696);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5758);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5759);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3619);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 5761);
					match s {
						[] => true,
						['u', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 5762);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7534);
							match s {
								[] => true,
								['t', 'n', 'o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 8697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 7535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 5763);
					match s {
						[] => true,
						['t', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t2::n3660(s, out),
		['m', s @ ..] => {
			super::push(out, 3621);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5771);
					s.is_empty()
				}
				['e', 'r', 't', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 5772);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 5773);
					match s {
						[] => true,
						['l', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7561);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5774);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 5775);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7563);
							match s {
								[] => true,
								['f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8729);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 7564);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 8730);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t2::n3662(s, out),
		['r', s @ ..] => super::t2::n3663(s, out),
		['s', s @ ..] => super::t2::n3664(s, out),
		['t', s @ ..] => {
			super::push(out, 3625);
			match s {
				[] => true,
				['a', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 5799);
					s.is_empty()
				}
				['c', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5800);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5801);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 5802);
					match s {
						[] => true,
						['a', s @ ..] => super::t2::n7734(s, out),
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 7611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t3::n3666(s, out),
		['v', s @ ..] => super::t3::n3667(s, out),
		['x', s @ ..] => super::t3::n3668(s, out),
		_ => false,
	}
}

/// Words ending -rius.
pub(crate) fn n2031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2012);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4030);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 4031);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6038);
					match s {
						[] => true,
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 7722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6039);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7723);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7724);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 8774);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 8775);
									match s {
										[] => true,
										['u', 'e', 't', 'y', 'h', 'p', 'm', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9288);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 8776);
									match s {
										[] => true,
										['a', 'l', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9289);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 7725);
							s.is_empty()
						}
						['u', 't', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 7726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 6040);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', 'r', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8777);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 8778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6041);
					match s {
						[] => true,
						['s', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7727);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6042);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7728);
							match s {
								[] => true,
								['l', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8779);
									s.is_empty()
								}
								['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 8780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 6043);
					s.is_empty()
				}
				['i', s @ ..] => super::t3::n6137(s, out),
				['l', s @ ..] => super::t3::n6138(s, out),
				['m', s @ ..] => {
					super::push(out, 6046);
					match s {
						[] => true,
						['g', 'e', 'm', 's', '\u{1}', s @ ..] => {
							super::push(out, 7746);
							s.is_empty()
						}
						['o', 'l', 'p', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7747);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7748);
							match s {
								[] => true,
								['i', 'f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t3::n6140(s, out),
				['p', s @ ..] => {
					super::push(out, 6048);
					match s {
						[] => true,
						['o', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 7759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t3::n6142(s, out),
				['s', s @ ..] => super::t3::n6143(s, out),
				['t', s @ ..] => super::t3::n6144(s, out),
				['u', s @ ..] => super::t3::n6145(s, out),
				['v', s @ ..] => super::t3::n6146(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t3::n4078(s, out),
		['o', s @ ..] => super::t3::n4079(s, out),
		['u', s @ ..] => super::t3::n4080(s, out),
		['y', s @ ..] => super::t3::n4081(s, out),
		_ => false,
	}
}

/// The word berillus.
pub(crate) fn n7927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7801);
	s.is_empty()
}

/// Words ending -tillus.
pub(crate) fn n6184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6090);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7802);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 8848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7803);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 8849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -villus.
pub(crate) fn n6185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6091);
	match s {
		[] => true,
		['l', 'u', 'p', '\u{1}', s @ ..] => {
			super::push(out, 7805);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xillus.
pub(crate) fn n6186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6092);
	s.is_empty()
}

/// Words ending -ullus.
pub(crate) fn n4118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4072);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 6093);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7806);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yllus.
pub(crate) fn n4119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4073);
	match s {
		[] => true,
		['r', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 6094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olus.
pub(crate) fn n2040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2021);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4074);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4075);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6095);
					match s {
						[] => true,
						['a', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6096);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4076);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6097);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6098);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6099);
					s.is_empty()
				}
				['m', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6100);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6101);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8850);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 8851);
									s.is_empty()
								}
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
			super::push(out, 4077);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 4078);
			match s {
				[] => true,
				['i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6102);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ulus.
pub(crate) fn n2041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2022);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4079);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4080);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6103);
					match s {
						[] => true,
						['c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7809);
							s.is_empty()
						}
						['t', 'e', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6104);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 7811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 6105);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7812);
							match s {
								[] => true,
								['i', 'n', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8852);
									s.is_empty()
								}
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
			super::push(out, 4081);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6106);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 6107);
					match s {
						[] => true,
						['r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6108);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6109);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7814);
							match s {
								[] => true,
								['a', 'r', 'o', 'h', 't', '\u{1}', s @ ..] => {
									super::push(out, 8853);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 8854);
									s.is_empty()
								}
								['u', 'd', '\u{1}', s @ ..] => {
									super::push(out, 8855);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 7815);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7816);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8856);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 8857);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 7817);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8858);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 7818);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8859);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											super::push(out, 9314);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 8860);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 9315);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 8861);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 8862);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 9316);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 7819);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7820);
							match s {
								[] => true,
								['n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 7821);
							match s {
								[] => true,
								['c', 'u', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 8864);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 7822);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 7823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6110);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6111);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 8865);
									match s {
										[] => true,
										['a', 'b', 'i', 'l', '\u{1}', s @ ..] => {
											super::push(out, 9317);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 9318);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 8866);
									s.is_empty()
								}
								['h', 'p', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 8867);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 8868);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 8869);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 8870);
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 9319);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 9320);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 8871);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 8872);
									match s {
										[] => true,
										['i', 't', '\u{1}', s @ ..] => {
											super::push(out, 9321);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 9322);
											s.is_empty()
										}
										['u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9323);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 8873);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 8874);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6112);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6113);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7824);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8875);
									match s {
										[] => true,
										['n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 9324);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 8876);
									match s {
										[] => true,
										['a', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 7825);
							match s {
								[] => true,
								['c', 'i', 'm', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t3::n6208(s, out),
				['u', s @ ..] => super::t3::n6209(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t3::n4128(s, out),
		['g', s @ ..] => super::t3::n4129(s, out),
		['i', s @ ..] => super::t3::n4130(s, out),
		['l', s @ ..] => {
			super::push(out, 4085);
			s.is_empty()
		}
		['m', s @ ..] => super::t3::n4132(s, out),
		['n', s @ ..] => super::t3::n4133(s, out),
		['p', s @ ..] => super::t3::n4134(s, out),
		['r', s @ ..] => super::t3::n4135(s, out),
		['s', s @ ..] => super::t3::n4136(s, out),
		['t', s @ ..] => super::t3::n4137(s, out),
		['v', s @ ..] => super::t3::n4138(s, out),
		_ => false,
	}
}

/// Words ending -ylus.
pub(crate) fn n2042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2023);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4093);
			s.is_empty()
		}
		['h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4094);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4095);
			match s {
				[] => true,
				['o', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -linus.
pub(crate) fn n4200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4154);
	match s {
		[] => true,
		['c', 'i', 'r', 't', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6191);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -minus.
pub(crate) fn n4201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4155);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6192);
			match s {
				[] => true,
				['c', 'y', 's', '\u{1}', s @ ..] => {
					super::push(out, 7867);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6193);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pinus.
pub(crate) fn n4203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4157);
	match s {
		[] => true,
		['m', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6195);
			s.is_empty()
		}
		['r', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6196);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rinus.
pub(crate) fn n4204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4158);
	match s {
		[] => true,
		['a', 'm', 's', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 6197);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6198);
			match s {
				[] => true,
				['o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7868);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6199);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sinus.
pub(crate) fn n4205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4159);
	match s {
		[] => true,
		['s', 'a', 's', 's', 'a', '\u{1}', s @ ..] => {
			super::push(out, 6200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tinus.
pub(crate) fn n4206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4160);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6201);
			match s {
				[] => true,
				['l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6202);
			match s {
				[] => true,
				['e', 'r', 'o', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6203);
			s.is_empty()
		}
		['r', 'e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 6204);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6205);
			s.is_empty()
		}
		['y', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vinus.
pub(crate) fn n4207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4161);
	s.is_empty()
}

/// Words ending -xinus.
pub(crate) fn n4208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4162);
	match s {
		[] => true,
		['a', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 6207);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6208);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 7871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lnus.
pub(crate) fn n2058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2039);
	s.is_empty()
}

/// Words ending -mnus.
pub(crate) fn n2059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2040);
	s.is_empty()
}

/// Words ending -nnus.
pub(crate) fn n2060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2041);
	s.is_empty()
}

/// Words ending -onus.
pub(crate) fn n2061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2042);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4163);
			match s {
				[] => true,
				['a', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6209);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4164);
			match s {
				[] => true,
				['r', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 6210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4165);
			match s {
				[] => true,
				['m', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4166);
			s.is_empty()
		}
		['r', 't', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4167);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4168);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnus.
pub(crate) fn n2062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2043);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4169);
			match s {
				[] => true,
				['f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 6212);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6213);
					match s {
						[] => true,
						['i', 'l', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 7872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -unus.
pub(crate) fn n2063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2044);
	match s {
		[] => true,
		['a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 4171);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iatus.
pub(crate) fn n4297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4248);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6266);
			match s {
				[] => true,
				['i', 'v', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'e', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6267);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6268);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7902);
					match s {
						[] => true,
						['n', 'i', 'd', 'r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8901);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8902);
							s.is_empty()
						}
						['u', 'q', 'i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8903);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 't', 'c', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6269);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -latus.
pub(crate) fn n4298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4249);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6270);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 6271);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 7904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6272);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 6273);
			match s {
				[] => true,
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
			super::push(out, 6274);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6275);
			s.is_empty()
		}
		['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 6276);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6277);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7906);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7907);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7908);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8905);
							match s {
								[] => true,
								['o', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -imatus.
pub(crate) fn n6376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6279);
	match s {
		[] => true,
		['n', 'a', '\u{1}', s @ ..] => {
			super::push(out, 7910);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word diplomatus.
pub(crate) fn n6377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6280);
	s.is_empty()
}

/// Words ending -natus.
pub(crate) fn n4300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4251);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6281);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 7911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6282);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					super::push(out, 7912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6283);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6284);
			match s {
				[] => true,
				['b', 'u', 'c', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7913);
					s.is_empty()
				}
				['l', 'i', 'u', 'q', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7914);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7915);
					match s {
						[] => true,
						['a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6285);
			match s {
				[] => true,
				['i', 'r', 'u', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7917);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7918);
					match s {
						[] => true,
						['t', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'o', '\u{1}', s @ ..] => {
			super::push(out, 6286);
			s.is_empty()
		}
		['u', 'b', 'i', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 6287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -patus.
pub(crate) fn n4301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4252);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6288);
			s.is_empty()
		}
		['i', 'c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ratus.
pub(crate) fn n4302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4253);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6290);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 6291);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6292);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7919);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6293);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6294);
			match s {
				[] => true,
				['p', 's', '\u{1}', s @ ..] => {
					super::push(out, 7921);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7922);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8908);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 9335);
									match s {
										[] => true,
										['i', 't', 'n', 'i', 'g', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 9468);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 't', 'n', 'i', 'g', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 8909);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8910);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9336);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 9337);
									match s {
										[] => true,
										['c', 'e', 'd', 'n', s @ ..] => {
											super::push(out, 9469);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 9338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8911);
							s.is_empty()
						}
						['x', s @ ..] => {
							super::push(out, 8912);
							match s {
								[] => true,
								['e', 's', 'i', 't', 'n', 'i', 'g', 'i', 'v', '\u{1}', s @ ..] => {
									super::push(out, 9339);
									s.is_empty()
								}
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
			super::push(out, 6295);
			match s {
				[] => true,
				['p', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7923);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7924);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									super::push(out, 9340);
									s.is_empty()
								}
								['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 6296);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6297);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7925);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7926);
					match s {
						[] => true,
						['u', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8913);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6298);
			match s {
				[] => true,
				['i', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7927);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -satus.
pub(crate) fn n4303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4254);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6299);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6300);
			match s {
				[] => true,
				['i', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7928);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tatus.
pub(crate) fn n4304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4255);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6301);
			match s {
				[] => true,
				['g', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7929);
					s.is_empty()
				}
				['r', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 7930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'p', 'e', 'n', '\u{1}', s @ ..] => {
			super::push(out, 6302);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6303);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6304);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vatus.
pub(crate) fn n4305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4256);
	match s {
		[] => true,
		['i', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6305);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word cyatus.
pub(crate) fn n4306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4257);
	s.is_empty()
}

/// Words ending -ctus.
pub(crate) fn n2104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2085);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4258);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6306);
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6307);
					s.is_empty()
				}
				['d', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6308);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 6309);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6310);
					s.is_empty()
				}
				['p', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 6311);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6312);
					match s {
						[] => true,
						['f', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7932);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 7933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4259);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 6314);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 7934);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 8914);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 7935);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 7936);
							s.is_empty()
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6315);
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 7938);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 7939);
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7941);
							s.is_empty()
						}
						['a', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7942);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 7943);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7944);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7945);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 8915);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 7946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 6317);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6318);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 8916);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9342);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8917);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
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
		['i', s @ ..] => {
			super::push(out, 4260);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6319);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6320);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 7947);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 7948);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8919);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6321);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4261);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6323);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6324);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 7949);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8920);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 7950);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6325);
					match s {
						[] => true,
						['p', 'a', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7951);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4262);
			match s {
				[] => true,
				['a', 'f', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 6326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4263);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 6327);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6328);
					match s {
						[] => true,
						['i', 'v', 'o', '\u{1}', s @ ..] => {
							super::push(out, 7952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6329);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6330);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -etus.
pub(crate) fn n2105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2086);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4264);
			match s {
				[] => true,
				['i', 'l', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', '\u{1}', s @ ..] => {
			super::push(out, 4265);
			s.is_empty()
		}
		['f', '\u{1}', s @ ..] => {
			super::push(out, 4266);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 4267);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6333);
					match s {
						[] => true,
						['c', 's', 'o', 'r', 'u', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6334);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 7954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4268);
			s.is_empty()
		}
		['p', 'a', 'r', 't', '\u{1}', s @ ..] => {
			super::push(out, 4269);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4270);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itus.
pub(crate) fn n2106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2087);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4271);
			match s {
				[] => true,
				['u', 'c', 'e', s @ ..] => {
					super::push(out, 6335);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 7955);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4272);
			match s {
				[] => true,
				['o', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6336);
					s.is_empty()
				}
				['r', 'e', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 6337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4273);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6338);
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6339);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7957);
							s.is_empty()
						}
						['p', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 7958);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 7959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 6341);
					s.is_empty()
				}
				['r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 6342);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6343);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7960);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 7961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', 'f', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4274);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4275);
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 6344);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4276);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6346);
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 7962);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'h', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6347);
					s.is_empty()
				}
				['o', 's', 'y', 'r', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4277);
			match s {
				[] => true,
				['a', 't', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6349);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4278);
			match s {
				[] => true,
				['e', 'g', 'i', 'l', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 6351);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6352);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4279);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4280);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4281);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6354);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6355);
					match s {
						[] => true,
						['p', 's', 'i', 'g', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 7964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 6356);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6357);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6358);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4282);
			match s {
				[] => true,
				['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6360);
					s.is_empty()
				}
				['e', 'a', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 6361);
					s.is_empty()
				}
				['i', 'u', 'q', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6362);
					s.is_empty()
				}
				['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6363);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6364);
					s.is_empty()
				}
				['s', 'e', 'c', 'r', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4283);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6366);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4284);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 4285);
			match s {
				[] => true,
				['r', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 6367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ltus.
pub(crate) fn n2107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2088);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4286);
			s.is_empty()
		}
		['u', 's', s @ ..] => {
			super::push(out, 4287);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntus.
pub(crate) fn n2108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2089);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4288);
			s.is_empty()
		}
		['e', 'v', s @ ..] => {
			super::push(out, 4289);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => {
					super::push(out, 6369);
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6370);
					s.is_empty()
				}
				['u', 'i', '\u{1}', s @ ..] => {
					super::push(out, 6371);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otus.
pub(crate) fn n2109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2090);
	match s {
		[] => true,
		['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ptus.
pub(crate) fn n2110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2091);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4291);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4292);
			s.is_empty()
		}
		['i', 'r', 'c', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4293);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4294);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6372);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 4295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtus.
pub(crate) fn n2111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2092);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4296);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4297);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6374);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 6375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 4298);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4299);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 's', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7966);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7967);
							match s {
								[] => true,
								['r', 'e', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 4300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stus.
pub(crate) fn n2112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2093);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4301);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 6376);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4302);
			match s {
				[] => true,
				['p', 'm', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 6378);
					s.is_empty()
				}
				['s', 'p', 'm', 'i', 'l', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4303);
			match s {
				[] => true,
				['h', 't', 'e', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 6380);
					s.is_empty()
				}
				['r', 'h', 'c', 'o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4304);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 4305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttus.
pub(crate) fn n2113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2094);
	s.is_empty()
}

/// Words ending -utus.
pub(crate) fn n2114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2095);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4306);
			match s {
				[] => true,
				['i', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6382);
					s.is_empty()
				}
				['r', 'o', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 6383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'a', 'm', 'a', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4307);
			s.is_empty()
		}
		['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4308);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4309);
			match s {
				[] => true,
				['i', 't', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6384);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xtus.
pub(crate) fn n2115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2096);
	s.is_empty()
}

/// Words ending -ytus.
pub(crate) fn n2116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2097);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4310);
			s.is_empty()
		}
		['l', 'e', 's', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4311);
			s.is_empty()
		}
		['r', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ratrix.
pub(crate) fn n6507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6409);
	match s {
		[] => true,
		['c', 'e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 7995);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7996);
			s.is_empty()
		}
		['i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 7997);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7998);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 8942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7999);
			match s {
				[] => true,
				['a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 8943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 8000);
			match s {
				[] => true,
				['c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -satrix.
pub(crate) fn n6508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6410);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 8001);
			match s {
				[] => true,
				['e', 'f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8002);
			match s {
				[] => true,
				['e', 'v', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'c', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 8003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tatrix.
pub(crate) fn n6509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6411);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8004);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8947);
					s.is_empty()
				}
				['e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8005);
			match s {
				[] => true,
				['c', 'i', 'l', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8949);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 8006);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					super::push(out, 8951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 8007);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8008);
			match s {
				[] => true,
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'l', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 8009);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ctrix.
pub(crate) fn n4409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4359);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 6414);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6415);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 8011);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8953);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8012);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 8954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8013);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8014);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8015);
					s.is_empty()
				}
				['t', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6416);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6417);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 8956);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -etrix.
pub(crate) fn n4410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4360);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 6418);
			s.is_empty()
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 6419);
			s.is_empty()
		}
		['s', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 6420);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itrix.
pub(crate) fn n4411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4361);
	match s {
		[] => true,
		['b', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 6421);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 6422);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6423);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6424);
			s.is_empty()
		}
		['s', 'u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 6425);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6426);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltrix.
pub(crate) fn n4412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4362);
	match s {
		[] => true,
		['u', 'c', s @ ..] => {
			super::push(out, 6427);
			match s {
				[] => true,
				['i', 't', 'a', 't', 'e', 'i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otrix.
pub(crate) fn n4414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4364);
	s.is_empty()
}

/// Words ending -ptrix.
pub(crate) fn n4415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4365);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6428);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 8018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6429);
			s.is_empty()
		}
		['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 6430);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 6431);
			match s {
				[] => true,
				['e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8019);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word desertrix.
pub(crate) fn n4416(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4366);
	s.is_empty()
}

/// Words ending -strix.
pub(crate) fn n4417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4367);
	match s {
		[] => true,
		['a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6432);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6433);
			match s {
				[] => true,
				['b', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'h', '\u{1}', s @ ..] => {
			super::push(out, 6434);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6435);
			s.is_empty()
		}
		['y', 'h', '\u{1}', s @ ..] => {
			super::push(out, 6436);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utrix.
pub(crate) fn n4418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4368);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 6437);
			s.is_empty()
		}
		['c', 'o', 'l', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 6438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ificentia.
pub(crate) fn n9128(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 9352);
			match s {
				[] => true,
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 9472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'o', 'n', 'o', 'h', '\u{1}', s @ ..] => {
			super::push(out, 9353);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -scentia.
pub(crate) fn n6607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6507);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 8064);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8065);
			match s {
				[] => true,
				['c', 'u', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8971);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8972);
					match s {
						[] => true,
						['u', 'r', 'c', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 9354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8973);
					s.is_empty()
				}
				['n', 'a', 'v', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8974);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8975);
					match s {
						[] => true,
						['c', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'r', 'e', 'f', 'f', 'e', '\u{1}', s @ ..] => {
					super::push(out, 8976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 8066);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dentia.
pub(crate) fn n4593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4537);
	match s {
		[] => true,
		['e', 'r', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6508);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6509);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8067);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 8068);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 8069);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8070);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8071);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8978);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6510);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6511);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 8073);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8074);
					match s {
						[] => true,
						['p', 's', 'i', 'r', 'u', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gentia.
pub(crate) fn n4594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4538);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 6512);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6513);
			match s {
				[] => true,
				['l', 'g', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 8075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6514);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8076);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6515);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6516);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ientia.
pub(crate) fn n4595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4539);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6517);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8078);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 8982);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 8983);
							match s {
								[] => true,
								['s', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8079);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8984);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 9359);
									s.is_empty()
								}
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
			super::push(out, 6518);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6519);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 6520);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8080);
					match s {
						[] => true,
						['s', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6521);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lentia.
pub(crate) fn n4596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4540);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 6522);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8082);
					s.is_empty()
				}
				['s', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 8083);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8084);
					match s {
						[] => true,
						['i', 'l', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6523);
			match s {
				[] => true,
				['m', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 8085);
					s.is_empty()
				}
				['r', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 8086);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mentia.
pub(crate) fn n4597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4541);
	match s {
		[] => true,
		['a', '\u{1}', s @ ..] => {
			super::push(out, 6524);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6525);
			match s {
				[] => true,
				['h', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 8087);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8088);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nentia.
pub(crate) fn n4598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4542);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 8089);
					s.is_empty()
				}
				['s', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8090);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rentia.
pub(crate) fn n4599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4543);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6526);
			match s {
				[] => true,
				['p', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 8092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6527);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 6528);
			match s {
				[] => true,
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 8093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6529);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tentia.
pub(crate) fn n4601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4545);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6530);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8987);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 8988);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6531);
			match s {
				[] => true,
				['i', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uentia.
pub(crate) fn n4602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4546);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 6532);
			match s {
				[] => true,
				['f', 'n', s @ ..] => {
					super::push(out, 8095);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 8989);
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 6533);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8096);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8991);
							match s {
								[] => true,
								['f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9360);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8992);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8097);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8098);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 9362);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 9363);
									match s {
										[] => true,
										['l', 'i', 't', 'b', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9473);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 9474);
											match s {
												[] => true,
												['a', 'v', '\u{1}', s @ ..] => {
													super::push(out, 9494);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 9475);
											s.is_empty()
										}
										['v', s @ ..] => {
											super::push(out, 9476);
											match s {
												[] => true,
												['a', 'u', 's', '\u{1}', s @ ..] => {
													super::push(out, 9495);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word coniventia.
pub(crate) fn n6635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6534);
	s.is_empty()
}

/// Words ending -untia.
pub(crate) fn n2473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2444);
	s.is_empty()
}

/// The word lucubratiuncula.
pub(crate) fn n9659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9496);
	s.is_empty()
}

/// Words ending -tarium.
pub(crate) fn n5040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4974);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6760);
			match s {
				[] => true,
				['b', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6761);
			match s {
				[] => true,
				['n', 'a', 'l', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8203);
					s.is_empty()
				}
				['r', 'c', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 8204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6762);
			match s {
				[] => true,
				['e', 'm', 'a', s @ ..] => {
					super::push(out, 8205);
					match s {
						[] => true,
						['r', 't', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6763);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6764);
			match s {
				[] => true,
				['i', 'l', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 8206);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tuarium.
pub(crate) fn n6869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6765);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 8207);
			s.is_empty()
		}
		['p', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 8208);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8209);
			match s {
				[] => true,
				['u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9031);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -varium.
pub(crate) fn n5042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4976);
	match s {
		[] => true,
		['a', 'l', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6766);
			s.is_empty()
		}
		['i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 6767);
			s.is_empty()
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 6768);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -brium.
pub(crate) fn n3032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3001);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 4977);
			match s {
				[] => true,
				['d', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 6769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4978);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -erium.
pub(crate) fn n3033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3002);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4979);
			match s {
				[] => true,
				['h', 'p', 's', 'i', 'm', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 6770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'i', 's', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 4980);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4981);
			match s {
				[] => true,
				['i', 'r', 'f', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4982);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4983);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 6772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'm', 'o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 4984);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4985);
			match s {
				[] => true,
				['i', 'c', 'c', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 6773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4986);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6774);
					match s {
						[] => true,
						['m', 'a', 'i', 'm', 'y', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 8210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 6775);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8211);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'm', 'e', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 6776);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6777);
					match s {
						[] => true,
						['u', 'd', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6778);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9032);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9377);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 9378);
											match s {
												[] => true,
												['r', 'h', 'p', 'o', 's', '\u{1}', s @ ..] => {
													super::push(out, 9483);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 6779);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 6780);
					match s {
						[] => true,
						['b', 's', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', 'o', 't', 's', 'e', '\u{1}', s @ ..] => {
			super::push(out, 4987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irium.
pub(crate) fn n3034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3003);
	match s {
		[] => true,
		['l', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 4988);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orium.
pub(crate) fn n3035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3004);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4989);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 4990);
			s.is_empty()
		}
		['d', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4991);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 4992);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 4993);
			match s {
				[] => true,
				['i', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 6781);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6782);
					match s {
						[] => true,
						['o', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 8216);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 6783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'r', 'e', 'v', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 4994);
			s.is_empty()
		}
		['p', 'm', 'e', '\u{1}', s @ ..] => {
			super::push(out, 4995);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4996);
			match s {
				[] => true,
				['a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6784);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6785);
					s.is_empty()
				}
				['n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 6786);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6787);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							super::push(out, 8217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6788);
					s.is_empty()
				}
				['u', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4997);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6790);
					match s {
						[] => true,
						['b', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 8218);
							s.is_empty()
						}
						['e', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8219);
							s.is_empty()
						}
						['g', 'r', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8220);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8221);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8222);
							match s {
								[] => true,
								['f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', 'l', 'c', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8223);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8224);
							match s {
								[] => true,
								['o', 'b', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 9034);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8225);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 9035);
									match s {
										[] => true,
										['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9379);
											s.is_empty()
										}
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
					super::push(out, 6791);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8226);
							match s {
								[] => true,
								['f', 'l', 'o', '\u{1}', s @ ..] => {
									super::push(out, 9036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 8227);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9037);
									s.is_empty()
								}
								['i', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 9038);
									s.is_empty()
								}
								['r', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8228);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 9040);
									match s {
										[] => true,
										['m', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9380);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8229);
							match s {
								[] => true,
								['d', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9041);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6792);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6793);
					match s {
						[] => true,
						['d', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8231);
							s.is_empty()
						}
						['m', 'r', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6794);
					match s {
						[] => true,
						['o', 'm', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6795);
					match s {
						[] => true,
						['i', 'r', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 8234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6796);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6797);
					match s {
						[] => true,
						['i', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6798);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -prium.
pub(crate) fn n3036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3005);
	s.is_empty()
}

/// Words ending -trium.
pub(crate) fn n3037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3006);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4998);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 6799);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'u', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 4999);
			s.is_empty()
		}
		['s', 'u', 'l', 'l', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5000);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urium.
pub(crate) fn n3038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3007);
	match s {
		[] => true,
		['a', 't', 'o', 'p', 'y', 'h', '\u{1}', s @ ..] => {
			super::push(out, 5001);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 5002);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5003);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5004);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 6801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 's', '\u{1}', s @ ..] => {
			super::push(out, 5005);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5006);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yrium.
pub(crate) fn n3039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3008);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5007);
			match s {
				[] => true,
				['l', 'o', 'c', 'o', 'r', 'e', 'x', '\u{1}', s @ ..] => {
					super::push(out, 6802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'r', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5008);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rticulum.
pub(crate) fn n8424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8285);
	s.is_empty()
}

/// The word posticulum.
pub(crate) fn n8425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8286);
	s.is_empty()
}

/// Words ending -lculum.
pub(crate) fn n5153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5087);
	s.is_empty()
}

/// Words ending -nculum.
pub(crate) fn n5154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5088);
	match s {
		[] => true,
		['u', 'i', 'c', 'a', 'd', 'n', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 6865);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oculum.
pub(crate) fn n5155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5089);
	s.is_empty()
}

/// Words ending -rculum.
pub(crate) fn n5156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5090);
	match s {
		[] => true,
		['e', 'b', 'u', 't', '\u{1}', s @ ..] => {
			super::push(out, 6866);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sculum.
pub(crate) fn n5157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5091);
	match s {
		[] => true,
		['a', 'v', '\u{1}', s @ ..] => {
			super::push(out, 6867);
			s.is_empty()
		}
		['o', '\u{1}', s @ ..] => {
			super::push(out, 6868);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6869);
			match s {
				[] => true,
				['h', 't', '\u{1}', s @ ..] => {
					super::push(out, 8287);
					s.is_empty()
				}
				['i', '\u{1}', s @ ..] => {
					super::push(out, 8288);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8289);
					match s {
						[] => true,
						['o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8290);
					match s {
						[] => true,
						['u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 9061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8291);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8292);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 8293);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uculum.
pub(crate) fn n5158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5092);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 6870);
			match s {
				[] => true,
				['e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 8294);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gulum.
pub(crate) fn n3092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3060);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5093);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 6871);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 6872);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5094);
			match s {
				[] => true,
				['a', 't', 'c', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 6873);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nulum.
pub(crate) fn n3093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3061);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5095);
			s.is_empty()
		}
		['e', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5096);
			s.is_empty()
		}
		['o', 'h', 'p', 'e', 'l', 'e', 't', '\u{1}', s @ ..] => {
			super::push(out, 5097);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pulum.
pub(crate) fn n3094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3062);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5098);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					super::push(out, 6874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5099);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8295);
							s.is_empty()
						}
						['s', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 8296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'r', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 5100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sulum.
pub(crate) fn n3095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3063);
	s.is_empty()
}

/// Words ending -tulum.
pub(crate) fn n3096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3064);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5101);
			match s {
				[] => true,
				['p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 6875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5102);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5103);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mentum.
pub(crate) fn n5285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5216);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6934);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8309);
					match s {
						[] => true,
						['i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9062);
							s.is_empty()
						}
						['r', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 9063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8310);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9064);
							match s {
								[] => true,
								['d', 'u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9391);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'v', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8311);
					match s {
						[] => true,
						['u', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8312);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8313);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9068);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9069);
							match s {
								[] => true,
								['u', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9392);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8314);
					match s {
						[] => true,
						['c', 's', 'e', 'r', 'c', 'c', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9070);
							s.is_empty()
						}
						['n', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8315);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9072);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9073);
							match s {
								[] => true,
								['i', 't', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 9393);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 9074);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9075);
							match s {
								[] => true,
								['c', 'i', 'n', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8316);
					match s {
						[] => true,
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8317);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9077);
							match s {
								[] => true,
								['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9395);
									s.is_empty()
								}
								['m', 'o', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9078);
							s.is_empty()
						}
						['r', 'o', '\u{1}', s @ ..] => {
							super::push(out, 9079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8318);
					match s {
						[] => true,
						['l', 'u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8319);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9081);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 9082);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 9083);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9084);
							s.is_empty()
						}
						['f', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9085);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9086);
							match s {
								[] => true,
								['l', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9087);
							match s {
								[] => true,
								['b', 'o', 'r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9088);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 9399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 9089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8320);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 9090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8321);
					match s {
						[] => true,
						['a', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 9091);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 9092);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9484);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 9485);
											match s {
												[] => true,
												['e', 'd', '\u{1}', s @ ..] => {
													super::push(out, 9497);
													s.is_empty()
												}
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
							super::push(out, 9093);
							match s {
								[] => true,
								['v', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9094);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9095);
							s.is_empty()
						}
						['u', 't', '\u{1}', s @ ..] => {
							super::push(out, 9096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8322);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8323);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6935);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8324);
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					super::push(out, 8325);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8326);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 9097);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8327);
					match s {
						[] => true,
						['c', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 9099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6936);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8328);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8329);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 9100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8330);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 9101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6937);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8331);
					s.is_empty()
				}
				['d', s @ ..] => super::t3::n8471(s, out),
				['g', s @ ..] => super::t3::n8472(s, out),
				['l', s @ ..] => super::t3::n8473(s, out),
				['n', s @ ..] => super::t3::n8474(s, out),
				['r', s @ ..] => super::t3::n8475(s, out),
				['t', s @ ..] => super::t3::n8476(s, out),
				['v', s @ ..] => super::t3::n8477(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6938);
			s.is_empty()
		}
		['m', s @ ..] => super::t3::n7048(s, out),
		['o', s @ ..] => super::t3::n7049(s, out),
		['r', s @ ..] => super::t3::n7050(s, out),
		['u', s @ ..] => super::t3::n7051(s, out),
		_ => false,
	}
}

/// Words ending -ficatio.
pub(crate) fn n7154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7043);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['b', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 9130);
					s.is_empty()
				}
				['c', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 9131);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9132);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9407);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 9408);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 9486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9133);
					match s {
						[] => true,
						['a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 9410);
							s.is_empty()
						}
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9411);
							s.is_empty()
						}
						['i', 'p', 'i', 'l', 'i', 'h', 'i', 'n', 'i', 'c', 'u', 'a', 'n', 'i', 'c', 'c', 'o', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 9412);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 9413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 9134);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9135);
					match s {
						[] => true,
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 9414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9136);
					match s {
						[] => true,
						['a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 9415);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 9416);
							s.is_empty()
						}
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 9417);
							s.is_empty()
						}
						['o', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9418);
							s.is_empty()
						}
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 9419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9137);
					match s {
						[] => true,
						['a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 9420);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 9421);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9422);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9423);
							match s {
								[] => true,
								['u', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 9138);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ncatio.
pub(crate) fn n5429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5357);
	match s {
		[] => true,
		['u', 'r', '\u{1}', s @ ..] => {
			super::push(out, 7050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ocatio.
pub(crate) fn n5430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5358);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 7051);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 7052);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 8396);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9140);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8398);
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 8399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rcatio.
pub(crate) fn n5431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5359);
	s.is_empty()
}

/// Words ending -scatio.
pub(crate) fn n5432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5360);
	s.is_empty()
}

/// Words ending -ucatio.
pub(crate) fn n5433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5361);
	match s {
		[] => true,
		['d', 'n', 'a', 'm', '\u{1}', s @ ..] => {
			super::push(out, 7053);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -datio.
pub(crate) fn n3441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3407);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5362);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5363);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8400);
							s.is_empty()
						}
						['o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5364);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7054);
					match s {
						[] => true,
						['r', 'p', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 7055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5365);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7056);
					match s {
						[] => true,
						['u', 'l', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7057);
					match s {
						[] => true,
						['i', 'r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7058);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7059);
					match s {
						[] => true,
						['a', 'l', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5366);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5367);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7060);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7061);
					match s {
						[] => true,
						['m', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7062);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8408);
							match s {
								[] => true,
								['o', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9141);
									s.is_empty()
								}
								['r', 'i', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 8409);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 9143);
									s.is_empty()
								}
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
			super::push(out, 5368);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7063);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 8410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5369);
			s.is_empty()
		}
		['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 5370);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5371);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7064);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8411);
							s.is_empty()
						}
						['r', 'f', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7065);
					s.is_empty()
				}
				['s', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7066);
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eatio.
pub(crate) fn n3442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3408);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 5372);
			match s {
				[] => true,
				['c', 'u', 'n', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'i', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5373);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5374);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7069);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5375);
			match s {
				[] => true,
				['q', 'a', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7071);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fatio.
pub(crate) fn n3443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3409);
	s.is_empty()
}

/// Words ending -gatio.
pub(crate) fn n3444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3410);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5376);
			match s {
				[] => true,
				['p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7072);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 7073);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5377);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7074);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8414);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 9144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7075);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5378);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7077);
					match s {
						[] => true,
						['a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8415);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 8416);
							s.is_empty()
						}
						['f', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8417);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 8418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7078);
					match s {
						[] => true,
						['a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7079);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8420);
							match s {
								[] => true,
								['f', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9145);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'f', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8421);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8422);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 7080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5379);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5380);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5381);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8424);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8425);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8426);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8427);
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
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5383);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7081);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7082);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hatio.
pub(crate) fn n3445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3411);
	match s {
		[] => true,
		['c', 'c', 'a', 'b', 'e', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iatio.
pub(crate) fn n3446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3412);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5385);
			match s {
				[] => true,
				['o', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5386);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7084);
					match s {
						[] => true,
						['m', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 5387);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5388);
			s.is_empty()
		}
		['m', 'e', 'd', 'n', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 5389);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5390);
			match s {
				[] => true,
				['b', 'e', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7085);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7086);
					match s {
						[] => true,
						['c', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', 'a', 'b', '\u{1}', s @ ..] => {
			super::push(out, 5391);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5392);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7087);
					match s {
						[] => true,
						['f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7088);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8432);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8433);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['b', 'o', '\u{1}', s @ ..] => {
											super::push(out, 9424);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 9425);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 9488);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9426);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'g', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 5393);
			match s {
				[] => true,
				['e', 'r', 'b', s @ ..] => {
					match s {
						[] => true,
						['b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8434);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8435);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -latio.
pub(crate) fn n3447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3413);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5394);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 7090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5395);
			s.is_empty()
		}
		['c', 'n', 'e', 'm', 'o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5396);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5397);
			match s {
				[] => true,
				['g', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7091);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 7092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5398);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7093);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5399);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7095);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 8436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', '\u{1}', s @ ..] => {
					super::push(out, 7096);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7097);
					match s {
						[] => true,
						['i', 'r', 'r', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 8437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5400);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7099);
					match s {
						[] => true,
						['b', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8438);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 8439);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8440);
							s.is_empty()
						}
						['t', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7100);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8442);
							match s {
								[] => true,
								['s', 'o', '\u{1}', s @ ..] => {
									super::push(out, 9147);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8443);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9148);
									s.is_empty()
								}
								['i', 't', '\u{1}', s @ ..] => {
									super::push(out, 9149);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 9150);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9427);
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9428);
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9429);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5401);
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7101);
					s.is_empty()
				}
				['m', 'm', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7102);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7103);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 7104);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 8445);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5402);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5403);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7105);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8446);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 9151);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 8447);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7106);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8449);
							match s {
								[] => true,
								['i', 'e', '\u{1}', s @ ..] => {
									super::push(out, 9153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8450);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8451);
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 9154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8452);
							s.is_empty()
						}
						['s', 'o', '\u{1}', s @ ..] => {
							super::push(out, 8453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7107);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 8454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7108);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7109);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8455);
							match s {
								[] => true,
								['s', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9155);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8456);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7110);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8458);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8459);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 9156);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 9157);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9430);
											s.is_empty()
										}
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
					super::push(out, 7111);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8460);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8461);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -matio.
pub(crate) fn n3448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3414);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5404);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7112);
					s.is_empty()
				}
				['g', 'l', 'a', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7113);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7114);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9158);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9431);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 9159);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9432);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9160);
									s.is_empty()
								}
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
			super::push(out, 5405);
			match s {
				[] => true,
				['c', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7115);
					s.is_empty()
				}
				['h', 'p', 's', 'a', 'l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 7116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5406);
			match s {
				[] => true,
				['l', 'b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7117);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7118);
					s.is_empty()
				}
				['s', 'e', 't', 'n', 'e', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7119);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7120);
					match s {
						[] => true,
						['s', 'i', 'x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5407);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5408);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7121);
					match s {
						[] => true,
						['f', 'n', s @ ..] => {
							super::push(out, 8464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7122);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9161);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9433);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 9162);
									s.is_empty()
								}
								['s', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 9163);
									s.is_empty()
								}
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
			super::push(out, 5409);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 7123);
					match s {
						[] => true,
						['s', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', 'i', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -natio.
pub(crate) fn n3449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3415);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5410);
			match s {
				[] => true,
				['m', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5411);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5412);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7126);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7127);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8466);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8467);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9164);
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7128);
					match s {
						[] => true,
						['p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5413);
			match s {
				[] => true,
				['b', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7129);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7130);
					match s {
						[] => true,
						['i', 't', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 8469);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8470);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7131);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7132);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8473);
							match s {
								[] => true,
								['m', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9166);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'i', 'v', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7133);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7134);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9168);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7135);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8475);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8476);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 9170);
									match s {
										[] => true,
										['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9434);
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9435);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8477);
							match s {
								[] => true,
								['l', 'e', '\u{1}', s @ ..] => {
									super::push(out, 9171);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 9172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8478);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 9173);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9174);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9436);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8479);
							match s {
								[] => true,
								['e', 't', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7136);
					match s {
						[] => true,
						['m', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8481);
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							super::push(out, 8482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', 'e', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7137);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7138);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8483);
							match s {
								[] => true,
								['a', 'r', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9176);
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									super::push(out, 9177);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 9178);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9437);
											s.is_empty()
										}
										['f', '\u{1}', s @ ..] => {
											super::push(out, 9438);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'l', 'g', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7139);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5414);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5415);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5416);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7141);
					s.is_empty()
				}
				['s', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5417);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7143);
					match s {
						[] => true,
						['p', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8485);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8486);
							match s {
								[] => true,
								['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5418);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -patio.
pub(crate) fn n3450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3416);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5419);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5420);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7145);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8487);
							s.is_empty()
						}
						['n', 'a', 'm', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7146);
					match s {
						[] => true,
						['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5421);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5422);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7147);
					s.is_empty()
				}
				['u', 's', 'u', '\u{1}', s @ ..] => {
					super::push(out, 7148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5423);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ratio.
pub(crate) fn n3451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3417);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5424);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5425);
			match s {
				[] => true,
				['l', 'c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7149);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7150);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 8490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5426);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7151);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7152);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 8491);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 8492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7153);
					s.is_empty()
				}
				['o', 'r', 'p', 'x', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7154);
					s.is_empty()
				}
				['u', 'c', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5427);
			match s {
				[] => true,
				['e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 5428);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5429);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7157);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8493);
							match s {
								[] => true,
								['l', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7158);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8494);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 9181);
									match s {
										[] => true,
										['i', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9439);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 9182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 8495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7159);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8496);
							match s {
								[] => true,
								['f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9183);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8497);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8498);
							s.is_empty()
						}
						['y', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'i', 'c', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7160);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7161);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8500);
							match s {
								[] => true,
								['l', 'l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 9184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7162);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 9185);
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7163);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8501);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7164);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8503);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8504);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7165);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7166);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5430);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7168);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7169);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7170);
					match s {
						[] => true,
						['m', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 8506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5431);
			match s {
				[] => true,
				['l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7171);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7172);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7173);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 8507);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									super::push(out, 9188);
									s.is_empty()
								}
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
			super::push(out, 5432);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7174);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7175);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7176);
					s.is_empty()
				}
				['f', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7177);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7178);
					match s {
						[] => true,
						['r', 'e', 't', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7179);
					match s {
						[] => true,
						['o', 'c', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8510);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7180);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7181);
					match s {
						[] => true,
						['i', 'm', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8512);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7182);
					match s {
						[] => true,
						['a', 'v', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8513);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8514);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7183);
					s.is_empty()
				}
				['v', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5433);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7185);
					match s {
						[] => true,
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8515);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5434);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7187);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7188);
					match s {
						[] => true,
						['e', 'n', 'e', 'f', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8516);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8517);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8518);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8519);
							match s {
								[] => true,
								['l', 'l', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9189);
									s.is_empty()
								}
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
			super::push(out, 5435);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7189);
					match s {
						[] => true,
						['t', 's', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7190);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7191);
					s.is_empty()
				}
				['g', s @ ..] => super::t3::n7306(s, out),
				['i', s @ ..] => super::t3::n7307(s, out),
				['m', s @ ..] => {
					super::push(out, 7194);
					s.is_empty()
				}
				['t', s @ ..] => super::t3::n7309(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t3::n5508(s, out),
		_ => false,
	}
}

/// Words ending -satio.
pub(crate) fn n3452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3418);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5437);
			s.is_empty()
		}
		['e', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5438);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5439);
			match s {
				[] => true,
				['u', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5440);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7199);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 8526);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7200);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7201);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 8528);
							s.is_empty()
						}
						['p', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5441);
			match s {
				[] => true,
				['u', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7202);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5442);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7203);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7204);
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5443);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7205);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7206);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tatio.
pub(crate) fn n3453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3419);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5444);
			match s {
				[] => true,
				['l', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5445);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7208);
					match s {
						[] => true,
						['r', 't', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7209);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8532);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8533);
							match s {
								[] => true,
								['t', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8534);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9195);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9440);
											s.is_empty()
										}
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
					super::push(out, 7210);
					match s {
						[] => true,
						['l', 'f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7211);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7212);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 8536);
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5446);
			match s {
				[] => true,
				['h', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7213);
					s.is_empty()
				}
				['i', 'u', 'q', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7214);
					s.is_empty()
				}
				['l', 'p', 'u', 'c', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5447);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7216);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7217);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8538);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8539);
							match s {
								[] => true,
								['i', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 9196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7218);
					match s {
						[] => true,
						['u', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8540);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7219);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8541);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9197);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 9198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8542);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 9199);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									super::push(out, 9200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7220);
					match s {
						[] => true,
						['c', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8543);
							s.is_empty()
						}
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 8544);
							s.is_empty()
						}
						['i', 'b', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7221);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8546);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9201);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									super::push(out, 9202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7222);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7223);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7224);
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 8548);
							s.is_empty()
						}
						['n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 7225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8550);
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							super::push(out, 8551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5448);
			match s {
				[] => true,
				['u', 's', s @ ..] => {
					super::push(out, 7226);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5449);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7227);
					match s {
						[] => true,
						['l', 'p', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 8553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7228);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8554);
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 9203);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 9204);
									s.is_empty()
								}
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 9205);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 9206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8555);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5450);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7229);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8557);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7230);
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					super::push(out, 7231);
					s.is_empty()
				}
				['v', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5451);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7233);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 8558);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5452);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7234);
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 8559);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 9208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7235);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 8560);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 8561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5453);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7236);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 7237);
					match s {
						[] => true,
						['v', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7238);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9209);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7239);
					match s {
						[] => true,
						['g', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t3::n5526(s, out),
		_ => false,
	}
}

/// Words ending -uatio.
pub(crate) fn n3454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3420);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5455);
			match s {
				[] => true,
				['a', 'v', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5456);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					super::push(out, 7246);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 5457);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7247);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7248);
					match s {
						[] => true,
						['t', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 't', 's', 'n', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 5458);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5459);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7249);
					s.is_empty()
				}
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7250);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vatio.
pub(crate) fn n3455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3421);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5460);
			match s {
				[] => true,
				['r', 'p', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5461);
			match s {
				[] => true,
				['l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 7252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5462);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7253);
					match s {
						[] => true,
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5463);
			match s {
				[] => true,
				['a', 'c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5464);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5465);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8570);
							s.is_empty()
						}
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8571);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8572);
							match s {
								[] => true,
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xatio.
pub(crate) fn n3456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3422);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5466);
			s.is_empty()
		}
		['u', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zatio.
pub(crate) fn n3457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3423);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7255);
					s.is_empty()
				}
				['r', 'a', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7256);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iator.
pub(crate) fn n3584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3546);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5618);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 5619);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7392);
					s.is_empty()
				}
				['i', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5620);
			match s {
				[] => true,
				['i', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'e', 'd', 'n', 'i', 'v', '\u{1}', s @ ..] => {
			super::push(out, 5621);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5622);
			s.is_empty()
		}
		['r', 'u', 'i', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 5623);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5624);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7395);
					match s {
						[] => true,
						['u', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 9228);
									s.is_empty()
								}
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'g', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 5625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lator.
pub(crate) fn n3585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3547);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5626);
			match s {
				[] => true,
				['b', 'i', 't', 's', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5627);
			s.is_empty()
		}
		['c', 'n', 'e', 'm', 'o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 5628);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5629);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7398);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 5630);
			match s {
				[] => true,
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5631);
			match s {
				[] => true,
				['g', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7400);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7401);
					s.is_empty()
				}
				['t', 'n', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7402);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5632);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7403);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8634);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 8635);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 8636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'u', 'g', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 5633);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5634);
			match s {
				[] => true,
				['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 5635);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5636);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5637);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7406);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8637);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 8638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 7407);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8639);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9444);
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9445);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8640);
							s.is_empty()
						}
						['n', 'e', 'm', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 8641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7408);
					s.is_empty()
				}
				['g', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7409);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7410);
					match s {
						[] => true,
						['i', 't', 's', s @ ..] => {
							super::push(out, 8642);
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7411);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8643);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8644);
							match s {
								[] => true,
								['p', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -amator.
pub(crate) fn n5720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5638);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 7412);
			match s {
				[] => true,
				['c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 8645);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -imator.
pub(crate) fn n5722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5640);
	match s {
		[] => true,
		['t', 's', 'i', 'x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 7413);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmator.
pub(crate) fn n5724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5642);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7414);
			match s {
				[] => true,
				['f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 8646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7415);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nator.
pub(crate) fn n3587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3549);
	match s {
		[] => true,
		['a', 'l', 'p', 'x', 'e', '\u{1}', s @ ..] => {
			super::push(out, 5643);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5644);
			match s {
				[] => true,
				['v', '\u{1}', s @ ..] => {
					super::push(out, 7416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5645);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7417);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7418);
					match s {
						[] => true,
						['s', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7419);
					match s {
						[] => true,
						['p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5646);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7420);
					s.is_empty()
				}
				['d', 'r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7421);
					s.is_empty()
				}
				['g', 'r', 'i', 'v', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7422);
					s.is_empty()
				}
				['h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7423);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7424);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8649);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8650);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8651);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 9233);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 9234);
									match s {
										[] => true,
										['e', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9446);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8652);
							s.is_empty()
						}
						['u', 'l', 'l', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7425);
					match s {
						[] => true,
						['u', '\u{1}', s @ ..] => {
							super::push(out, 8654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'u', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 7426);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7427);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5647);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5648);
			match s {
				[] => true,
				['i', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5649);
			s.is_empty()
		}
		['u', 'i', 'e', 'i', '\u{1}', s @ ..] => {
			super::push(out, 5650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pator.
pub(crate) fn n3588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3550);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5651);
			s.is_empty()
		}
		['i', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 5652);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rator.
pub(crate) fn n3589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3551);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5653);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5654);
			match s {
				[] => true,
				['p', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5655);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 7430);
					s.is_empty()
				}
				['e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7431);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5656);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7433);
					match s {
						[] => true,
						['i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 7434);
					match s {
						[] => true,
						['i', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7435);
					match s {
						[] => true,
						['i', 'r', 'f', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7436);
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5657);
			match s {
				[] => true,
				['i', 'n', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 7438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5658);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5659);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7439);
					s.is_empty()
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 7440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5660);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 5661);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7441);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 8660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7442);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8661);
							match s {
								[] => true,
								['o', 'm', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9235);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5662);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7443);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'l', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 7444);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7445);
					s.is_empty()
				}
				['m', 'r', 'u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 7446);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sator.
pub(crate) fn n3590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3552);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5663);
			s.is_empty()
		}
		['e', 'm', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 5664);
			s.is_empty()
		}
		['n', 'e', 'p', 's', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5665);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5666);
			match s {
				[] => true,
				['u', 'c', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5667);
			match s {
				[] => true,
				['i', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7448);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'c', 'c', 'a', '\u{1}', s @ ..] => {
			super::push(out, 5668);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tator.
pub(crate) fn n3591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3553);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5669);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7449);
					match s {
						[] => true,
						['r', 't', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7450);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8665);
							match s {
								[] => true,
								['t', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9236);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8666);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 9237);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7451);
					s.is_empty()
				}
				['u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 7452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5670);
			match s {
				[] => true,
				['r', 'p', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5671);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7454);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 7455);
					match s {
						[] => true,
						['s', 'i', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 8667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 7456);
					match s {
						[] => true,
						['a', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7457);
					match s {
						[] => true,
						['a', 'l', 'c', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7458);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7459);
					match s {
						[] => true,
						['i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 8670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'c', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5672);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5673);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7461);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8671);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9238);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'p', 'p', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7462);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'p', '\u{1}', s @ ..] => {
			super::push(out, 5674);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 5675);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5676);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5677);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7463);
					s.is_empty()
				}
				['a', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7464);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5678);
			match s {
				[] => true,
				['f', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7466);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uator.
pub(crate) fn n3592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3554);
	match s {
		[] => true,
		['n', 'i', 's', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 5679);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vator.
pub(crate) fn n3593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3555);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5680);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5681);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5682);
			match s {
				[] => true,
				['e', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7467);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zator.
pub(crate) fn n3594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3556);
	s.is_empty()
}

/// Words ending -litas.
pub(crate) fn n3660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3620);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5765);
			match s {
				[] => true,
				['c', 'o', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7537);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7538);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7539);
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8698);
							s.is_empty()
						}
						['i', 'd', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8699);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8700);
							match s {
								[] => true,
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 9245);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7540);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7542);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 8702);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8703);
							match s {
								[] => true,
								['d', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9246);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 9247);
									s.is_empty()
								}
								['g', 'i', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 9248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'i', 't', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8704);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', 'c', 'n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7543);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7544);
					match s {
						[] => true,
						['e', 'b', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 8705);
							s.is_empty()
						}
						['g', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8706);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 8707);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 9249);
									s.is_empty()
								}
								['p', 'm', 'e', 't', 'x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 9250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8708);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8709);
							match s {
								[] => true,
								['t', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 9251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7545);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 9252);
									s.is_empty()
								}
								['r', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 9253);
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									super::push(out, 9254);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 8710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7546);
					match s {
						[] => true,
						['q', s @ ..] => {
							super::push(out, 8711);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8712);
							match s {
								[] => true,
								['n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 9255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 8713);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9256);
									s.is_empty()
								}
								['i', 'r', 'i', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 9257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 7547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5766);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8714);
							match s {
								[] => true,
								['f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5767);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7548);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8716);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9259);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 9260);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 9261);
									match s {
										[] => true,
										['a', 'p', 'e', 's', '\u{1}', s @ ..] => {
											super::push(out, 9447);
											s.is_empty()
										}
										['i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
											super::push(out, 9448);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 9449);
											match s {
												[] => true,
												['s', 'n', 'e', 'm', 'm', 'o', 'c', 'n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9489);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', 'n', 'o', 'p', 's', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 9262);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 9263);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 9450);
											match s {
												[] => true,
												['n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 9490);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 9451);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8717);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8718);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 9264);
									s.is_empty()
								}
								['s', s @ ..] => super::t3::n9427(s, out),
								['t', s @ ..] => super::t3::n9428(s, out),
								_ => false,
							}
						}
						['o', s @ ..] => super::t3::n8869(s, out),
						['u', s @ ..] => super::t3::n8870(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t3::n7673(s, out),
				['d', 'e', 'a', '\u{1}', s @ ..] => super::t3::n7674(s, out),
				['g', s @ ..] => super::t3::n7675(s, out),
				['m', s @ ..] => super::t3::n7676(s, out),
				['n', 'a', '\u{1}', s @ ..] => super::t3::n7677(s, out),
				['r', s @ ..] => super::t3::n7678(s, out),
				['t', s @ ..] => super::t3::n7679(s, out),
				['v', s @ ..] => super::t3::n7680(s, out),
				['x', s @ ..] => super::t3::n7681(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t3::n5852(s, out),
		['o', 's', '\u{1}', s @ ..] => super::t3::n5853(s, out),
		['u', s @ ..] => super::t3::n5854(s, out),
		_ => false,
	}
}

/// Words ending -nitas.
pub(crate) fn n3662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3622);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5776);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7565);
					s.is_empty()
				}
				['g', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7566);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7567);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 8732);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7568);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5777);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7569);
					s.is_empty()
				}
				['o', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5778);
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 7571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5779);
			match s {
				[] => true,
				['c', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7572);
					s.is_empty()
				}
				['d', 'u', 't', 'i', 'l', 'i', 'b', 'a', 'c', 'i', 'f', 'i', 'r', 'o', 'n', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 7573);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 7574);
					match s {
						[] => true,
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'r', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 7575);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7576);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7577);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7578);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7579);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5780);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5781);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5782);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 7580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5783);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7581);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9269);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 9456);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 9457);
											s.is_empty()
										}
										['u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 9458);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 9270);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7582);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 9271);
									s.is_empty()
								}
								['u', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 9272);
									s.is_empty()
								}
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
			super::push(out, 5784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ritas.
pub(crate) fn n3663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3623);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5785);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7583);
					s.is_empty()
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7584);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 7585);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7586);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8737);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8738);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 9273);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8739);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 9274);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 9459);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 9275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 5786);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7588);
					match s {
						[] => true,
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'i', 'u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7589);
					s.is_empty()
				}
				['u', 'l', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 7590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5787);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 7591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5788);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7592);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 8741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7593);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7594);
					s.is_empty()
				}
				['t', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7595);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5789);
			s.is_empty()
		}
		['i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 5790);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5791);
			match s {
				[] => true,
				['i', 'r', s @ ..] => {
					super::push(out, 7597);
					match s {
						[] => true,
						['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 8742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5792);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7598);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8743);
							match s {
								[] => true,
								['s', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 9276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7599);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7600);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 9277);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 9278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sitas.
pub(crate) fn n3664(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3624);
	match s {
		[] => true,
		['e', 'b', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5793);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5794);
			match s {
				[] => true,
				['u', 's', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 7601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 5795);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5796);
			match s {
				[] => true,
				['d', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 7602);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7603);
					match s {
						[] => true,
						['p', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8744);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8745);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8746);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 9279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7604);
					match s {
						[] => true,
						['u', 'b', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7605);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8748);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 9280);
									s.is_empty()
								}
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 9281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 8749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7606);
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 8750);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8751);
							match s {
								[] => true,
								['d', 'i', 'b', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 9282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7607);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8752);
							match s {
								[] => true,
								['b', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 9283);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 8753);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 9284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 't', 'c', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5797);
			match s {
				[] => true,
				['e', 'v', 'i', s @ ..] => {
					super::push(out, 7609);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8754);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 8755);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 5798);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -astitas.
pub(crate) fn n7734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7610);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 8756);
			s.is_empty()
		}
		['v', '\u{1}', s @ ..] => {
			super::push(out, 8757);
			s.is_empty()
		}
		_ => false,
	}
}

