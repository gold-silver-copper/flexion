use flexion::Path;

/// Words ending -чий.
pub(crate) fn n2162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2156);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10324);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10325);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29589);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 29590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10326);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10327);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29592);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56884);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84682);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106755);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 56885);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84683);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106756);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121726);
													match s {
														[] => true,
														['г', s @ ..] => super::t34::n132265(s, out),
														_ => false,
													}
												}
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
							super::push(out, 56886);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84684);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106757);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 56887);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84685);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106758);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 84686);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106759);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 56888);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84687);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106760);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121727);
													s.is_empty()
												}
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
							super::push(out, 56889);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84688);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106761);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 56890);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 84689);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106762);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 56891);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84690);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106763);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121728);
													match s {
														[] => true,
														['я', s @ ..] => super::t34::n132266(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 106764);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121729);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10328);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29593);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10329);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29594);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10330);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 29595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10331);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 29596);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ший.
pub(crate) fn n2163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2157);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10332);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29597);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56895);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84691);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106765);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10333);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29598);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56896);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84692);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106766);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121730);
													match s {
														[] => true,
														['с', s @ ..] => super::t34::n132267(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29599);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 29600);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10334);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10335);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10336);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29602);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10337);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10338);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29603);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 29604);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56899);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84693);
									s.is_empty()
								}
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
			super::push(out, 10339);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щий.
pub(crate) fn n2164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2158);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10340);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 29605);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 56900);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84694);
									match s {
										[] => true,
										['с', 'о', s @ ..] => {
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121731);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 121732);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132268(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10341);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10342);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29607);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56901);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29608);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56902);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84696);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106767);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121733);
													match s {
														[] => true,
														['в', s @ ..] => super::t34::n132269(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29609);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56903);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84697);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106768);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10343);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29610);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56904);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 84698);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106769);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121734);
													match s {
														[] => true,
														['в', s @ ..] => super::t34::n132270(s, out),
														_ => false,
													}
												}
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
							super::push(out, 56905);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 56906);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 84699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 56907);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 56908);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 56909);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84700);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106770);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121735);
													match s {
														[] => true,
														['в', s @ ..] => super::t34::n132271(s, out),
														_ => false,
													}
												}
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
							super::push(out, 56910);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 56911);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84701);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106771);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 121736);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132272(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29611);
					match s {
						[] => true,
						['в', s @ ..] => super::t34::n57494(s, out),
						['д', s @ ..] => super::t34::n57495(s, out),
						['к', s @ ..] => super::t34::n57496(s, out),
						['р', s @ ..] => super::t34::n57497(s, out),
						['ц', 'н', 'а', 'т', '\u{1}', s @ ..] => super::t34::n57498(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t34::n29819(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t34::n10374(s, out),
		_ => false,
	}
}

/// Words ending -кобой.
pub(crate) fn n29843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29636);
	s.is_empty()
}

/// Words ending -лобой.
pub(crate) fn n29844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29637);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 56936);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84717);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 106780);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нобой.
pub(crate) fn n29845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29638);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 56937);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 84718);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 106781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 56938);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 84719);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 106782);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -робой.
pub(crate) fn n29846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29639);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 56939);
			match s {
				[] => true,
				['в', 'з', '\u{1}', s @ ..] => {
					super::push(out, 84720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 56940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тобой.
pub(crate) fn n29847(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29640);
	match s {
		[] => true,
		['и', 'к', '\u{1}', s @ ..] => {
			super::push(out, 56941);
			s.is_empty()
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 84721);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 106783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 84722);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 106784);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 121745);
									s.is_empty()
								}
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
			super::push(out, 56942);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 84723);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 106785);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сбой.
pub(crate) fn n10388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10358);
	s.is_empty()
}

/// Words ending -тбой.
pub(crate) fn n10389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10359);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29641);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 56943);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 84724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 29642);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 29643);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 56944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 84725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убой.
pub(crate) fn n10390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10360);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29644);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 56945);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 84726);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 106786);
									s.is_empty()
								}
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
			super::push(out, 29645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыбой.
pub(crate) fn n10391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10361);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 29646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 56946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вой.
pub(crate) fn n2167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2161);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10362);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10363);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29647);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29648);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10364);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29649);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56948);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29650);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10365);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29651);
					match s {
						[] => true,
						['у', 'р', 'о', s @ ..] => {
							super::push(out, 56950);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84728);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106787);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121746);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29652);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56951);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84729);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106788);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10366);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29653);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56952);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10367);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29654);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56953);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10368);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29655);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 56954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10369);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29656);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 29657);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 29658);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56955);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84730);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106789);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121747);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29660);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56957);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84731);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106790);
											s.is_empty()
										}
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
					super::push(out, 29661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10370);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 10371);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29663);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гой.
pub(crate) fn n2168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2162);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10372);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10373);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29665);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 56959);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84732);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106791);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121748);
													match s {
														[] => true,
														['р', s @ ..] => super::t34::n132279(s, out),
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
									super::push(out, 84733);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106792);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121749);
													match s {
														[] => true,
														['с', s @ ..] => super::t34::n132280(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10374);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29666);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56960);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84734);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106793);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10375);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дой.
pub(crate) fn n2169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2163);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10376);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10377);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29669);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 56961);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29670);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 56962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10378);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10379);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29672);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 29673);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56963);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84736);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29674);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 56964);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84737);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106794);
											s.is_empty()
										}
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
					super::push(out, 29675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10380);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 29676);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жой.
pub(crate) fn n2170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2164);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 10381);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 29677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зой.
pub(crate) fn n2171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2165);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10382);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29678);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56966);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84738);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106795);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121750);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10383);
			match s {
				[] => true,
				['б', 'е', 'м', 'а', '\u{1}', s @ ..] => {
					super::push(out, 29679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кой.
pub(crate) fn n2172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2166);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10384);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29680);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56967);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84739);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106796);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10385);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29681);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 56968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10386);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10387);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29682);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 29683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56970);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 56971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10388);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29684);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56972);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 29685);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56973);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84741);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106797);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121751);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132281(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29686);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56974);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84742);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лой.
pub(crate) fn n2173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2167);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10389);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10390);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 29688);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 56975);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84743);
									s.is_empty()
								}
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
			super::push(out, 10391);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29689);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10392);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29690);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 29691);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 56977);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84744);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106798);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121752);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29692);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56978);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 29693);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 56979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29694);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56980);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84746);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106799);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 56981);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84747);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 56982);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84748);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106800);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 56983);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84749);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 106801);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мой.
