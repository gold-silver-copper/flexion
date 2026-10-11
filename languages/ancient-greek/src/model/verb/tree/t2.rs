use flexion::Path;

/// Words ending -ινω.
pub(crate) fn n231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 231);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['β', s @ ..] => super::t3::n1956(s, out),
				['δ', s @ ..] => {
					super::push(out, 1924);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3418);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4962);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6295);
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
						['ρ', s @ ..] => {
							super::push(out, 3419);
							match s {
								[] => true,
								['ε', s @ ..] => super::t3::n5337(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => super::t3::n1958(s, out),
				['θ', s @ ..] => super::t3::n1959(s, out),
				['ι', s @ ..] => super::t3::n1960(s, out),
				['κ', s @ ..] => super::t3::n1961(s, out),
				['λ', s @ ..] => super::t3::n1962(s, out),
				['μ', s @ ..] => super::t3::n1963(s, out),
				['ξ', s @ ..] => {
					super::push(out, 1931);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t3::n3596(s, out),
						_ => false,
					}
				}
				['π', s @ ..] => super::t3::n1965(s, out),
				['ρ', s @ ..] => super::t3::n1966(s, out),
				['σ', s @ ..] => super::t3::n1967(s, out),
				['φ', s @ ..] => super::t3::n1968(s, out),
				['χ', s @ ..] => super::t3::n1969(s, out),
				['ῥ', s @ ..] => super::t3::n1970(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t3::n806(s, out),
		['λ', s @ ..] => super::t3::n807(s, out),
		['π', s @ ..] => super::t3::n808(s, out),
		['ρ', s @ ..] => super::t3::n809(s, out),
		['τ', s @ ..] => super::t3::n810(s, out),
		_ => false,
	}
}

/// Words ending -κνω.
pub(crate) fn n232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 232);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1950);
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

/// Words ending -μνω.
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 233);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 811);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1952);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3474);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5018);
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
		['ε', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3475);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5019);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6329);
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
						['ι', s @ ..] => {
							super::push(out, 3476);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5020);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6330);
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
								['ρ', s @ ..] => {
									super::push(out, 5021);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6331);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7252);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9175(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 3477);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5022);
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
						['ν', s @ ..] => {
							super::push(out, 3478);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5023);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6332);
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
						['ο', s @ ..] => {
							super::push(out, 3479);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5024);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6333);
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
		['ι', s @ ..] => {
			super::push(out, 813);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1954);
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

/// Words ending -υνω.
pub(crate) fn n234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 234);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1955);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3480);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 5025);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6334);
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
				['ρ', s @ ..] => {
					super::push(out, 1956);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3481);
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
		['δ', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1957);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1958);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3482);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 5026);
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
				['η', s @ ..] => {
					super::push(out, 1959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1960);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3484);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5027);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6335);
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
				['ν', s @ ..] => {
					super::push(out, 1961);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3485);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5028);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6336);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7253);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 816);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1962);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 3486);
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
				['η', s @ ..] => {
					super::push(out, 1963);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3487);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5029);
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
				['ι', s @ ..] => {
					super::push(out, 1964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1965);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3489);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5030);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5031);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6337);
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
								['τ', s @ ..] => {
									super::push(out, 5032);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6338);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7254);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9176(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1966);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3490);
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
		['λ', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1967);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3491);
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
				['β', s @ ..] => {
					super::push(out, 1968);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3492);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5033);
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
				['λ', s @ ..] => {
					super::push(out, 1969);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3493);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5034);
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
				['ο', s @ ..] => {
					super::push(out, 1970);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3494);
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
				['π', s @ ..] => {
					super::push(out, 1971);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3495);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3496);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5035);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6339);
											match s {
												[] => true,
												['κ', s @ ..] => super::t3::n8333(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t3::n820(s, out),
		['ξ', s @ ..] => super::t3::n821(s, out),
		['ρ', s @ ..] => super::t3::n822(s, out),
		['σ', s @ ..] => super::t3::n823(s, out),
		['τ', s @ ..] => super::t3::n824(s, out),
		['φ', s @ ..] => super::t3::n825(s, out),
		['χ', s @ ..] => super::t3::n826(s, out),
		_ => false,
	}
}

/// Words ending -απακριβοω.
pub(crate) fn n8339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7258);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word μαστιγοω.
pub(crate) fn n8340(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ῥαφανιδοω.
pub(crate) fn n8341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7259);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιοω.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2001);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3524);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 5057);
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
				['κ', s @ ..] => {
					super::push(out, 2002);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3525);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5058);
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
				['ρ', s @ ..] => {
					super::push(out, 2003);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3526);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5059);
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
		['β', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2006);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3527);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5060);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6349);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2007);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3528);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5061);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6350);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3529);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 3530);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5062);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6351);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2009);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3531);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5063);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6352);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7260);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9182(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2010);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2011);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3533);
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
		['ξ', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2012);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3534);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 3535);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5064);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6353);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2013);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3536);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5065);
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
				['μ', s @ ..] => {
					super::push(out, 2014);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3537);
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
		['ρ', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2015);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3538);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5066);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6354);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7261);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9183(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2016);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3539);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5067);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6355);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7262);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9184(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2017);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3540);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 5068);
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

/// Words ending -κοω.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 853);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2020);
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
		['ρ', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2021);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3543);
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

/// Words ending -λοω.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2022);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 3544);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5069);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 6356);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2023);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 2024);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3545);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3546);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5070);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 2025);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2026);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3548);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5071);
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
		['ι', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 2027);
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
		['κ', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2028);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2029);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3550);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5073);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6357);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7263);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9185(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2031);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2032);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2033);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3553);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2034);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3554);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5075);
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
				['ο', s @ ..] => {
					super::push(out, 2035);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3555);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 3556);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5076);
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

/// Words ending -μοω.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2036);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3557);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5077);
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
		['η', s @ ..] => {
			super::push(out, 864);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2037);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3558);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5078);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2038);
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
		['μ', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2039);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3559);
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
		['ο', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2040);
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
		['υ', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 2041);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3560);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5079);
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
		['χ', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2042);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3561);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5080);
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

/// Words ending -νοω.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2043);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3562);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5081);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6358);
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
		['ε', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2044);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3563);
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
		['η', s @ ..] => {
			super::push(out, 872);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2045);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3564);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 5082);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6359);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7264);
													match s {
														[] => true,
														['κ', s @ ..] => super::t3::n9186(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2046);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3565);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 5083);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6360);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7265);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9187(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2047);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2048);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2049);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3568);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2050);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2051);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3570);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5086);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6361);
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
		_ => false,
	}
}