pub(crate) fn n2174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2168);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10393);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29695);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 56984);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 84750);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106802);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10394);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29696);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 56985);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29697);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 56986);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 56987);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29698);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56988);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84753);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106803);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10395);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29699);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 56989);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84754);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106804);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 121753);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ной.
pub(crate) fn n2175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2169);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10396);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10397);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 29700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 56990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10398);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 10399);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29701);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 29702);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 56991);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84755);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106805);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121754);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 84756);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106806);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121755);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10400);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29703);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 29704);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 56992);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84757);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106807);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121756);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29705);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 56993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29706);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 56994);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 56995);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84759);
									s.is_empty()
								}
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
			super::push(out, 10401);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29707);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 56996);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 84760);
									s.is_empty()
								}
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
			super::push(out, 10402);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10403);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29709);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 29710);
					match s {
						[] => true,
						['л', 'г', '\u{1}', s @ ..] => {
							super::push(out, 56997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29711);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 56998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 29712);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 56999);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84761);
									s.is_empty()
								}
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
			super::push(out, 10404);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 29713);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57000);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84762);
									s.is_empty()
								}
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
			super::push(out, 10405);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10406);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29715);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57001);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84763);
									s.is_empty()
								}
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
			super::push(out, 10407);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 29716);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57002);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84764);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106808);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10408);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 10409);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29717);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57003);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 29718);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57004);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84765);
									s.is_empty()
								}
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
			super::push(out, 10410);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29719);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29720);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57006);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84766);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106809);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121757);
													match s {
														[] => true,
														['ч', s @ ..] => super::t34::n132282(s, out),
														_ => false,
													}
												}
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
							super::push(out, 57007);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29721);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10411);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29722);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10412);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 29723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10413);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29724);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57010);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84768);
									s.is_empty()
								}
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
			super::push(out, 10414);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29725);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57011);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пой.
pub(crate) fn n2176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2170);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10415);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 29726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10416);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29727);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57012);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84770);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10417);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29729);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57013);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10418);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29730);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29731);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57015);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10419);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29732);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рой.
pub(crate) fn n2177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2171);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10420);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10421);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29733);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 57017);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84771);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106810);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121758);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132283(s, out),
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
									super::push(out, 84772);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106811);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121759);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 84773);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106812);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121760);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132284(s, out),
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
									super::push(out, 84774);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106813);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121761);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132285(s, out),
														_ => false,
													}
												}
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
							super::push(out, 57018);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 57019);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 57020);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 29734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10422);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10423);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29735);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57021);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84777);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106814);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10424);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10425);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57022);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 57023);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84778);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106815);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 57024);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 57025);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84779);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106816);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121762);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word косой.
pub(crate) fn n2178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2172);
	s.is_empty()
}

/// Words ending -той.
pub(crate) fn n2179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2173);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10426);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10427);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29739);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57027);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 84780);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106817);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10428);
			match s {
				[] => true,
				['о', 'л', s @ ..] => {
					super::push(out, 29740);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57028);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 84781);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106818);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10429);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29741);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29742);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57030);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уой.
pub(crate) fn n2180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2174);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 10430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29743);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хой.
pub(crate) fn n2181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2175);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 10431);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29744);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57031);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чой.
pub(crate) fn n2182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2176);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 10432);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29745);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шой.
pub(crate) fn n2183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2177);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 10433);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29746);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57032);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84784);
									s.is_empty()
								}
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
			super::push(out, 10434);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29747);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57033);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яой.
pub(crate) fn n2184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2178);
	s.is_empty()
}

/// Words ending -уй-перехуй.
pub(crate) fn n132286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130929);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 136279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оранжевый.
pub(crate) fn n123040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121765);
	s.is_empty()
}

/// Words ending -хрублевый.
pub(crate) fn n123041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121766);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 130930);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 136280);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 139422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оричневый.
pub(crate) fn n123042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121767);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 130931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -непутевый.
pub(crate) fn n123043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121768);
	s.is_empty()
}

/// Words ending -чивый.
pub(crate) fn n29994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29787);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 57065);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 84810);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 106834);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121769);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 130932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шивый.
pub(crate) fn n29995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29788);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 57066);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 84811);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овый.
pub(crate) fn n10502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10472);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 29789);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 57067);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 84812);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 106835);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121770);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 130933);
													match s {
														[] => true,
														['у', s @ ..] => super::t34::n137692(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 57068);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84813);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 106836);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121771);
											s.is_empty()
										}
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
					super::push(out, 57069);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84814);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106837);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121772);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 130934);
													match s {
														[] => true,
														['п', s @ ..] => super::t34::n137693(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29790);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 57070);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 84815);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106838);
									s.is_empty()
								}
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
			super::push(out, 29791);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 57071);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 84816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 29792);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 57072);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 84817);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106839);
									s.is_empty()
								}
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
			super::push(out, 29793);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 57073);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 84818);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 106840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 84819);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 106841);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121773);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 130935);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 29794);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 57074);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 84820);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 84821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 57075);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 84822);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106842);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121774);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130936);
													match s {
														[] => true,
														['л', s @ ..] => super::t34::n137694(s, out),
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
									super::push(out, 106843);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 121775);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 130937);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n137695(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твый.
pub(crate) fn n10503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10473);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 29795);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 57076);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 84823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явый.
pub(crate) fn n10504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10474);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29796);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 29797);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дый.
pub(crate) fn n2201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2195);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10475);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29798);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10476);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29799);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азый.
pub(crate) fn n10507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10477);
	s.is_empty()
}

/// Words ending -лый.
pub(crate) fn n2203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2197);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10478);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10479);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10480);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29802);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29803);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57080);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84824);
									s.is_empty()
								}
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
			super::push(out, 10481);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10482);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29805);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мый.