/// Words ending -οξοω.
pub(crate) fn n878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 877);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 2052);
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

/// Words ending -ποω.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 250);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 878);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2053);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3571);
							match s {
								[] => true,
								['ε', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ροω.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 879);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2054);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2055);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2056);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3572);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 5088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2057);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3573);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5089);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6362);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7266);
													match s {
														[] => true,
														['τ', s @ ..] => super::t3::n9188(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 881);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2059);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3575);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5090);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6363);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7267);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9189(s, out),
														['π', s @ ..] => super::t3::n9190(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2060);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3576);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 5091);
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
				['τ', s @ ..] => {
					super::push(out, 2061);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3577);
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
		['η', s @ ..] => {
			super::push(out, 882);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2062);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 5092);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 6364);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7268);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9191(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 883);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2063);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3579);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5093);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6365);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7269);
													match s {
														[] => true,
														['κ', s @ ..] => super::t3::n9192(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2064);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3580);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5094);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 5095);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6366);
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
								['ο', s @ ..] => {
									super::push(out, 5096);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6367);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7270);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9193(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 884);
			match s {
				[] => true,
				['α', s @ ..] => super::t3::n2099(s, out),
				['γ', s @ ..] => {
					super::push(out, 2066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => super::t3::n2101(s, out),
				['π', s @ ..] => super::t3::n2102(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σοω.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 885);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 2069);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3585);
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
		['σ', s @ ..] => {
			super::push(out, 886);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2070);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3586);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5099);
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

/// Words ending -τοω.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2071);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3587);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5100);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6368);
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
		['ι', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2072);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3588);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 5101);
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
		['ο', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2073);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3589);
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
		['ρ', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2074);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 3590);
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
		['σ', s @ ..] => {
			super::push(out, 891);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2075);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3591);
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
				['ε', s @ ..] => {
					super::push(out, 2076);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3592);
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

/// Words ending -φοω.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2077);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3593);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 5102);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6369);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7271);
													match s {
														[] => true,
														['μ', s @ ..] => super::t3::n9194(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χοω.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 893);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2078);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3594);
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
		['σ', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2079);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3595);
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
		['υ', s @ ..] => {
			super::push(out, 896);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 2080);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3596);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5103);
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

/// Words ending -ψοω.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 897);
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

/// The word αναβλεπω.
pub(crate) fn n8358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7272);
	s.is_empty()
}

/// Words ending -παραβλεπω.
pub(crate) fn n8359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7273);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δρεπω.
pub(crate) fn n2125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2089);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πρεπω.
pub(crate) fn n2126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2090);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3604);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 5109);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6373);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7274);
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

/// Words ending -ρρεπω.
pub(crate) fn n2127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2091);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3605);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5110);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6374);
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

/// Words ending -τρεπω.
pub(crate) fn n2128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2092);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3606);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3607);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5111);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6375);
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
				['τ', s @ ..] => {
					super::push(out, 5112);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6376);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7275);
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
		['ι', s @ ..] => {
			super::push(out, 3608);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5113);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6377);
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
		['κ', s @ ..] => {
			super::push(out, 3609);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 6378);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7276);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7799);
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
		['ν', s @ ..] => {
			super::push(out, 3610);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5115);
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
		['ο', s @ ..] => {
			super::push(out, 3611);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5116);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6379);
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
				['ρ', s @ ..] => {
					super::push(out, 5117);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6380);
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

/// Words ending -φεπω.
pub(crate) fn n906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 905);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2093);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2094);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3612);
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

/// Words ending -ῥεπω.
pub(crate) fn n907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 906);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηπω.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 907);
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

/// Words ending -ιπω.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3613);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5118);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6381);
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
								['ρ', s @ ..] => {
									super::push(out, 5119);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6382);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7277);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9199(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 5120);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6383);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7278);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9200(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 3614);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5121);
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
						['λ', s @ ..] => {
							super::push(out, 3615);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5122);
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
						['ο', s @ ..] => {
							super::push(out, 3616);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5123);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6384);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 6385);
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
								['ρ', s @ ..] => {
									super::push(out, 5124);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6386);
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
				['ρ', s @ ..] => {
					super::push(out, 2097);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3617);
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

/// Words ending -λπω.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3618);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3619);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5125);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6387);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7279);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9201(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2099);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2100);
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

/// Words ending -μπω.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 3620);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5126);
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
		['ε', s @ ..] => {
			super::push(out, 912);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2102);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3621);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3622);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5127);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6388);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7280);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9202(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 5128);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6389);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7281);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9203(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 3623);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5129);
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
						['ο', s @ ..] => {
							super::push(out, 3624);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5130);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6390);
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
								['ρ', s @ ..] => {
									super::push(out, 5131);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6391);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7282);
													s.is_empty()
												}
												['μ', s @ ..] => {
													super::push(out, 7283);
													match s {
														[] => true,
														['υ', s @ ..] => super::t3::n9204(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3625);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5132);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6392);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7284);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9205(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρπω.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2103);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2104);
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

/// The word μεταφερω.
pub(crate) fn n8381(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word περιφερω.
pub(crate) fn n8382(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word υπερφερω.
pub(crate) fn n8383(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word προσφερω.
pub(crate) fn n8384(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ιρω.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2114);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2115);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3637);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5143);
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
				['ι', s @ ..] => {
					super::push(out, 2116);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3638);
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
				['κ', s @ ..] => {
					super::push(out, 2117);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3639);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5144);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6398);
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
						['σ', s @ ..] => {
							super::push(out, 3640);
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
				['μ', s @ ..] => {
					super::push(out, 2118);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3641);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5145);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6399);
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
				['ν', s @ ..] => {
					super::push(out, 2119);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 2120);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3643);
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
				['π', s @ ..] => {
					super::push(out, 2121);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3644);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3645);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5147);
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
				['σ', s @ ..] => {
					super::push(out, 2122);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2123);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3647);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5148);
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
						['ν', s @ ..] => {
							super::push(out, 3648);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5149);
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
				['χ', s @ ..] => {
					super::push(out, 2124);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3649);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 3650);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5150);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6400);
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
		['ε', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2125);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3651);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3652);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5151);
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
				['γ', s @ ..] => {
					super::push(out, 2127);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3653);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 3654);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 5152);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6401);
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
								['τ', s @ ..] => {
									super::push(out, 5153);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6402);
											match s {
												[] => true,
												['α', s @ ..] => super::t3::n8389(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => super::t3::n2167(s, out),
				['κ', s @ ..] => super::t3::n2168(s, out),
				['π', s @ ..] => super::t3::n2169(s, out),
				['τ', s @ ..] => super::t3::n2170(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υρω.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2135);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2136);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3660);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5156);
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

/// Words ending -σσω.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 925);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2137);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2138);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3662);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5157);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 5158);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6406);
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
						['λ', s @ ..] => {
							super::push(out, 3663);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5159);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6407);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 6408);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7289);
													match s {
														[] => true,
														['σ', s @ ..] => super::t3::n9210(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 6409);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7290);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9211(s, out),
														_ => false,
													}
												}
												['υ', s @ ..] => {
													super::push(out, 7291);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9212(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 6410);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7292);
													match s {
														[] => true,
														['π', s @ ..] => super::t3::n9213(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 6411);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7293);
													match s {
														[] => true,
														['α', s @ ..] => super::t3::n9214(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => super::t3::n3855(s, out),
						['υ', s @ ..] => super::t3::n3856(s, out),
						_ => false,
					}
				}
				['μ', s @ ..] => super::t3::n2180(s, out),
				['ν', s @ ..] => super::t3::n2181(s, out),
				['π', s @ ..] => super::t3::n2182(s, out),
				['ρ', s @ ..] => super::t3::n2183(s, out),
				['τ', s @ ..] => super::t3::n2184(s, out),
				['φ', s @ ..] => super::t3::n2185(s, out),
				['ῥ', s @ ..] => super::t3::n2186(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t3::n927(s, out),
		['η', s @ ..] => super::t3::n928(s, out),
		['ι', s @ ..] => super::t3::n929(s, out),
		['υ', s @ ..] => super::t3::n930(s, out),
		['ω', s @ ..] => super::t3::n931(s, out),
		_ => false,
	}
}

/// Words ending -τεθνηκετω.
pub(crate) fn n8424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7308);
	s.is_empty()
}

/// Words ending -στερηθητω.
pub(crate) fn n8425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7309);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 7809);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πτω.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2168);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 2169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 2170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3717);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3718);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5213);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6443);
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
				['θ', s @ ..] => {
					super::push(out, 2171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3719);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3720);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5214);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6444);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 3721);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5215);
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 6445);
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
				['ι', s @ ..] => {
					super::push(out, 2172);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3723);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 3724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5216);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 5217);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6446);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7310);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9229(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3725);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 3726);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3727);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5219);
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
				['ν', s @ ..] => {
					super::push(out, 2175);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3728);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5220);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 3729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5221);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3730);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5222);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6447);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 6448);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7311);
													match s {
														[] => true,
														['ε', s @ ..] => super::t3::n9230(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 2176);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3731);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2177);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3732);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5224);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6449);
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
						['τ', s @ ..] => {
							super::push(out, 3733);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5225);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6450);
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
				['ῥ', s @ ..] => {
					super::push(out, 2178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['λ', s @ ..] => super::t3::n2220(s, out),
				['π', s @ ..] => {
					super::push(out, 2180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t3::n937(s, out),
		['ι', s @ ..] => super::t3::n938(s, out),
		['μ', s @ ..] => super::t3::n939(s, out),
		['ο', s @ ..] => super::t3::n940(s, out),
		['ρ', s @ ..] => super::t3::n941(s, out),
		['υ', s @ ..] => super::t3::n942(s, out),
		['ω', s @ ..] => super::t3::n943(s, out),
		_ => false,
	}
}

/// Words ending -υναλλαττω.
pub(crate) fn n8465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7335);
	match s {
		[] => true,
		['ξ', '\u{1}', s @ ..] => {
			super::push(out, 7819);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιπαττω.
pub(crate) fn n8466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7336);
	s.is_empty()
}

/// Words ending -ρεττω.
pub(crate) fn n2242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2201);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 3771);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5258);
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

/// Words ending -ηττω.
pub(crate) fn n946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 945);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 2202);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5259);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5260);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6488);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιττω.
pub(crate) fn n947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 946);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 2203);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2204);
			match s {
				[] => true,
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3774);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υττω.
pub(crate) fn n948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 947);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 2205);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2206);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3775);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5262);
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
				['ο', s @ ..] => {
					super::push(out, 3776);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5263);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωττω.
pub(crate) fn n949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 2207);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3777);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5264);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2208);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3778);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5265);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6490);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7338);
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
		_ => false,
	}
}

/// The word συνθραυω.
pub(crate) fn n8469(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αποθραυω.
pub(crate) fn n8470(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ευω.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 961);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2220);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3785);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5271);
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
				['ρ', s @ ..] => {
					super::push(out, 2221);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3786);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5272);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6493);
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
				['σ', s @ ..] => {
					super::push(out, 2222);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3787);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5273);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6494);
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
				['υ', s @ ..] => {
					super::push(out, 2223);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3788);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 5274);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6495);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 7339);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9255(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 962);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 963);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2225);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2226);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3789);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5275);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 6496);
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
						['π', s @ ..] => {
							super::push(out, 3790);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5276);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6497);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7340);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9256(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2227);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3791);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5277);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6498);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7341);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t3::n9257(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2228);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3792);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 3793);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5278);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6499);
											match s {
												[] => true,
												['ε', s @ ..] => super::t3::n8477(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => super::t3::n2270(s, out),
				_ => false,
			}
		}
		['θ', s @ ..] => super::t3::n965(s, out),
		['ι', s @ ..] => super::t3::n966(s, out),
		['κ', s @ ..] => super::t3::n967(s, out),
		['λ', s @ ..] => super::t3::n968(s, out),
		['μ', s @ ..] => super::t3::n969(s, out),
		['ν', s @ ..] => super::t3::n970(s, out),
		['ξ', s @ ..] => super::t3::n971(s, out),
		['π', s @ ..] => super::t3::n972(s, out),
		['ρ', s @ ..] => super::t4::n973(s, out),
		['σ', s @ ..] => super::t4::n974(s, out),
		['τ', s @ ..] => super::t4::n975(s, out),
		['φ', s @ ..] => super::t4::n976(s, out),
		['χ', s @ ..] => super::t4::n977(s, out),
		_ => false,
	}
}