pub(crate) fn n2204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2198);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 10483);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29806);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57081);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84825);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106844);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 121776);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 84826);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106845);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 121777);
													match s {
														[] => true,
														['д', s @ ..] => super::t34::n132295(s, out),
														_ => false,
													}
												}
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
							super::push(out, 57082);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84827);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106846);
											s.is_empty()
										}
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
					super::push(out, 29807);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57083);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84828);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106847);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121778);
													s.is_empty()
												}
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
							super::push(out, 57084);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 84829);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106848);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121779);
													match s {
														[] => true,
														['и', s @ ..] => super::t34::n132296(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29808);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57085);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84830);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106849);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121780);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10484);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29809);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10485);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29810);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29811);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57088);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ный.
pub(crate) fn n2205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2199);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10486);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29812);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57089);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84832);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 84833);
									s.is_empty()
								}
								['с', 'о', s @ ..] => {
									super::push(out, 84834);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106850);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121781);
													match s {
														[] => true,
														['р', s @ ..] => super::t34::n132297(s, out),
														_ => false,
													}
												}
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
							super::push(out, 57090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29813);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57091);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 57092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10487);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29814);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57093);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 57094);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84835);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106851);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121782);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132298(s, out),
														_ => false,
													}
												}
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
							super::push(out, 57095);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 29815);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57096);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84837);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106852);
											s.is_empty()
										}
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
					super::push(out, 29816);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57097);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84838);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106853);
											s.is_empty()
										}
										['х', s @ ..] => {
											super::push(out, 106854);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 57098);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29817);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57099);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84840);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106855);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 121783);
													match s {
														[] => true,
														['л', s @ ..] => super::t34::n132299(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29818);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57100);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 84841);
									s.is_empty()
								}
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
			super::push(out, 10488);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29819);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57101);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 84842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 29820);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57102);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 84843);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106856);
											s.is_empty()
										}
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
					super::push(out, 29821);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 10489);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29822);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57104);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84844);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106857);
											s.is_empty()
										}
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
					super::push(out, 29823);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57105);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29824);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57106);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84846);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106858);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121784);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29825);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 57107);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 29826);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57108);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 84848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => super::t34::n30034(s, out),
				['я', s @ ..] => super::t34::n30035(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t34::n10520(s, out),
		['й', s @ ..] => super::t34::n10521(s, out),
		['м', s @ ..] => super::t34::n10522(s, out),
		['н', s @ ..] => super::t34::n10523(s, out),
		['р', s @ ..] => super::t34::n10524(s, out),
		['с', s @ ..] => super::t34::n10525(s, out),
		['т', s @ ..] => super::t34::n10526(s, out),
		['ч', s @ ..] => super::t34::n10527(s, out),
		['ш', s @ ..] => super::t34::n10528(s, out),
		['ь', s @ ..] => super::t34::n10529(s, out),
		['я', s @ ..] => super::t34::n10530(s, out),
		_ => false,
	}
}

/// Words ending -пый.
pub(crate) fn n2206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2200);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 10501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рый.
pub(crate) fn n2207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2201);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10502);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29864);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10503);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10504);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29866);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сый.
pub(crate) fn n2208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2202);
	s.is_empty()
}

/// Words ending -тый.
pub(crate) fn n2209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2203);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10505);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 29867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57174);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 57175);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84915);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106920);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121832);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 29868);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 29869);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57176);
							match s {
								[] => true,
								['в', 'о', s @ ..] => {
									super::push(out, 84916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 29870);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 29871);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57177);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84917);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106921);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121833);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10506);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29872);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 57178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29873);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57179);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84919);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106922);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121834);
													match s {
														[] => true,
														['ш', s @ ..] => super::t34::n132331(s, out),
														_ => false,
													}
												}
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
							super::push(out, 57180);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 106923);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 106924);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10507);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29874);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10508);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10509);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29876);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57182);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хый.
pub(crate) fn n2210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2204);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 10510);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 29877);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цый.
pub(crate) fn n2211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2205);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 10511);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29878);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57184);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84920);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106925);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121835);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10512);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29879);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -издоквак.
pub(crate) fn n108066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106929);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 121836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лвак.
pub(crate) fn n10579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10549);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29920);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 57210);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овак.
pub(crate) fn n10580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10550);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 29921);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 57211);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рвак.
pub(crate) fn n10581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10551);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 29922);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 57212);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 57213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 29923);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 57214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 84936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -увак.
pub(crate) fn n10582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10552);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 29924);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гак.
pub(crate) fn n2227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2221);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 10553);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29925);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10554);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57216);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дак.
pub(crate) fn n2228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2222);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10555);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10556);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29927);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29928);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57218);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84937);
									s.is_empty()
								}
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
			super::push(out, 10557);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29929);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57219);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 10558);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29930);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10559);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29931);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10560);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29932);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 29933);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57222);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84938);
									s.is_empty()
								}
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
			super::push(out, 10561);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29934);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57223);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 57224);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84940);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106930);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 57225);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84941);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 57226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10562);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29935);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57227);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10563);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29936);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 29937);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10564);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29938);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10565);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 29940);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10566);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29941);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57232);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 57233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 29942);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57234);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 57235);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84942);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106931);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121837);
													s.is_empty()
												}
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
							super::push(out, 57236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10567);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 29943);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 29944);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 29945);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 29946);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 29947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жак.
pub(crate) fn n2229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2223);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10568);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 10569);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29948);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57238);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84943);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 57239);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 84944);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106932);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 121838);
													match s {
														[] => true,
														['т', s @ ..] => super::t34::n132332(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 29949);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57240);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84945);
									s.is_empty()
								}
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
			super::push(out, 10570);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 29950);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 29951);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 29952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10571);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 29953);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 29954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10572);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29955);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57242);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 57243);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84946);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 29956);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57244);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29957);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57245);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10573);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 29958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10574);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зак.
pub(crate) fn n2230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2224);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10576);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29960);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57247);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 57248);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84947);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106933);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121839);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10577);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10578);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 29963);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10579);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 29964);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10580);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29965);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57251);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84948);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106934);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10581);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29966);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 29967);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57253);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84949);
									s.is_empty()
								}
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
			super::push(out, 10582);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 29968);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57254);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84950);
									s.is_empty()
								}
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
			super::push(out, 10583);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 29969);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10584);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 29970);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57256);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 84951);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иак.