/// Words ending -λυω.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 981);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2284);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2285);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3860);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5344);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5345);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6552);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7370);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9288(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 5346);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6553);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 6554);
											match s {
												[] => true,
												['σ', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 5347);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6555);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7371);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9289(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 6556);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7372);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9290(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 5348);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6557);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7373);
													match s {
														[] => true,
														['π', '\u{1}', s @ ..] => super::t4::n9291(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2286);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3861);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 5349);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6558);
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
								['π', s @ ..] => {
									super::push(out, 5350);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6559);
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
								['τ', s @ ..] => {
									super::push(out, 5351);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6560);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7374);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9292(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2287);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3862);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5352);
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
				['τ', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3863);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5353);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6561);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 6562);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7375);
													match s {
														[] => true,
														['ε', '\u{1}', s @ ..] => super::t4::n9293(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 6563);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7376);
													match s {
														[] => true,
														['π', '\u{1}', s @ ..] => super::t4::n9294(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t4::n986(s, out),
		['κ', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2294);
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
		['λ', s @ ..] => super::t4::n988(s, out),
		['ο', s @ ..] => super::t4::n989(s, out),
		['φ', s @ ..] => {
			super::push(out, 988);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => super::t4::n991(s, out),
		_ => false,
	}
}

/// Words ending -νυω.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 993);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2302);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3873);
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
				['τ', s @ ..] => {
					super::push(out, 2303);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3874);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5362);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 994);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2304);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3875);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5363);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6569);
											match s {
												[] => true,
												['α', 'κ', 'γ', 'ε', '\u{1}', s @ ..] => {
													super::push(out, 7378);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 995);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3876);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5364);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6570);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7379);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3877);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5365);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6571);
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
		['κ', s @ ..] => {
			super::push(out, 996);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2306);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3878);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5366);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 6572);
											match s {
												[] => true,
												['π', 'α', '\u{1}', s @ ..] => {
													super::push(out, 7380);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 997);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3880);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5367);
									match s {
										[] => true,
										['ρ', 'κ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ουω.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3881);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 3882);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5368);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 5369);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6573);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3883);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5370);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6574);
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
		['λ', s @ ..] => {
			super::push(out, 1001);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2311);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3884);
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
		['ρ', s @ ..] => {
			super::push(out, 1002);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2312);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3885);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3886);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5371);
									match s {
										[] => true,
										['α', s @ ..] => {
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
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 3887);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5372);
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 6576);
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
						['κ', s @ ..] => {
							super::push(out, 3888);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5373);
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
						['ο', s @ ..] => {
							super::push(out, 3889);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5374);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6577);
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
								['ρ', s @ ..] => {
									super::push(out, 5375);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6578);
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
						['σ', s @ ..] => {
							super::push(out, 3890);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5376);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6579);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7381);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9296(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3891);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3892);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5377);
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
						['π', s @ ..] => {
							super::push(out, 3893);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5378);
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

/// Words ending -ρυω.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 3894);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5379);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6580);
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
		['ε', s @ ..] => {
			super::push(out, 1005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2317);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3895);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5380);
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
		['κ', s @ ..] => {
			super::push(out, 1006);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2318);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3896);
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
		['τ', s @ ..] => {
			super::push(out, 1007);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σσυω.
pub(crate) fn n1010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1008);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2320);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3897);
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

/// Words ending -τυω.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2321);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3898);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5381);
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
		['ρ', s @ ..] => {
			super::push(out, 1010);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 3899);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5382);
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
		['σ', s @ ..] => {
			super::push(out, 1011);
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

/// Words ending -φυω.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1012);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2323);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3900);
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

/// Words ending -χυω.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2325);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3901);
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
		['σ', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3902);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3903);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5383);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6581);
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
		_ => false,
	}
}

/// The word αναγραφω.
pub(crate) fn n8554(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -παραγραφω.
pub(crate) fn n8555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7382);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -καταγραφω.
pub(crate) fn n8556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7383);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ξυγγραφω.
pub(crate) fn n8557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7384);
	s.is_empty()
}

/// The word συγγραφω.
pub(crate) fn n8558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7385);
	s.is_empty()
}

/// The word επιγραφω.
pub(crate) fn n8559(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -περιγραφω.
pub(crate) fn n8560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7386);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word υπογραφω.
pub(crate) fn n8561(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word προγραφω.
pub(crate) fn n8562(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word εισγραφω.
pub(crate) fn n8563(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ροσγραφω.
pub(crate) fn n7406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6592);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 7387);
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

/// Words ending -εφω.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1019);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2330);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3910);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3911);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5395);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6593);
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
						['ο', s @ ..] => {
							super::push(out, 3912);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5396);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 6594);
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
						['σ', s @ ..] => {
							super::push(out, 3913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5397);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 5398);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6595);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7388);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9301(s, out),
														['ν', s @ ..] => super::t4::n9302(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 6596);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7389);
													match s {
														[] => true,
														['κ', s @ ..] => super::t4::n9303(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 5399);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6597);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7390);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9304(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 5400);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6598);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7391);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9305(s, out),
														_ => false,
													}
												}
												['υ', s @ ..] => {
													super::push(out, 7392);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9306(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3914);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5401);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6599);
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
		_ => false,
	}
}

/// Words ending -ηφω.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 293);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1021);
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

/// Words ending -ιφω.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 294);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2333);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3915);
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
		['ν', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1024);
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

/// Words ending -ρφω.
pub(crate) fn n295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 295);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2334);
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

/// Words ending -υφω.
pub(crate) fn n296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 296);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2335);
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
		['τ', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2336);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2337);
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

/// Words ending -ροσπαρεχω.
pub(crate) fn n8573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7393);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 7842);
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

/// Words ending -ερεχω.
pub(crate) fn n2398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2351);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3926);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 5405);
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

/// Words ending -τρεχω.
pub(crate) fn n2399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2352);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3927);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 3928);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5406);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6601);
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
				['τ', s @ ..] => {
					super::push(out, 5407);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6602);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 7394);
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
		['ι', s @ ..] => {
			super::push(out, 3929);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5408);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6603);
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
				['ρ', s @ ..] => {
					super::push(out, 5409);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6604);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7395);
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
		['σ', s @ ..] => {
			super::push(out, 3930);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5410);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6605);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7396);
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

/// Words ending -σεχω.
pub(crate) fn n1040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1038);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2353);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3931);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5411);
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

/// Words ending -τεχω.
pub(crate) fn n1041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1039);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2354);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3932);
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
		['ε', s @ ..] => {
			super::push(out, 2355);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3933);
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
		['ν', s @ ..] => {
			super::push(out, 2356);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3934);
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

/// Words ending -ηχω.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 300);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2357);
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
		['ψ', s @ ..] => {
			super::push(out, 1041);
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

/// Words ending -ιχω.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 301);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2358);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2359);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3935);
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
				['τ', s @ ..] => {
					super::push(out, 2360);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5412);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 5413);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6606);
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
		_ => false,
	}
}

/// Words ending -ρχω.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2361);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2362);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3937);
							match s {
								[] => true,
								['ξ', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 5414);
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
				['ξ', s @ ..] => {
					super::push(out, 2363);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3938);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2364);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3940);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2365);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3941);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5417);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6607);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7397);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9311(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2366);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3942);
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