pub(crate) fn n2231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2225);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10585);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29971);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 57257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10586);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10587);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29973);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -как.
pub(crate) fn n2232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2226);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10588);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 29974);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10589);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29975);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57260);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лак.
pub(crate) fn n2233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2227);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10590);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 10591);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 29976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 10592);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29977);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57261);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84953);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 106935);
											s.is_empty()
										}
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
					super::push(out, 29978);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57262);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84954);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106936);
											s.is_empty()
										}
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
					super::push(out, 29979);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57263);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 29980);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57264);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 84956);
									s.is_empty()
								}
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
			super::push(out, 10593);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10594);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10595);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 10596);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29982);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57266);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84957);
									s.is_empty()
								}
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
			super::push(out, 10597);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29983);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57267);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84958);
									s.is_empty()
								}
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
			super::push(out, 10598);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10599);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29985);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57268);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84959);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106937);
											s.is_empty()
										}
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
					super::push(out, 29986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10600);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10601);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 29988);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57269);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84960);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 29989);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10602);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 29990);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57271);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84961);
									s.is_empty()
								}
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
			super::push(out, 10603);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29991);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57272);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84962);
									s.is_empty()
								}
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
			super::push(out, 10604);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 29992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57273);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 57274);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84963);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106938);
											s.is_empty()
										}
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
					super::push(out, 29993);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57275);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84964);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106939);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10605);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 29994);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57276);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84965);
									s.is_empty()
								}
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
			super::push(out, 10606);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 29995);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57277);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 84966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 57278);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84967);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 29996);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 29997);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мак.
pub(crate) fn n2234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2228);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10607);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10608);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 29998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57279);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 57280);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84968);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106940);
											s.is_empty()
										}
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
					super::push(out, 29999);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 57281);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84969);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106941);
											s.is_empty()
										}
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
					super::push(out, 30000);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57282);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 84970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30001);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57283);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30002);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57284);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84972);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106942);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121840);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10609);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30003);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 10610);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10611);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30004);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57286);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84973);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 57287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10612);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57288);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 57289);
							s.is_empty()
						}
						['к', '\u{1}', s @ ..] => {
							super::push(out, 57290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 30006);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57291);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84974);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106943);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10613);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30007);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10614);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30008);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30009);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57294);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 84975);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106944);
											s.is_empty()
										}
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
					super::push(out, 30010);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 30011);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57296);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10616);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10617);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30013);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30014);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 30015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 10618);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30016);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57299);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 84976);
									s.is_empty()
								}
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
			super::push(out, 10619);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30017);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 30018);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30019);
					match s {
						[] => true,
						['о', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 57301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10620);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30020);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57302);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 84977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нак.
pub(crate) fn n2235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2229);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10621);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30021);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 30022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10622);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30023);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57304);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30024);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57305);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10623);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30026);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57306);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 84980);
									s.is_empty()
								}
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
			super::push(out, 10624);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30027);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 57307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10625);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 30028);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57308);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84981);
									s.is_empty()
								}
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
			super::push(out, 10626);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30029);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57309);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84982);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106945);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10627);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30030);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57310);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84983);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30031);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57311);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 84984);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106946);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10628);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30032);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57312);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10629);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30033);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10630);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30034);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оак.
pub(crate) fn n2236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2230);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 10631);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30035);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пак.
pub(crate) fn n2237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2231);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10632);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10633);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57314);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 84985);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106947);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 57315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10634);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30036);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10635);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30037);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57317);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 30038);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57318);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84987);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106948);
											s.is_empty()
										}
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
					super::push(out, 30039);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30040);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30041);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30042);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 57322);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 84988);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106949);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10636);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30043);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57323);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 84989);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106950);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121841);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132333(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 30044);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 57324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30045);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10637);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30046);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10638);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30047);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 30048);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10639);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30050);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57327);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 84991);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106951);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121842);
													s.is_empty()
												}
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
							super::push(out, 57328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84992);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 84993);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106952);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121843);
													match s {
														[] => true,
														['п', s @ ..] => super::t34::n132334(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10640);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30051);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10641);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30052);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 57330);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 84994);
									s.is_empty()
								}
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
			super::push(out, 10642);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30053);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30054);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10643);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30055);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 30056);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57333);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84995);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рак.
pub(crate) fn n2238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2232);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10644);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10645);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30057);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 30058);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30059);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10646);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10647);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30061);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57334);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 84996);
									s.is_empty()
								}
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
			super::push(out, 10648);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30062);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57335);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10649);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 30063);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 30064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10650);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30065);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57336);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 84997);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106953);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121844);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132335(s, out),
														_ => false,
													}
												}
												['д', s @ ..] => {
													super::push(out, 121845);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132336(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10651);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 30066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10652);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30067);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10653);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30069);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57339);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 84998);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106954);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 57340);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 84999);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106955);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121846);
													match s {
														[] => true,
														['п', s @ ..] => super::t34::n132337(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10654);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30070);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30071);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57342);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85000);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106956);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10655);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30072);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 57343);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 85001);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106957);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10656);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 30073);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 30074);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57344);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 57345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 30075);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57346);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 85003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30076);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30077);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57348);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85004);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106958);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10657);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30078);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30079);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57349);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 85005);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106959);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121847);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30080);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30081);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 30082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10658);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 30083);
					match s {
						[] => true,
						['е', s @ ..] => super::t34::n57935(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30084);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сак.
pub(crate) fn n2239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2233);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10659);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 10660);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30085);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10661);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10662);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30087);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 30088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10663);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30089);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30090);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10664);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30091);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57355);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30092);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57356);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 85007);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106961);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 57357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10665);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 30093);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57358);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85008);
									s.is_empty()
								}
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
			super::push(out, 10666);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30094);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30095);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10667);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30097);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10668);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30098);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -так.
pub(crate) fn n2240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2234);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10669);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30099);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57362);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 85009);
									s.is_empty()
								}
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
			super::push(out, 10670);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30100);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 30101);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57363);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85010);
									s.is_empty()
								}
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
			super::push(out, 10671);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30102);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57364);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85011);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 85012);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30103);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57365);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85013);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106962);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10672);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30104);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30105);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57367);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85014);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30106);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57368);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 85015);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 30107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10673);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30108);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 10674);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30109);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10675);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30110);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57372);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85016);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30111);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10676);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30112);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уак.
pub(crate) fn n2241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2235);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 10677);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30113);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10678);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30114);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фак.
pub(crate) fn n2242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2236);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10679);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 10680);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 10681);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30115);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10682);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30116);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 30117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10683);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 10684);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10685);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хак.
pub(crate) fn n2243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2237);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10686);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30118);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10687);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30119);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10688);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 30120);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чак.
pub(crate) fn n2244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2238);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10689);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 10690);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30121);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57380);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 85018);
									s.is_empty()
								}
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
			super::push(out, 10691);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10692);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 30123);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57381);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85019);
									s.is_empty()
								}
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
			super::push(out, 10693);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30124);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57382);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 85020);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106963);
											s.is_empty()
										}
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
					super::push(out, 30125);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57383);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30126);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57384);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 57385);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 57386);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85022);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 85023);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106964);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10694);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30127);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 30128);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 57387);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 57388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 30129);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10695);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30130);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 30131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10696);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30132);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57390);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 85024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 57391);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85025);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106965);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121849);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шак.
pub(crate) fn n2245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2239);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30133);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30134);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10697);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30135);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 30136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10698);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30137);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10699);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30139);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85026);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30140);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57395);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30141);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10700);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10701);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30143);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30145);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10702);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57399);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 85028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'б', '\u{1}', s @ ..] => {
							super::push(out, 57400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30146);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57401);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 85029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 10703);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30147);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щак.
pub(crate) fn n2246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2240);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10704);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10705);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10706);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30150);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 57403);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -систербек.
pub(crate) fn n123125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121850);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 130982);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -век.
pub(crate) fn n2250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2244);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10728);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 10729);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30169);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57420);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106971);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121851);
													match s {
														[] => true,
														['н', s @ ..] => super::t34::n132340(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['д', s @ ..] => {
											super::push(out, 106972);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121852);
													match s {
														[] => true,
														['р', s @ ..] => super::t34::n132341(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 106973);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121853);
													match s {
														[] => true,
														['в', s @ ..] => super::t34::n132342(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 106974);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121854);
													match s {
														[] => true,
														['р', s @ ..] => super::t34::n132343(s, out),
														_ => false,
													}
												}
												['д', s @ ..] => {
													super::push(out, 121855);
													match s {
														[] => true,
														['е', s @ ..] => super::t34::n132344(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 121856);
													match s {
														[] => true,
														['я', s @ ..] => super::t34::n132345(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 106975);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121857);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132346(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['х', 'р', 'е', 'в', 'с', '\u{1}', s @ ..] => {
											super::push(out, 106976);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дек.
pub(crate) fn n2251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2245);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 10730);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30170);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57421);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85043);
									s.is_empty()
								}
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
			super::push(out, 10731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30171);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57422);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 30172);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57423);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 30173);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10732);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30174);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10733);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30175);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 30176);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 30177);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 57426);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85046);
									s.is_empty()
								}
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
			super::push(out, 10734);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 30178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 10735);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30179);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еек.
pub(crate) fn n2252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2246);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 10736);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30180);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 10737);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30181);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57430);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85047);
									s.is_empty()
								}
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
			super::push(out, 10738);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 30182);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30183);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 57432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30184);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57433);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 85048);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106977);
											s.is_empty()
										}
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
					super::push(out, 30185);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жек.
pub(crate) fn n2253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2247);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10740);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30186);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57434);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85049);
									s.is_empty()
								}
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
			super::push(out, 10741);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30187);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 30188);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57435);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 85050);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106978);
											s.is_empty()
										}
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
					super::push(out, 30189);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30190);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 57437);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85051);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106979);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10742);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30191);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57438);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 85052);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106980);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 121858);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10743);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57439);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85053);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 85054);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106981);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121859);
													s.is_empty()
												}
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
							super::push(out, 57440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зек.
pub(crate) fn n2254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2248);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10744);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 10745);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30193);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10746);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30194);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шкек.
pub(crate) fn n10780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10748);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 30195);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 57443);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лек.
pub(crate) fn n2257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2251);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10749);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10750);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 30196);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 30197);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57444);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85055);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106982);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121860);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10751);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30198);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57445);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 57446);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85056);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 57447);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85057);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106983);
											s.is_empty()
										}
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
					super::push(out, 30199);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57448);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 57449);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85058);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30200);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30201);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57450);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 57451);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85059);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106984);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 57452);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85060);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106985);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121861);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132347(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 30202);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57453);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85061);
									s.is_empty()
								}
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
			super::push(out, 10752);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10753);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30204);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57454);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 85062);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 57455);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 85063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 57456);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30205);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57457);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30206);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57458);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 85066);
									s.is_empty()
								}
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
			super::push(out, 10754);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10755);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30208);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30209);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57459);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 85067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 57460);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 57461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мек.
pub(crate) fn n2258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2252);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10756);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10757);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 10758);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 30212);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 57462);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нек.
pub(crate) fn n2259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2253);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30213);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30214);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30215);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57463);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85068);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10760);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30217);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57464);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85069);
									s.is_empty()
								}
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
			super::push(out, 10761);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30218);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 30219);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 30220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57465);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 57466);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 85070);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 57467);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 85071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 30221);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30222);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 30223);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 57469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30224);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57470);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 85072);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 106986);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 85073);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 106987);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 85074);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106988);
											s.is_empty()
										}
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
					super::push(out, 30225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10762);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10763);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30227);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57473);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10764);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10765);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30230);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10766);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оек.
pub(crate) fn n2260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2254);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10767);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 10768);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 30232);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57476);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85076);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106989);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10769);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10770);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30234);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пек.
pub(crate) fn n2261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2255);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 10771);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10772);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 30236);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57479);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85077);
									s.is_empty()
								}
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
			super::push(out, 10773);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30237);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10774);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30238);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57481);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85078);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 106990);
											s.is_empty()
										}
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
					super::push(out, 30239);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57482);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30240);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10775);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 10776);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10777);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30242);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рек.