/// Words ending -σχω.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1045);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2367);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3943);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5418);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6608);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7398);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9312(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2368);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3944);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5419);
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
				['τ', s @ ..] => {
					super::push(out, 2369);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3945);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5420);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6609);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υχω.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2370);
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
		['ο', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2371);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3946);
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
		['ρ', s @ ..] => {
			super::push(out, 1049);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2373);
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
		['ψ', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2374);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2375);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3948);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5421);
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

/// The word επισταμαι.
pub(crate) fn n8584(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -πεκτιθεμαι.
pub(crate) fn n8585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7400);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 7844);
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

/// Words ending -ακειμαι.
pub(crate) fn n4202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3973);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 5440);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 6623);
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
		['τ', s @ ..] => {
			super::push(out, 5441);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6624);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 7401);
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

/// Words ending -γκειμαι.
pub(crate) fn n4203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3974);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 5442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 5443);
			match s {
				[] => true,
				['ξ', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 6625);
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

/// Words ending -ικειμαι.
pub(crate) fn n4204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3975);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 5444);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 6626);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7402);
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

/// Words ending -οκειμαι.
pub(crate) fn n4205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3976);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 5445);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 6627);
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
		['ρ', s @ ..] => {
			super::push(out, 5446);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 6628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σκειμαι.
pub(crate) fn n4206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3977);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 5447);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 6629);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 7403);
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

/// Words ending -ομαι.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1081);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2406);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 3978);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3979);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3980);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5449);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6630);
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
				['κ', s @ ..] => {
					super::push(out, 2408);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 3981);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5450);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6631);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3982);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5451);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6632);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 3983);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5452);
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
				['λ', s @ ..] => {
					super::push(out, 2409);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3984);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2410);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 3985);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5454);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6633);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2411);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3986);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 5455);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 6634);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7404);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9317(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 3987);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 3988);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5456);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6635);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7405);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 7406);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9318(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2412);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3989);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5457);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6636);
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
				['ρ', s @ ..] => {
					super::push(out, 2413);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3990);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 3991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 5458);
									match s {
										[] => true,
										['τ', s @ ..] => super::t4::n7462(s, out),
										_ => false,
									}
								}
								['ι', s @ ..] => super::t4::n5944(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => super::t4::n2466(s, out),
				['τ', s @ ..] => {
					super::push(out, 2415);
					match s {
						[] => true,
						['κ', s @ ..] => super::t4::n4223(s, out),
						_ => false,
					}
				}
				['χ', s @ ..] => super::t4::n2468(s, out),
				_ => false,
			}
		}
		['β', s @ ..] => super::t4::n1086(s, out),
		['γ', s @ ..] => super::t4::n1087(s, out),
		['δ', s @ ..] => super::t4::n1088(s, out),
		['ε', s @ ..] => super::t4::n1089(s, out),
		['ζ', s @ ..] => super::t4::n1090(s, out),
		['θ', s @ ..] => super::t4::n1091(s, out),
		['ι', s @ ..] => super::t4::n1092(s, out),
		['κ', s @ ..] => super::t4::n1093(s, out),
		['λ', s @ ..] => super::t4::n1094(s, out),
		['μ', s @ ..] => super::t4::n1095(s, out),
		['ν', s @ ..] => super::t4::n1096(s, out),
		['ο', s @ ..] => super::t4::n1097(s, out),
		['π', s @ ..] => super::t4::n1098(s, out),
		['ρ', s @ ..] => super::t4::n1099(s, out),
		['σ', s @ ..] => super::t4::n1100(s, out),
		['τ', s @ ..] => super::t4::n1101(s, out),
		['υ', s @ ..] => super::t4::n1102(s, out),
		['φ', s @ ..] => super::t4::n1103(s, out),
		['χ', s @ ..] => super::t4::n1104(s, out),
		['ω', s @ ..] => super::t4::n1105(s, out),
		_ => false,
	}
}

/// Words ending -υμαι.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2481);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4115);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5618);
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
				['π', s @ ..] => {
					super::push(out, 2483);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4116);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5619);
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
				['ρ', s @ ..] => {
					super::push(out, 2484);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4117);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5620);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 5621);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6779);
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
		_ => false,
	}
}

/// The word παρατιθημι.
pub(crate) fn n9434(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word κατατιθημι.
pub(crate) fn n9435(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -υνεπιτιθημι.
pub(crate) fn n9436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7930);
	match s {
		[] => true,
		['ξ', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['σ', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επεντιθημι.
pub(crate) fn n9437(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ροστιθημι.
pub(crate) fn n7645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6798);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 7535);
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

/// Words ending -αναπιμπλημι.
pub(crate) fn n9439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7932);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word εμπιμπλημι.
pub(crate) fn n9440(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word εμπιμπρημι.
pub(crate) fn n9441(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -εγκαθιστημι.
pub(crate) fn n9442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7933);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8126);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τικαθιστημι.
pub(crate) fn n9443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7934);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 8127);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 8204);
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

/// Words ending -ποκαθιστημι.
pub(crate) fn n9444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7935);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 8128);
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

/// The word περιιστημι.
pub(crate) fn n9445(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -νιστημι.
pub(crate) fn n4383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4149);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5648);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6808);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 6809);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7546);
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
				['τ', s @ ..] => {
					super::push(out, 6810);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 7547);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 7936);
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
		['ε', s @ ..] => {
			super::push(out, 5649);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 5650);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 6812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 6813);
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

/// Words ending -οιστημι.
pub(crate) fn n4385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4151);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 5652);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 6814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7549);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ριστημι.
pub(crate) fn n4386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4152);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5653);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 6815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φιστημι.
pub(crate) fn n4387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4153);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5654);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6816);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 6817);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 7551);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 7937);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 8129);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 7938);
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
		['ε', s @ ..] => {
			super::push(out, 5655);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6818);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 5656);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 5657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6820);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επιπαρειμι.
pub(crate) fn n8775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7553);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεθνηκοιμι.
pub(crate) fn n8776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7554);
	s.is_empty()
}

/// Words ending -αγνυμι.
pub(crate) fn n2573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2521);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 4169);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5669);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6824);
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

/// Words ending -ηγνυμι.
pub(crate) fn n2574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2522);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4170);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 5670);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6825);
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
		['ρ', s @ ..] => {
			super::push(out, 4171);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5671);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6826);
							match s {
								[] => true,
								['ξ', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 7555);
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
		['ῥ', s @ ..] => {
			super::push(out, 4172);
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

/// Words ending -ιγνυμι.
pub(crate) fn n2575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2523);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4173);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6827);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6828);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7556);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7940);
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
						['μ', s @ ..] => {
							super::push(out, 6829);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7557);
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 7941);
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
						['σ', s @ ..] => {
							super::push(out, 6830);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7558);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7942);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 8130);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9973(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 4174);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 5673);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6831);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 7559);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7943);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 8131);
													match s {
														[] => true,
														['ε', s @ ..] => super::t4::n9974(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4175);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5674);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5675);
					match s {
						[] => true,
						['α', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ργνυμι.
pub(crate) fn n2576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2524);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 4176);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5676);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6833);
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

/// Words ending -ευγνυμι.
pub(crate) fn n4411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4177);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 5677);
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

/// Words ending -ινυμι.
pub(crate) fn n1140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1135);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2526);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4178);
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

/// Words ending -κνυμι.
pub(crate) fn n1141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1136);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2527);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4179);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5678);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6834);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 6835);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7560);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7944);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9804(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 6836);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7561);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7945);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9805(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 6837);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7562);
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
								['ο', s @ ..] => {
									super::push(out, 6838);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7563);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7946);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9806(s, out),
														_ => false,
													}
												}
												['υ', s @ ..] => {
													super::push(out, 7947);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9807(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μνυμι.
pub(crate) fn n1142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1137);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4180);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4181);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5679);
							match s {
								[] => true,
								['ξ', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 6839);
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

/// Words ending -ννυμι.
pub(crate) fn n1143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1138);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2529);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4182);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5680);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6840);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7564);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 7948);
													match s {
														[] => true,
														['ι', s @ ..] => super::t4::n9808(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4183);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5681);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6841);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7565);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 7949);
													match s {
														[] => true,
														['π', s @ ..] => super::t4::n9809(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4184);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5682);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6842);
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
				['τ', s @ ..] => {
					super::push(out, 4185);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5683);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6843);
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
		['ε', s @ ..] => {
			super::push(out, 2530);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4186);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 4187);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5684);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6844);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7566);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7950);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9810(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4188);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5685);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6845);
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
		['ω', s @ ..] => {
			super::push(out, 2531);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 4189);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5686);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6846);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7567);
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
				['ρ', s @ ..] => {
					super::push(out, 4190);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5687);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6847);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7568);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7951);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9811(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5688);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6848);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7569);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 4191);
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

/// Words ending -ρνυμι.
pub(crate) fn n1144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1139);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2532);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4192);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4193);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5689);
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
				['τ', s @ ..] => {
					super::push(out, 4194);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5690);
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

/// Words ending -παραδιδωμι.
pub(crate) fn n8801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7570);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μεταδιδωμι.
pub(crate) fn n8802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7571);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιδιδωμι.
pub(crate) fn n8803(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αντιδιδωμι.
pub(crate) fn n8804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7572);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αποδιδωμι.
pub(crate) fn n8805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7573);
	s.is_empty()
}

/// Words ending -ταποδιδωμι.
pub(crate) fn n8806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7574);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 7952);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 8136);
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

/// The word προδιδωμι.
pub(crate) fn n8807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7575);
	s.is_empty()
}

/// Words ending -απροδιδωμι.
pub(crate) fn n8808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7576);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 7953);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 8137);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 8208);
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

/// Words ending -τεθνηκατον.
pub(crate) fn n8823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7590);
	s.is_empty()
}

/// Words ending -τεθνηκετον.
pub(crate) fn n8824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7591);
	s.is_empty()
}

/// Words ending -στερηθητον.
pub(crate) fn n8825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7592);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 7955);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεθνηκητον.
pub(crate) fn n8826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7593);
	s.is_empty()
}

/// Words ending -τερηθειτον.
pub(crate) fn n8827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7594);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 7956);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκοιτον.
pub(crate) fn n8828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7595);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 7957);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γεγονοιτον.
pub(crate) fn n8829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7596);
	s.is_empty()
}

/// Words ending -αταμειδιαω.
pub(crate) fn n8838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7603);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 7959);
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

/// The word πασχητιαω.
pub(crate) fn n8839(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word υπερνικαω.
pub(crate) fn n8840(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word προσδοκαω.
pub(crate) fn n8841(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word καταγελαω.
pub(crate) fn n8842(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -κατακοιμαω.
pub(crate) fn n8843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7604);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word απαγχοναω.
pub(crate) fn n8844(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word εκλιστραω.
pub(crate) fn n8845(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word καταφωραω.
pub(crate) fn n8846(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επιφοιταω.
pub(crate) fn n8847(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -προαπανταω.
pub(crate) fn n8848(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7605);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word εγκαταλεγω.
pub(crate) fn n9483(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word κατεπειγω.
pub(crate) fn n8851(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word καταφευγω.
pub(crate) fn n8852(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word υπεκφευγω.
pub(crate) fn n8853(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ραιοπραγεω.
pub(crate) fn n8854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7608);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 7960);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 8138);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -παδοφεγγεω.
pub(crate) fn n8855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7609);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 7961);
			match s {
				[] => true,
				['α', 'λ', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επιχορηγεω.
pub(crate) fn n8856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7610);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word στρατηγεω.
pub(crate) fn n8857(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ογεω.
pub(crate) fn n540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1391);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2764);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4371);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5807);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6920);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 7611);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9487(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 4372);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5808);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6921);
											match s {
												[] => true,
												['β', s @ ..] => {
													super::push(out, 7612);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9488(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2765);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 4373);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 5809);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6922);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 5810);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6923);
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
						['μ', s @ ..] => {
							super::push(out, 4374);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5811);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6924);
											s.is_empty()
										}
										['ξ', s @ ..] => {
											super::push(out, 6925);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7613);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9489(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 4375);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 5812);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6926);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7614);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9490(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4376);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5813);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 6927);
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
						['τ', s @ ..] => {
							super::push(out, 4377);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5814);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6928);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7615);
													match s {
														[] => true,
														['τ', s @ ..] => super::t4::n9491(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 5815);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6929);
											match s {
												[] => true,
												['β', s @ ..] => {
													super::push(out, 7616);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9492(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 4378);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 5816);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 6930);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7617);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9493(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 4379);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5817);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6931);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7618);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9494(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => super::t4::n2864(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1392);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t4::n2865(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ργεω.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1393);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2768);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2769);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4381);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5819);
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
		['ε', s @ ..] => {
			super::push(out, 1394);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2770);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4382);
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
		['υ', s @ ..] => {
			super::push(out, 1395);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2771);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4383);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5820);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 6932);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 7619);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9495(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 4384);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5821);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6933);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['γ', s @ ..] => {
													super::push(out, 7620);
													match s {
														[] => true,
														['υ', s @ ..] => super::t4::n9496(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 4385);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5822);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6934);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4386);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5823);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6935);
											match s {
												[] => true,
												['χ', s @ ..] => {
													super::push(out, 7621);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9497(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 5824);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6936);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7622);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9498(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 4387);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5825);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6937);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7623);
													match s {
														[] => true,
														['τ', s @ ..] => super::t4::n9499(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['η', s @ ..] => {
									super::push(out, 5826);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6938);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7624);
													match s {
														[] => true,
														['ε', s @ ..] => super::t4::n9500(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 6939);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7625);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9501(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 5827);
									match s {
										[] => true,
										['ε', s @ ..] => super::t4::n7834(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υγεω.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1396);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2772);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4388);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 5828);
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
		['ζ', s @ ..] => {
			super::push(out, 1397);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2773);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4389);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5829);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6941);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7627);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9503(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2774);
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