pub(crate) fn n2262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2256);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10778);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30243);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 30244);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57486);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85079);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 30245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30247);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57489);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30248);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57490);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 30249);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 30250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10779);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 10780);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30252);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 30253);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30254);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57493);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 85082);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30255);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57494);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 85083);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 106991);
											s.is_empty()
										}
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
					super::push(out, 30256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57495);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10781);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 10782);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30257);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57496);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 85084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 57497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30258);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30259);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57499);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85085);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106992);
											s.is_empty()
										}
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
					super::push(out, 30260);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10783);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30261);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57501);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 30262);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57502);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 85087);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106993);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10784);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30263);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30264);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 57504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 30265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 10785);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30266);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30267);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10786);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 30268);
					match s {
						[] => true,
						['т', 'р', 'е', 'д', '\u{1}', s @ ..] => {
							super::push(out, 57506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30269);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 30270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10787);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30271);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57507);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 85088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 30272);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10788);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30273);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10789);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30274);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57508);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 57509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30275);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57510);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10790);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 30276);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сек.
pub(crate) fn n2263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2257);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10791);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30277);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10792);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30278);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10793);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30279);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10794);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30280);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57515);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85090);
									s.is_empty()
								}
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
			super::push(out, 10795);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30281);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10796);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30282);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30283);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57517);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85091);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106994);
											s.is_empty()
										}
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
					super::push(out, 30284);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57518);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 85092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30285);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57519);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85093);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 57520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85094);
									s.is_empty()
								}
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
			super::push(out, 10797);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30286);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10798);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30287);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10799);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30289);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10800);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30290);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57524);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85095);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106995);
											s.is_empty()
										}
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
					super::push(out, 30291);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10801);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30292);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30293);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тек.
pub(crate) fn n2264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2258);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10802);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 30294);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57527);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 85096);
									s.is_empty()
								}
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
			super::push(out, 10803);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57528);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 30296);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 30297);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30298);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57529);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85097);
									s.is_empty()
								}
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
			super::push(out, 10804);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30299);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10805);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 30300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10806);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57533);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 57534);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 57535);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 85098);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 85099);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106996);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121862);
													match s {
														[] => true,
														['т', s @ ..] => super::t34::n132348(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', 'к', 'у', '\u{1}', s @ ..] => {
									super::push(out, 85100);
									s.is_empty()
								}
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
			super::push(out, 10807);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30304);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 57536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 10808);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30305);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 57537);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85101);
									s.is_empty()
								}
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
			super::push(out, 10809);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30306);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57538);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 85102);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106997);
											s.is_empty()
										}
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
					super::push(out, 30307);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57539);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30308);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57540);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85104);
									s.is_empty()
								}
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
			super::push(out, 10810);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 10811);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 10812);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10813);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30311);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 10814);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30312);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57542);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85105);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 10815);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30313);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10816);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уек.
pub(crate) fn n2265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2259);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 10817);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 10818);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 10819);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30316);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57544);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хек.
pub(crate) fn n2266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2260);
	s.is_empty()
}

/// Words ending -цек.
pub(crate) fn n2267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2261);
	s.is_empty()
}

/// Words ending -чек.
pub(crate) fn n2268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2262);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10820);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30317);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57545);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 85107);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106998);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121863);
													match s {
														[] => true,
														['т', s @ ..] => super::t34::n132349(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10821);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 30318);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30319);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57546);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 85108);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106999);
											s.is_empty()
										}
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
					super::push(out, 30320);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57547);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85109);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 107000);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10822);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30321);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 57548);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85110);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 107001);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10823);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30322);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 57549);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 30323);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57550);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85112);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 107002);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 121864);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30324);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30325);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57551);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 57552);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85113);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 107003);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 121865);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132350(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 30326);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57553);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85114);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 107004);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121866);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132351(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 30327);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57554);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85115);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 107005);
											s.is_empty()
										}
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
					super::push(out, 30328);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30329);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57555);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 85116);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 107006);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121867);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30330);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57556);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85117);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 107007);
											s.is_empty()
										}
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
					super::push(out, 30331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t34::n10856(s, out),
		['о', s @ ..] => super::t34::n10857(s, out),
		['у', s @ ..] => {
			super::push(out, 10826);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30350);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30351);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шек.
pub(crate) fn n2269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10827);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30352);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57630);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 85247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30353);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57631);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30354);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57632);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85249);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 107136);
											s.is_empty()
										}
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
					super::push(out, 30355);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57633);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 85250);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30356);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57634);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 85251);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 107137);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10828);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30357);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57635);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 30358);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57636);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85253);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 107138);
											s.is_empty()
										}
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
					super::push(out, 30359);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57637);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30360);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57638);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85255);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 57639);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85256);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 107139);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121970);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30361);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10829);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57641);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 57642);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 57643);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 85257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30363);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57644);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 85258);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 57645);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85259);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 107140);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 121971);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10830);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30364);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57646);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 85260);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 107141);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10831);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10832);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30366);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57647);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85261);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 107142);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10833);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30367);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57648);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85262);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 57649);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85263);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 107143);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 57650);
							match s {
								[] => true,
								['р', s @ ..] => super::t34::n86170(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t34::n30579(s, out),
				['д', s @ ..] => super::t34::n30580(s, out),
				['м', s @ ..] => super::t34::n30581(s, out),
				['н', s @ ..] => super::t34::n30582(s, out),
				['п', s @ ..] => super::t34::n30583(s, out),
				['р', s @ ..] => super::t34::n30584(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t34::n10866(s, out),
		['э', s @ ..] => {
			super::push(out, 10835);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t34::n30592(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word мозамбик.
pub(crate) fn n123252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121977);
	s.is_empty()
}

/// Words ending -карпозубик.
pub(crate) fn n123253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121978);
	s.is_empty()
}

/// Words ending -вик.
pub(crate) fn n2272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2266);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10851);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10852);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30398);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57677);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85282);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 107152);
											s.is_empty()
										}
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
					super::push(out, 30399);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57678);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 57679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30400);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10853);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30402);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 57682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30403);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57683);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 85284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 30404);
					match s {
						[] => true,
						['о', 'р', s @ ..] => {
							super::push(out, 57684);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85285);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 107153);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121979);
													s.is_empty()
												}
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
							super::push(out, 57685);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 85286);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 107154);
											s.is_empty()
										}
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
					super::push(out, 30405);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57686);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85287);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 107155);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 57687);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85288);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 107156);
											s.is_empty()
										}
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
					super::push(out, 30406);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57688);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 85289);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 107157);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 121980);
													match s {
														[] => true,
														['е', s @ ..] => super::t34::n132417(s, out),
														_ => false,
													}
												}
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
							super::push(out, 57689);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 85290);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 107158);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 85291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 57690);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 57691);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => super::t34::n58282(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t34::n30621(s, out),
				['о', s @ ..] => super::t34::n30622(s, out),
				['р', s @ ..] => super::t34::n30623(s, out),
				['т', s @ ..] => super::t34::n30624(s, out),
				['ц', s @ ..] => super::t34::n30625(s, out),
				['ч', s @ ..] => super::t34::n30626(s, out),
				['ш', s @ ..] => super::t34::n30627(s, out),
				['щ', s @ ..] => super::t34::n30628(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t34::n10886(s, out),
		['о', s @ ..] => super::t34::n10887(s, out),
		['с', s @ ..] => super::t34::n10888(s, out),
		['ю', s @ ..] => super::t34::n10889(s, out),
		['я', s @ ..] => super::t34::n10890(s, out),
		_ => false,
	}
}

/// Words ending -роплегик.
pub(crate) fn n86331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85424);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 107250);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 122017);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 131077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -регик.
pub(crate) fn n30649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30433);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 57805);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тегик.
pub(crate) fn n30650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30434);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 57806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -огик.
pub(crate) fn n10893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10861);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 30435);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ргик.
pub(crate) fn n10894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10862);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 30436);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 57807);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дик.
pub(crate) fn n2274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2268);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10863);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30437);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 30438);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							super::push(out, 57809);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 57810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85425);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 57811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30440);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30442);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 30443);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10864);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 30444);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30445);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 30446);
					s.is_empty()
				}
				['п', 'и', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57815);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 85426);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 57816);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85427);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 107251);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10865);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30448);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57817);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 85428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30449);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 57818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10866);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10867);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30451);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57819);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 85429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 30452);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10868);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30453);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 30454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 30455);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57822);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 85430);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 107252);
											s.is_empty()
										}
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
					super::push(out, 30456);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 30457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10869);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10870);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30459);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 30460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', 'л', 'а', s @ ..] => {
			super::push(out, 10871);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30461);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57824);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 85431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 10872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жик.
pub(crate) fn n2275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2269);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10873);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30463);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57825);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85432);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 107253);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 122018);
													match s {
														[] => true,
														['п', s @ ..] => super::t34::n132435(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 30464);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30465);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57827);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85433);
									s.is_empty()
								}
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
			super::push(out, 10874);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 10875);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30466);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57828);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30467);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 30468);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30469);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57830);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85435);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 107254);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 122019);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10876);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30470);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 30471);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10877);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 30473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10878);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30474);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57831);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10879);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30475);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57832);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85436);
									s.is_empty()
								}
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
			super::push(out, 10880);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 30476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10881);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30477);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57833);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30478);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10882);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30479);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 30480);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 30481);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30482);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 30483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 10883);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57837);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 57838);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 10884);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зик.
pub(crate) fn n2276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2270);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30486);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 30487);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30488);
					match s {
						[] => true,
						['г', 'е', s @ ..] => {
							super::push(out, 57839);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 85438);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 107255);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 122020);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 85439);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 107256);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 122021);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30489);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30490);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 30491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 10886);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30492);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 57840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30493);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10887);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 30494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10888);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30495);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57842);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 85440);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 107257);
											s.is_empty()
										}
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
					super::push(out, 30496);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57843);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85441);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 107258);
											s.is_empty()
										}
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
					super::push(out, 30497);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57844);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85442);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 107259);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 122022);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30498);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57845);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 10889);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10890);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30500);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57846);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10891);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 30501);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 57847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 10892);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30502);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57849);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 57850);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85443);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 107260);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 57851);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 85444);
									s.is_empty()
								}
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
			super::push(out, 10893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кик.
pub(crate) fn n2277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2271);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 10894);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30504);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 57852);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 85445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30505);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57853);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 30506);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57854);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85447);
									s.is_empty()
								}
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
			super::push(out, 10895);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 30507);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лик.
pub(crate) fn n2278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2272);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10896);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30508);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30509);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 30510);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57855);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 85448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 30511);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30512);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 30513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30514);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30515);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57858);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85449);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 107261);
											s.is_empty()
										}
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
					super::push(out, 30516);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57859);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 57860);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 85451);
									s.is_empty()
								}
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
			super::push(out, 10897);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30517);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30518);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 30519);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57862);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 57863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 30520);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 57864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 10898);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30521);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 10899);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30522);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 57866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 10900);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30523);
					s.is_empty()
				}
				['г', 'н', s @ ..] => {
					super::push(out, 30524);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57867);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 85452);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 107262);
											s.is_empty()
										}
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
					super::push(out, 30525);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30526);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 30527);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 30528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 10901);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30529);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57869);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 85453);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 107263);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 122023);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132436(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10902);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30530);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57870);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 85454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30531);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 30532);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57871);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 30533);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57872);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 85456);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 107264);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 122024);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10903);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30534);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 57873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30535);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 57874);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 57875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10904);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30536);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57876);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30537);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 57877);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 57878);
							match s {
								[] => true,
								['е', s @ ..] => super::t34::n86367(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 30538);
					s.is_empty()
				}
				['с', s @ ..] => super::t34::n30757(s, out),
				['т', s @ ..] => super::t34::n30758(s, out),
				['ы', s @ ..] => super::t34::n30759(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t34::n10938(s, out),
		['м', s @ ..] => super::t34::n10939(s, out),
		['о', s @ ..] => super::t34::n10940(s, out),
		['р', s @ ..] => super::t34::n10941(s, out),
		['с', s @ ..] => super::t34::n10942(s, out),
		['т', s @ ..] => super::t34::n10943(s, out),
		['у', s @ ..] => super::t34::n10944(s, out),
		['ф', s @ ..] => super::t34::n10945(s, out),
		['х', s @ ..] => super::t34::n10946(s, out),
		['ы', s @ ..] => super::t34::n10947(s, out),
		['ю', s @ ..] => super::t34::n10948(s, out),
		['я', s @ ..] => super::t34::n10949(s, out),
		_ => false,
	}
}