/// Words ending -ωγεω.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2775);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4390);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5830);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6942);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7628);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9504(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word φιλομαθεω.
pub(crate) fn n8879(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word κακοπαθεω.
pub(crate) fn n8880(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word προσπαθεω.
pub(crate) fn n8881(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -παραβοηθεω.
pub(crate) fn n8882(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7629);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επιβοηθεω.
pub(crate) fn n8883(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -προσβοηθεω.
pub(crate) fn n8884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7630);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μυθεω.
pub(crate) fn n1444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1427);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2800);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 4408);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5841);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6949);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7631);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7968);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9823(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ουθεω.
pub(crate) fn n1445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1428);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 2801);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4409);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5842);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6950);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ξ', s @ ..] => {
											super::push(out, 7632);
											match s {
												[] => true,
												['ε', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οχθεω.
pub(crate) fn n1446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1429);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 2802);
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

/// Words ending -ωθεω.
pub(crate) fn n565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 565);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1430);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1431);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2803);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4410);
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
		['π', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2804);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word περιποιεω.
pub(crate) fn n8888(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word αντιποιεω.
pub(crate) fn n8889(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αγαθοποιεω.
pub(crate) fn n8890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7633);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7969);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ψιλοποιεω.
pub(crate) fn n8891(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -γματοποιεω.
pub(crate) fn n8892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7634);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 7970);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 8140);
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

/// Words ending -ρρητοποιεω.
pub(crate) fn n8893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7635);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 7971);
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

/// Words ending -λωττοποιεω.
pub(crate) fn n8894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7636);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 7972);
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

/// Words ending -οσχοποιεω.
pub(crate) fn n7855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6958);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 7637);
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

/// Words ending -ωοποιεω.
pub(crate) fn n4702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4419);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 5851);
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

/// Words ending -σποιεω.
pub(crate) fn n2911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2809);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 4420);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 5852);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6959);
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

/// Words ending -κομπολακεω.
pub(crate) fn n8897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7638);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λιοφυλακεω.
pub(crate) fn n8898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7639);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 7973);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 8141);
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

/// Words ending -ιχοφυλακεω.
pub(crate) fn n8899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7640);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 7974);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 8142);
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

/// The word συναδικεω.
pub(crate) fn n8900(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word φιλοδικεω.
pub(crate) fn n8901(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -οικεω.
pub(crate) fn n1455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1438);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2816);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2817);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4427);
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
		['ν', s @ ..] => {
			super::push(out, 2818);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4428);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4429);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5858);
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
		['ρ', s @ ..] => {
			super::push(out, 2819);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4430);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5859);
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
		['σ', s @ ..] => {
			super::push(out, 2820);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4431);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5860);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6965);
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
		['τ', s @ ..] => {
			super::push(out, 2821);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4432);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5861);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6966);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 6967);
									match s {
										[] => true,
										['ε', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['υ', 'σ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 6968);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7641);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7975);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9829(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', 'ρ', 'π', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['σ', 'ι', 'ε', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ευδοκεω.
pub(crate) fn n4717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4433);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τοκεω.
pub(crate) fn n1457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1440);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2824);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 4434);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5862);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6969);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7642);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7976);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9830(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρκεω.
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 571);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2825);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2826);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4435);
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
		['ο', s @ ..] => {
			super::push(out, 1442);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2827);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4436);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5863);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6970);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['κ', s @ ..] => {
													super::push(out, 7643);
													match s {
														[] => true,
														['ε', s @ ..] => super::t4::n9519(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σκεω.
pub(crate) fn n572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 572);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1443);
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

/// The word παρακαλεω.
pub(crate) fn n8909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7644);
	s.is_empty()
}

/// Words ending -σπαρακαλεω.
pub(crate) fn n8910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7645);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 7978);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 8144);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 8209);
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

/// The word κατακαλεω.
pub(crate) fn n8911(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word μετακαλεω.
pub(crate) fn n8912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7646);
	s.is_empty()
}

/// The word προσκαλεω.
pub(crate) fn n8913(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ελεω.
pub(crate) fn n574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 574);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1447);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4446);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4447);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5872);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6976);
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
				['η', s @ ..] => {
					super::push(out, 2836);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4448);
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
		['τ', s @ ..] => {
			super::push(out, 1448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2837);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2838);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4449);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5873);
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
				['ι', s @ ..] => {
					super::push(out, 2839);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4450);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5874);
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
						['σ', s @ ..] => {
							super::push(out, 4451);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 5875);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6977);
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
				['κ', s @ ..] => {
					super::push(out, 2840);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4452);
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
				['ο', s @ ..] => {
					super::push(out, 2841);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4453);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5876);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 5877);
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
		['φ', s @ ..] => {
			super::push(out, 1449);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 2842);
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

/// The word απειλεω.
pub(crate) fn n6445(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -μιλεω.
pub(crate) fn n1470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1453);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2846);
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

/// Words ending -πιλεω.
pub(crate) fn n1471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1454);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φιλεω.
pub(crate) fn n1472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1455);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2847);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κλεω.
pub(crate) fn n578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ολεω.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 579);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1456);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2848);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4455);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5878);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6978);
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
				['ο', s @ ..] => {
					super::push(out, 2849);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 4456);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 5879);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6979);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7647);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9521(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1457);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2850);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4457);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5880);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 5881);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6980);
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
		_ => false,
	}
}