/// Words ending -мик.
pub(crate) fn n2279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2273);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10917);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 30569);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30570);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 57916);
							match s {
								[] => true,
								['д', 'о', s @ ..] => {
									super::push(out, 85479);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 107272);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 122027);
													match s {
														[] => true,
														['а', s @ ..] => super::t34::n132437(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57917);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 85480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 57918);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85481);
									s.is_empty()
								}
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
			super::push(out, 10918);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 30571);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57919);
							match s {
								[] => true,
								['к', 'а', '-', s @ ..] => {
									super::push(out, 85482);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 107273);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 122028);
													match s {
														[] => true,
														['б', s @ ..] => super::t34::n132438(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 10919);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 30572);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 57920);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 85483);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 107274);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 122029);
													match s {
														[] => true,
														['в', s @ ..] => super::t34::n132439(s, out),
														_ => false,
													}
												}
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
							super::push(out, 57921);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 10920);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 30573);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 57922);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85484);
									s.is_empty()
								}
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
			super::push(out, 10921);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30574);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 57923);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10922);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30575);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 57924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85485);
									s.is_empty()
								}
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
			super::push(out, 10923);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30576);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 30577);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 30578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 30579);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 30580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30581);
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 30582);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 30583);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 57927);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 57928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 57929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10924);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 10925);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30585);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 57930);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 85486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ник.
pub(crate) fn n2280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2274);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 10926);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 10927);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30586);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 30587);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 57931);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 85487);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 107275);
											s.is_empty()
										}
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
					super::push(out, 30588);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 57932);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 85488);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 107276);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 122030);
													match s {
														[] => true,
														['и', s @ ..] => super::t34::n132440(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 30589);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 57933);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 85489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30590);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 57934);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 85490);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 107277);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 122031);
													s.is_empty()
												}
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
							super::push(out, 57935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30591);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 57936);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 85491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 57937);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 85492);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 107278);
											s.is_empty()
										}
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
					super::push(out, 30592);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 57938);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 85493);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 107279);
											match s {
												[] => true,
												['н', s @ ..] => super::t34::n123307(s, out),
												['р', s @ ..] => super::t34::n123308(s, out),
												['ь', s @ ..] => super::t34::n123309(s, out),
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 107280);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => super::t34::n30812(s, out),
				['ш', s @ ..] => super::t34::n30813(s, out),
				['щ', s @ ..] => {
					super::push(out, 30595);
					match s {
						[] => true,
						['о', s @ ..] => super::t34::n58534(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t34::n10961(s, out),
		['в', s @ ..] => super::t34::n10962(s, out),
		['г', s @ ..] => super::t34::n10963(s, out),
		['д', s @ ..] => super::t34::n10964(s, out),
		['е', s @ ..] => super::t34::n10965(s, out),
		['ж', s @ ..] => super::t34::n10966(s, out),
		['з', s @ ..] => super::t34::n10967(s, out),
		['и', s @ ..] => super::t34::n10968(s, out),
		['й', s @ ..] => super::t34::n10969(s, out),
		['к', s @ ..] => super::t34::n10970(s, out),
		['м', s @ ..] => super::t34::n10971(s, out),
		['н', s @ ..] => super::t34::n10972(s, out),
		['о', s @ ..] => super::t34::n10973(s, out),
		['п', s @ ..] => super::t34::n10974(s, out),
		['р', s @ ..] => super::t34::n10975(s, out),
		['с', s @ ..] => super::t34::n10976(s, out),
		['т', s @ ..] => super::t34::n10977(s, out),
		['у', s @ ..] => super::t34::n10978(s, out),
		['ф', s @ ..] => super::t34::n10979(s, out),
		['х', s @ ..] => super::t34::n10980(s, out),
		['ц', s @ ..] => super::t34::n10981(s, out),
		['ч', s @ ..] => super::t35::n10982(s, out),
		['ш', s @ ..] => super::t35::n10983(s, out),
		['щ', s @ ..] => super::t35::n10984(s, out),
		['ь', s @ ..] => super::t35::n10985(s, out),
		['ю', s @ ..] => super::t35::n10986(s, out),
		['я', s @ ..] => super::t35::n10987(s, out),
		_ => false,
	}
}

/// Words ending -оик.
pub(crate) fn n2281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2275);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 10955);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30772);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 58661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 10956);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 30773);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пик.
pub(crate) fn n2282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2276);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 10957);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30774);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 58662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30775);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 58663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30776);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 58664);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 86727);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108461);
											s.is_empty()
										}
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
					super::push(out, 30777);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 58665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 10958);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 30778);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 30779);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 30780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 10959);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 30781);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 58666);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86728);
									s.is_empty()
								}
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
			super::push(out, 10960);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30782);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 58667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 10961);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 30783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 10962);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 30784);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 58668);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 86729);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 108462);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 86730);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 108463);
											s.is_empty()
										}
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
					super::push(out, 30785);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 58669);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 30786);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 58670);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 86732);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 108464);
											s.is_empty()
										}
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
					super::push(out, 30787);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 58671);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 30788);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 58672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 30789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 58673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 30790);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 58674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 30791);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 30792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 10963);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 30793);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 58675);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 86734);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 30794);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 58676);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 86735);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 58677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86736);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 58678);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 86737);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 108465);
											s.is_empty()
										}
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
					super::push(out, 30795);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 58679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 10964);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 30796);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 58680);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 86738);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 108466);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 10965);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30797);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 58681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 30798);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 30799);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 30800);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 10966);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 10967);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 30802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 10968);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 30803);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 58682);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 86739);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