/// Words ending -πλεω.
pub(crate) fn n580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 580);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1458);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2851);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4458);
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
				['ρ', s @ ..] => {
					super::push(out, 2852);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4459);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5882);
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
				['τ', s @ ..] => {
					super::push(out, 2853);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4460);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5883);
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
		['ι', s @ ..] => {
			super::push(out, 1460);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2854);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4461);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 5884);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6981);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7648);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9522(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2855);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4462);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5885);
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
		['κ', s @ ..] => {
			super::push(out, 1461);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4463);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5886);
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
		['μ', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2857);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 4464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5887);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 4465);
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
		['ο', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2858);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4466);
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
		['σ', s @ ..] => {
			super::push(out, 1464);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2860);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4467);
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
				['ο', s @ ..] => {
					super::push(out, 2861);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4468);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5888);
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

/// Words ending -τλεω.
pub(crate) fn n581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 581);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1465);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2862);
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

/// Words ending -υλεω.
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 582);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1466);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2863);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4469);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5889);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6982);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φλεω.
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 583);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1467);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χλεω.
pub(crate) fn n584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 584);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1468);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2864);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4470);
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

/// Words ending -ωλεω.
pub(crate) fn n585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 585);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1470);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2866);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αταπολεμεω.
pub(crate) fn n8920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7649);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 7979);
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

/// Words ending -ντιπολεμεω.
pub(crate) fn n8921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7650);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 7980);
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

/// Words ending -ξυμπολεμεω.
pub(crate) fn n8922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7651);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συμπολεμεω.
pub(crate) fn n8923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7652);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word βλασφημεω.
pub(crate) fn n8924(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επευφημεω.
pub(crate) fn n8925(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ομεω.
pub(crate) fn n590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1477);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2873);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4475);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5896);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6988);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7653);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 7654);
													match s {
														[] => true,
														['δ', s @ ..] => super::t4::n9527(s, out),
														_ => false,
													}
												}
												['ν', s @ ..] => {
													super::push(out, 7655);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9528(s, out),
														['υ', s @ ..] => super::t4::n9529(s, out),
														_ => false,
													}
												}
												['τ', s @ ..] => {
													super::push(out, 7656);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9530(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1478);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2874);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2875);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4476);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5897);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6989);
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
		['ν', s @ ..] => {
			super::push(out, 1479);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2876);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4477);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 5898);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 6990);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7657);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9531(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2877);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4478);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 5899);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6991);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7658);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 7659);
													match s {
														[] => true,
														['α', s @ ..] => super::t4::n9532(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4479);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 5900);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6992);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7660);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9533(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1480);
			match s {
				[] => true,
				['δ', s @ ..] => super::t4::n2987(s, out),
				['τ', s @ ..] => {
					super::push(out, 2879);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => super::t4::n1499(s, out),
		_ => false,
	}
}

/// Words ending -κοσμεω.
pub(crate) fn n2994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2885);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 4486);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5906);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6996);
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
				['ν', s @ ..] => {
					super::push(out, 5907);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6997);
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

/// Words ending -υμεω.
pub(crate) fn n593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 593);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1485);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2886);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2887);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4487);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5908);
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

/// Words ending -ακωνομανεω.
pub(crate) fn n8940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7664);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 7989);
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

/// The word εξασθενεω.
pub(crate) fn n8941(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ονεω.
pub(crate) fn n602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 602);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1499);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2904);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 4499);
							match s {
								[] => true,
								['ζ', s @ ..] => {
									super::push(out, 5917);
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
		['δ', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1501);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2906);
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
		['κ', s @ ..] => {
			super::push(out, 1502);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2907);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4500);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 5918);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7000);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1503);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1504);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2909);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4502);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5919);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 7001);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7665);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9538(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1505);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2910);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2911);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4503);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5920);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7002);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7666);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t4::n9539(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2912);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4504);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 5921);
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
		['ρ', s @ ..] => {
			super::push(out, 1506);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2913);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4505);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 4506);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 5922);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7003);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7667);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9540(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 4507);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5923);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7004);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7668);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n9541(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 4508);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5924);
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
		['τ', s @ ..] => {
			super::push(out, 1507);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2914);
					match s {
						[] => true,
						['ο', s @ ..] => super::t4::n4802(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => super::t4::n1526(s, out),
		['χ', s @ ..] => super::t4::n1527(s, out),
		_ => false,
	}
}

/// Words ending -πνεω.
pub(crate) fn n603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1510);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1511);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2917);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4512);
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
				['ν', s @ ..] => {
					super::push(out, 2918);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4513);
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
		['ι', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2919);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4514);
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
		['μ', s @ ..] => {
			super::push(out, 1513);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2920);
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
		['σ', s @ ..] => {
			super::push(out, 1514);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2921);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4515);
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
		['υ', s @ ..] => {
			super::push(out, 1515);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2922);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4516);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5931);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7012);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υνεω.
pub(crate) fn n604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1516);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2923);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4517);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5932);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7013);
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
		_ => false,
	}
}

/// Words ending -εχνεω.
pub(crate) fn n1535(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1517);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 2924);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4518);
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

/// Words ending -ωνεω.
pub(crate) fn n606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 606);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1518);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2925);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4519);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5933);
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
		['φ', s @ ..] => {
			super::push(out, 1519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2926);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2927);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4520);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 5934);
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
				['σ', s @ ..] => {
					super::push(out, 2928);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4521);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5935);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7014);
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
		['ψ', s @ ..] => {
			super::push(out, 1520);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2929);
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

/// The word διασκοπεω.
pub(crate) fn n8955(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word επισκοπεω.
pub(crate) fn n8956(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -περισκοπεω.
pub(crate) fn n8957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7674);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word θεοπροπεω.
pub(crate) fn n8958(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ισορροπεω.
pub(crate) fn n8959(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ποδοκτυπεω.
pub(crate) fn n8960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7675);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ενανθρωπεω.
pub(crate) fn n8961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7676);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αποστερεω.
pub(crate) fn n8962(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word λαγοθηρεω.
pub(crate) fn n8963(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ναυκληρεω.
pub(crate) fn n8964(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -αιρεω.
pub(crate) fn n1566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1546);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2956);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 2957);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4548);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5954);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 7025);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7677);
											match s {
												[] => true,
												['ξ', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['σ', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2958);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4549);
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
		['ν', s @ ..] => {
			super::push(out, 2959);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4550);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4551);
					match s {
						[] => true,
						['ξ', '\u{1}', s @ ..] => {
							super::push(out, 5956);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5957);
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
		['ξ', s @ ..] => {
			super::push(out, 2960);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 2961);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5959);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4554);
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

/// Words ending -σειρεω.
pub(crate) fn n3075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2962);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χειρεω.
pub(crate) fn n3076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2963);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 4555);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5960);
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
		['ι', s @ ..] => {
			super::push(out, 4556);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 5961);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 7026);
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

/// Words ending -κρεω.
pub(crate) fn n622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 622);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2964);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

