use flexion::Path;

/// Words ending -чница.
pub(crate) fn n24873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24718);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51944);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80536);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 80537);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 80538);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 80539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 51945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80540);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103660);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 80541);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103661);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119647);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129641);
											s.is_empty()
										}
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
					super::push(out, 80542);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103662);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119648);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 80543);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103663);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119649);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 129642);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103664);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119650);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129643);
											s.is_empty()
										}
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
					super::push(out, 80544);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103665);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 119651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80545);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103666);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 103667);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103668);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119654);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129644);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135570);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80546);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103669);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80547);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103670);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119656);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 103671);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119657);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129645);
											s.is_empty()
										}
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
					super::push(out, 80548);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103672);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 103673);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119658);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129646);
											s.is_empty()
										}
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
					super::push(out, 80549);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103674);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 119659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 103675);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119660);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 129647);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 103676);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119661);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 103677);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119662);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129648);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135571);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140453(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51946);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80550);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103678);
							match s {
								[] => true,
								['г', s @ ..] => super::t50::n120926(s, out),
								['л', s @ ..] => super::t50::n120927(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t50::n81392(s, out),
				['н', s @ ..] => super::t50::n81393(s, out),
				['п', s @ ..] => super::t50::n81394(s, out),
				['р', s @ ..] => super::t50::n81395(s, out),
				['т', s @ ..] => super::t50::n81396(s, out),
				['ч', s @ ..] => super::t50::n81397(s, out),
				['я', s @ ..] => super::t50::n81398(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t50::n52470(s, out),
		['у', s @ ..] => super::t50::n52471(s, out),
		['ы', s @ ..] => super::t50::n52472(s, out),
		['ю', s @ ..] => super::t50::n52473(s, out),
		['я', s @ ..] => super::t50::n52474(s, out),
		_ => false,
	}
}

/// Words ending -шница.
pub(crate) fn n24874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24719);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51952);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80581);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 103734);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119722);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129679);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135582);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140459(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 80582);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 51953);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 80583);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103736);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119723);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129680);
											s.is_empty()
										}
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
					super::push(out, 80584);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103737);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 103738);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119724);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 129681);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135583);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 119725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80585);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 103739);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 103740);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 103741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 51954);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80587);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103742);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119728);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129682);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135584);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80588);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 103743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 103744);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119729);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 103745);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119730);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 80589);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 51955);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80590);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 80591);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103748);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 103749);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119731);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103750);
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									super::push(out, 119732);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 103751);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 80592);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103752);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 103753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 80593);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119734);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129683);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 119735);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129684);
											s.is_empty()
										}
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
					super::push(out, 80594);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 103754);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119736);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103755);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 119737);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 119738);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129686);
											s.is_empty()
										}
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
					super::push(out, 80595);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 51956);
			match s {
				[] => true,
				['а', s @ ..] => super::t50::n81439(s, out),
				['л', s @ ..] => super::t50::n81440(s, out),
				['р', s @ ..] => {
					super::push(out, 80598);
					s.is_empty()
				}
				['т', s @ ..] => super::t50::n81442(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t50::n52480(s, out),
		['э', s @ ..] => super::t50::n52481(s, out),
		['ю', s @ ..] => super::t50::n52482(s, out),
		_ => false,
	}
}

/// Words ending -щница.
pub(crate) fn n24875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24720);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51960);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 51961);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 51962);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80607);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103767);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119748);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129693);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135588);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140460(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 80608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 51963);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 80609);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 51964);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80610);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 103769);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 119749);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129694);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135589);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьница.
pub(crate) fn n24876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24721);
	match s {
		[] => true,
		['л', s @ ..] => super::t50::n52488(s, out),
		_ => false,
	}
}

/// Words ending -юница.
pub(crate) fn n24877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24722);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 51966);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яница.
pub(crate) fn n24878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24723);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 51967);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80617);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 103805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80618);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103806);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 51968);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80619);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 80620);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103808);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119807);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 80621);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103809);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119808);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103810);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 103811);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119809);
									s.is_empty()
								}
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
			super::push(out, 51969);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80622);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 103812);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 80623);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103813);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80624);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103814);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119811);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129759);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135630);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51970);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80625);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103815);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 80626);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 51971);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 80627);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103817);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 119813);
									s.is_empty()
								}
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
			super::push(out, 51972);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80628);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103818);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119814);
									s.is_empty()
								}
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
			super::push(out, 51973);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 80629);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -протопопица.
pub(crate) fn n137038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135631);
	s.is_empty()
}

/// Words ending -фельдшерица.
pub(crate) fn n137039(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135632);
	s.is_empty()
}

/// Words ending -трица.
pub(crate) fn n24899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24744);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52009);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 80665);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 103839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80666);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103840);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119827);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 129764);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 135633);
													match s {
														[] => true,
														['-', s @ ..] => super::t50::n140496(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 52010);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80667);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80668);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 80669);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -скароносица.
pub(crate) fn n137041(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135634);
	s.is_empty()
}

/// Words ending -азномастица.
pub(crate) fn n137042(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135635);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятица.
pub(crate) fn n24918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24763);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 52038);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 80698);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 52039);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80699);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иросдатчица.
pub(crate) fn n137043(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135636);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139061);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141037);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142197);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 142852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -платьевщица.
pub(crate) fn n137044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135637);
	s.is_empty()
}

/// Words ending -позитивщица.
pub(crate) fn n137045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135638);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139062);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 141038);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142198);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овщица.
pub(crate) fn n52586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52063);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 80725);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 80726);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103886);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119852);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 103887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 80727);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103888);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 119853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129785);
									s.is_empty()
								}
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
			super::push(out, 80728);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 80729);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103890);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 119854);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129786);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119855);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129787);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135639);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 119856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 103891);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119857);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 129788);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135640);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'м', s @ ..] => {
			super::push(out, 80730);
			match s {
				[] => true,
				['а', 'л', s @ ..] => {
					super::push(out, 103892);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 80731);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 103893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 80732);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 103894);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 103895);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 119859);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129789);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 103896);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119860);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129790);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 135641);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 80733);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 119861);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 119862);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129791);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 135642);
											s.is_empty()
										}
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
					super::push(out, 103897);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 119863);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 129792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 103898);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 103899);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119864);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 129793);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135643);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 119865);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 129794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 119866);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 129795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 103900);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 103901);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119867);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 129796);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 135644);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 80734);
			match s {
				[] => true,
				['а', 'р', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 119868);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 119869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 103902);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 119870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 103903);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 80735);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вещица.
pub(crate) fn n52589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52066);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 80737);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мщица.
pub(crate) fn n24939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24784);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 52067);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 52068);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 52069);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 52070);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80738);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103905);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 103906);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 80739);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103907);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119874);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 129797);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135645);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нщица.
pub(crate) fn n24940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24785);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52071);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 52072);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80740);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 80741);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103908);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119875);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129798);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135646);
													s.is_empty()
												}
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
							super::push(out, 103909);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119876);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 129799);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135647);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80742);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 80743);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103910);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119877);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129800);
											s.is_empty()
										}
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
					super::push(out, 80744);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103911);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119878);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129801);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'р', 'в', '\u{1}', s @ ..] => {
							super::push(out, 103912);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 103913);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 103914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80745);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103915);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119879);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129802);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135648);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 80746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 52073);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 52074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ощица.
pub(crate) fn n24941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24786);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 52075);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 80747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103916);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 52076);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80748);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пщица.
pub(crate) fn n24942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24787);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 52077);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 52078);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 52079);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103917);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119880);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103918);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 103919);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119882);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129803);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 103920);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119883);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129804);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 52080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рщица.
pub(crate) fn n24943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24788);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52081);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80749);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 80750);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 103921);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119884);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129805);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80751);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 80752);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103922);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119885);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 129806);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135649);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140500(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 80753);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103923);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119886);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129807);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 103924);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 119887);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129808);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135650);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140501(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 52082);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 52083);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80754);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 103925);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119888);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129809);
											s.is_empty()
										}
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
					super::push(out, 80755);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 80756);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103927);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119889);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129810);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103928);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 103929);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 80757);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 103930);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119891);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129811);
											s.is_empty()
										}
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
					super::push(out, 80758);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103931);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119892);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 103932);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119893);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129812);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 103933);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119894);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 129813);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 52084);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 80759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьщица.
pub(crate) fn n24944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24789);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80760);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103934);
							match s {
								[] => true,
								['и', 'е', s @ ..] => {
									super::push(out, 119895);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 129814);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135651);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140502(s, out),
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
									super::push(out, 119896);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 119897);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 129815);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135652);
													match s {
														[] => true,
														['л', s @ ..] => super::t50::n140503(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 129816);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135653);
													match s {
														[] => true,
														['м', s @ ..] => super::t50::n140504(s, out),
														_ => false,
													}
												}
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
							super::push(out, 103935);
							s.is_empty()
						}
						['з', s @ ..] => super::t50::n105035(s, out),
						['к', s @ ..] => super::t50::n105036(s, out),
						['н', s @ ..] => {
							super::push(out, 103938);
							s.is_empty()
						}
						['п', s @ ..] => super::t50::n105038(s, out),
						['с', s @ ..] => {
							super::push(out, 103940);
							s.is_empty()
						}
						['т', s @ ..] => super::t50::n105040(s, out),
						['х', s @ ..] => super::t50::n105041(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t50::n81607(s, out),
				['и', s @ ..] => super::t50::n81608(s, out),
				['о', 'г', s @ ..] => super::t50::n81609(s, out),
				['у', 'п', s @ ..] => super::t50::n81610(s, out),
				['я', 'е', s @ ..] => super::t50::n81611(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -липмейкерша.
pub(crate) fn n137080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135673);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 139078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -роллерша.
pub(crate) fn n105117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104016);
	s.is_empty()
}

/// Words ending -олерша.
pub(crate) fn n52786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52260);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 80878);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 104017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлерша.
pub(crate) fn n52787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52261);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 80879);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 104018);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -флерша.
pub(crate) fn n52788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52262);
	s.is_empty()
}

/// Words ending -мерша.
pub(crate) fn n25133(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 52263);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 80880);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 104019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'р', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 80881);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 80882);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 104020);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 52264);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80883);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 104021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 52265);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 80884);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 104022);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нерша.
pub(crate) fn n25134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24974);
	match s {
		[] => true,
		['е', 'р', 'т', s @ ..] => {
			super::push(out, 52266);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 52267);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80886);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 104023);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119953);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129851);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', 'а', 'м', s @ ..] => {
			super::push(out, 52268);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 52269);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 80888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 52270);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80889);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 104024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 52271);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 80890);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 104025);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -перша.
pub(crate) fn n25135(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80891);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 104026);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 52272);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -серша.
pub(crate) fn n25136(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 52273);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80894);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 104027);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', 'о', 'б', 'к', s @ ..] => {
			super::push(out, 52274);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 80895);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 52275);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 80896);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 104029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 52276);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 80897);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 104030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -терша.
pub(crate) fn n25137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24975);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 52277);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 80898);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 104031);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119956);
									s.is_empty()
								}
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
			super::push(out, 52278);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 52279);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 52280);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80899);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 104032);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 119957);
									s.is_empty()
								}
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
			super::push(out, 52281);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80900);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 104033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 52282);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 52283);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80901);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 104034);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119958);
									s.is_empty()
								}
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
			super::push(out, 52284);
			match s {
				[] => true,
				['и', 'н', s @ ..] => {
					super::push(out, 80902);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 104035);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 80903);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 104036);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119960);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80904);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 80905);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 104037);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 52285);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80906);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ферша.
pub(crate) fn n25138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24976);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 52286);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 80907);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -херша.
pub(crate) fn n25139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24977);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 52287);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 80908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -церша.
pub(crate) fn n25140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24978);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 52288);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 80909);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -черша.
pub(crate) fn n25141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24979);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 52289);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьерша.
pub(crate) fn n25142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24980);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 52290);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 80911);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юерша.
pub(crate) fn n25143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24981);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 52291);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80912);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льтитаскинг.
pub(crate) fn n137102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135695);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 139085);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word тревеллинг.
pub(crate) fn n137103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135696);
	s.is_empty()
}

/// The word канцеллинг.
pub(crate) fn n137104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135697);
	s.is_empty()
}

/// Words ending -ктимблейминг.
pub(crate) fn n137105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135698);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139086);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 141047);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word боулдеринг.
pub(crate) fn n137106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135699);
	s.is_empty()
}

/// Words ending -римендеринг.
pub(crate) fn n137107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135700);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139087);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141048);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 142201);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 142854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -композитинг.
pub(crate) fn n137108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135701);
	s.is_empty()
}

/// The word одельстинг.
pub(crate) fn n137109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135702);
	s.is_empty()
}

/// Words ending -пауэрлифтинг.
pub(crate) fn n137110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135703);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 139088);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word веб-серфинг.
pub(crate) fn n137111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135704);
	s.is_empty()
}

/// Words ending -рнет-серфинг.
pub(crate) fn n137112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135705);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139089);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141049);
					match s {
						[] => true,
						['н', 'и', '\u{1}', s @ ..] => {
							super::push(out, 142202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ф-паблишинг.
pub(crate) fn n137113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135706);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139090);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шер-гинеколог.
pub(crate) fn n137116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135709);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 139093);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141051);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ог-эндокринолог.
pub(crate) fn n137117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135710);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139094);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141052);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 142203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еса-монолог.
pub(crate) fn n137118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135711);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 139095);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 141053);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульмонолог.
pub(crate) fn n131272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129920);
	s.is_empty()
}

/// Words ending -фонолог.
pub(crate) fn n82236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81388);
	s.is_empty()
}

/// Words ending -унолог.
pub(crate) fn n53532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52995);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 81389);
			match s {
				[] => true,
				['м', 'и', '\u{1}', s @ ..] => {
					super::push(out, 104321);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хнолог.
pub(crate) fn n53533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52996);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 81390);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 120090);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129921);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 135712);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 139096);
													match s {
														[] => true,
														['и', s @ ..] => super::t50::n142499(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 139097);
													match s {
														[] => true,
														['х', s @ ..] => super::t50::n142500(s, out),
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
									super::push(out, 129922);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 135713);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139098);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n142501(s, out),
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
									super::push(out, 129923);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135714);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 139099);
													match s {
														[] => true,
														['п', s @ ..] => super::t50::n142502(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['е', s @ ..] => {
											super::push(out, 135715);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139100);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n142503(s, out),
														_ => false,
													}
												}
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
							super::push(out, 120091);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 120092);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 129924);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 135716);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -полог.
pub(crate) fn n25964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25793);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 52997);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 52998);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 81391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 52999);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 81392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ролог.
pub(crate) fn n25965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25794);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 53000);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 53001);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 53002);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 53003);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 81393);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 81394);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 81395);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 104323);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120093);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 129925);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 53004);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 81396);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 104324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 53005);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 53006);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 81397);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 81398);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 81399);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 104325);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120094);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129926);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 53007);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 53008);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -солог.
pub(crate) fn n25966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25795);
	s.is_empty()
}

/// Words ending -толог.
pub(crate) fn n25967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25796);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53009);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 53010);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 81400);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 104326);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120095);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129927);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135717);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53011);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 81401);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 104327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120096);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фолог.
pub(crate) fn n25968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25797);
	s.is_empty()
}

/// Words ending -холог.
pub(crate) fn n25969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25798);
	match s {
		[] => true,
		['и', 'с', 'п', 'о', s @ ..] => {
			super::push(out, 53012);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 81402);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 104328);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рыба-хирург.
pub(crate) fn n137132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135725);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 139104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кс-бойфренд.
pub(crate) fn n137147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135740);
	s.is_empty()
}

/// The word микропровод.
pub(crate) fn n140554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139117);
	s.is_empty()
}

/// Words ending -лектропровод.
pub(crate) fn n140555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139118);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 141068);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -предстоящее.
pub(crate) fn n137162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135755);
	s.is_empty()
}

/// Words ending -вание.
pub(crate) fn n26862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26678);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53818);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 81991);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 81992);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 104660);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 81993);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 104661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 81994);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 104662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 53819);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 81995);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 81996);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 81997);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 81998);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 81999);
					s.is_empty()
				}
				['ч', 'т', s @ ..] => {
					super::push(out, 82000);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104663);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120278);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130005);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 53820);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82001);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 104664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 82002);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 82003);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 104666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 82004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t50::n54366(s, out),
		['о', s @ ..] => super::t50::n54367(s, out),
		['р', s @ ..] => {
			super::push(out, 53823);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 53824);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 82035);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 82036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 53825);
			match s {
				[] => true,
				['б', s @ ..] => super::t50::n82885(s, out),
				['в', s @ ..] => super::t50::n82886(s, out),
				['д', s @ ..] => {
					super::push(out, 82039);
					s.is_empty()
				}
				['з', s @ ..] => super::t50::n82888(s, out),
				['л', s @ ..] => super::t50::n82889(s, out),
				['м', s @ ..] => super::t50::n82890(s, out),
				['н', 'з', s @ ..] => {
					super::push(out, 82042);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 104834);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t50::n121766(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t50::n82892(s, out),
				['р', s @ ..] => super::t50::n82893(s, out),
				['с', s @ ..] => super::t50::n82894(s, out),
				['т', s @ ..] => super::t50::n82895(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зание.
pub(crate) fn n26866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26682);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53846);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 82069);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104863);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 104864);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 120527);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 130210);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135887);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140645(s, out),
														_ => false,
													}
												}
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
							super::push(out, 104865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 82070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104866);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 104867);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 120528);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130211);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135888);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82071);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104868);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120529);
									s.is_empty()
								}
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
			super::push(out, 53847);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82072);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 82073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104869);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 104870);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120530);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130212);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135889);
													s.is_empty()
												}
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
							super::push(out, 104871);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 120531);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130213);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 135890);
													match s {
														[] => true,
														['м', s @ ..] => super::t50::n140646(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53848);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 53849);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 82074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 53850);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					super::push(out, 82075);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 53851);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 82076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 53852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кание.
pub(crate) fn n26867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26683);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53853);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82077);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104872);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', '\u{1}', s @ ..] => {
					super::push(out, 82078);
					s.is_empty()
				}
				['т', 'а', s @ ..] => {
					super::push(out, 82079);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 104873);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120533);
									s.is_empty()
								}
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
			super::push(out, 53854);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82080);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 104874);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120534);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130214);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 53855);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82081);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 104875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 82082);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 104876);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 104877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 53856);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 82083);
					s.is_empty()
				}
				['р', 'а', s @ ..] => {
					super::push(out, 82084);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104878);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 120535);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130215);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135891);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82085);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 82086);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 82087);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104879);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120536);
									s.is_empty()
								}
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
			super::push(out, 53857);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 82088);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 82089);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 104880);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 120537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 104881);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120538);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 120539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', '\u{1}', s @ ..] => {
							super::push(out, 104882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 82090);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 82091);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104884);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 82092);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 104885);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 120540);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82093);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 104886);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 82094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 53858);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82095);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 82096);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 104888);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 120541);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 82097);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 82098);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 53859);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82099);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104890);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 104891);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 104892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120542);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130216);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 120543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 104893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 82100);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 104894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 104895);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120544);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 104896);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 120545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 104897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104898);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 120546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 104899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 53860);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82101);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 104900);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 120547);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130217);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t50::n82955(s, out),
				['у', s @ ..] => super::t50::n82956(s, out),
				['ы', s @ ..] => super::t50::n82957(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t50::n54408(s, out),
		['ч', s @ ..] => super::t50::n54409(s, out),
		['ш', s @ ..] => super::t50::n54410(s, out),
		['ы', s @ ..] => super::t50::n54411(s, out),
		['ь', s @ ..] => super::t50::n54412(s, out),
		['ю', s @ ..] => super::t50::n54413(s, out),
		['я', s @ ..] => super::t50::n54414(s, out),
		_ => false,
	}
}

/// Words ending -елание.
pub(crate) fn n54415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53868);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 82127);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 104927);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 104928);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 120559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 104929);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 82128);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -клание.
pub(crate) fn n54417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53870);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82130);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олание.
pub(crate) fn n54418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53871);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 82131);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 104930);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 120560);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 130219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слание.
pub(crate) fn n54419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53872);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 82132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ылание.
pub(crate) fn n54420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53873);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 82133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мание.
pub(crate) fn n26869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26685);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 53874);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 53875);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 82134);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104931);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 53876);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нание.
pub(crate) fn n26870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26686);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 53877);
			match s {
				[] => true,
				['о', 'к', s @ ..] => {
					super::push(out, 82135);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 104932);
							match s {
								[] => true,
								['з', 'я', s @ ..] => {
									super::push(out, 120561);
									s.is_empty()
								}
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
			super::push(out, 53878);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 82136);
					match s {
						[] => true,
						['о', 'п', 'с', s @ ..] => {
							super::push(out, 104933);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120562);
									match s {
										[] => true,
										['в', '-', s @ ..] => {
											super::push(out, 130220);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 135892);
													match s {
														[] => true,
														['а', s @ ..] => super::t50::n140647(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 135893);
													match s {
														[] => true,
														['а', s @ ..] => super::t50::n140648(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пание.
pub(crate) fn n26871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26687);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53879);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 82137);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 82138);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 82139);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 104934);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 104935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 82140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 53880);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 53881);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 53882);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82142);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104936);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 53883);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82143);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 104937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120563);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 120564);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130221);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 120565);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 130222);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 53884);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 82144);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 82145);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 82146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 53885);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 82147);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рание.
pub(crate) fn n26872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26688);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 53886);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 53887);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 53888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сание.
pub(crate) fn n26873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26689);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53889);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 53890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тание.
pub(crate) fn n26874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26690);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53891);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 104939);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120566);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130223);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 104940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 82148);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 82149);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 82150);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 104941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 82151);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 104942);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 82152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 53892);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 53893);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 82153);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 82154);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 53894);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 82155);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 104944);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 120568);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130224);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135894);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140649(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 82156);
					s.is_empty()
				}
				['м', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 82157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 53895);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 53896);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 53897);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 53898);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 82158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 82159);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 53899);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82160);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 104947);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 120569);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130225);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 104948);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120570);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130226);
											s.is_empty()
										}
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
					super::push(out, 82161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хание.
pub(crate) fn n26875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26691);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 82162);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 82163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 82164);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 82165);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 104950);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120571);
									s.is_empty()
								}
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
			super::push(out, 53901);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 53902);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 53903);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 53904);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82166);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 104951);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 82167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104952);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 104953);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 120573);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130227);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цание.
pub(crate) fn n26876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26692);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53905);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82168);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 53906);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 82169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 53907);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 53908);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чание.
pub(crate) fn n26877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26693);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53909);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 82171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 53910);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82172);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 104955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 53911);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 82173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 53912);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 82174);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104956);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 82175);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 104957);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 120575);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130228);
											s.is_empty()
										}
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
					super::push(out, 82176);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 104958);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120576);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130229);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 135895);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140650(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', 'а', s @ ..] => {
											super::push(out, 130230);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135896);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140651(s, out),
														_ => false,
													}
												}
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
							super::push(out, 104959);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120577);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130231);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 135897);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140652(s, out),
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
									super::push(out, 120578);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 130232);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135898);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140653(s, out),
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
									super::push(out, 120579);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 130233);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135899);
													match s {
														[] => true,
														['л', s @ ..] => super::t50::n140654(s, out),
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
									super::push(out, 120580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 104960);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 104961);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 120581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 82177);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 53913);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82178);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 104963);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 53914);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82179);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104964);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 82180);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 104965);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 53915);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 82181);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 104966);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 120582);
									s.is_empty()
								}
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
			super::push(out, 53916);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82182);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 82183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 53917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82184);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 104967);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 53918);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 82185);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 82186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 53919);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82187);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 104968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шание.
pub(crate) fn n26878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26694);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 53920);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82188);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 82189);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 82190);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 104969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 53921);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 82191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 53922);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 82192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 53923);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 82193);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 104970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 53924);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82194);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 104971);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 53925);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 82195);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104972);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 104973);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 104974);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 120583);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 104975);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 104976);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120584);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 104977);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 120585);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130235);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 53926);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82196);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 104978);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120586);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 120587);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щание.
pub(crate) fn n26879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26695);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 53927);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82197);
					match s {
						[] => true,
						['о', 'и', s @ ..] => {
							super::push(out, 104979);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 120588);
									s.is_empty()
								}
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
			super::push(out, 53928);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 53929);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -плесневение.
pub(crate) fn n137310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135900);
	s.is_empty()
}

/// Words ending -ан-откровение.
pub(crate) fn n137311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135901);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 139215);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141141);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -езлиствение.
pub(crate) fn n137312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135902);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 139216);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141142);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шепелявение.
pub(crate) fn n137313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135903);
	s.is_empty()
}

/// Words ending -дение.
pub(crate) fn n26884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26700);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53937);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82206);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 82207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 53938);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 53939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 82208);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 82209);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 104987);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 120594);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 120595);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 104988);
							match s {
								[] => true,
								['о', s @ ..] => super::t50::n121865(s, out),
								_ => false,
							}
						}
						['в', s @ ..] => super::t50::n106100(s, out),
						['д', s @ ..] => super::t50::n106101(s, out),
						['е', s @ ..] => super::t50::n106102(s, out),
						['з', s @ ..] => super::t50::n106103(s, out),
						['и', s @ ..] => super::t50::n106104(s, out),
						['о', s @ ..] => super::t50::n106105(s, out),
						['т', s @ ..] => super::t50::n106106(s, out),
						['ы', s @ ..] => super::t50::n106107(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t50::n83069(s, out),
				['о', s @ ..] => super::t50::n83070(s, out),
				['р', s @ ..] => super::t50::n83071(s, out),
				['с', s @ ..] => super::t50::n83072(s, out),
				['ъ', s @ ..] => super::t50::n83073(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t50::n54487(s, out),
		['и', s @ ..] => super::t50::n54488(s, out),
		['л', s @ ..] => super::t50::n54489(s, out),
		['р', s @ ..] => super::t50::n54490(s, out),
		['у', s @ ..] => super::t50::n54491(s, out),
		['ю', s @ ..] => super::t50::n54492(s, out),
		['я', s @ ..] => super::t50::n54493(s, out),
		_ => false,
	}
}

/// Words ending -жение.
pub(crate) fn n26886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26702);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53948);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82227);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 105018);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 82228);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 53949);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 53950);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 82229);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105019);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 120639);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130279);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135927);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 130280);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135928);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105020);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 82231);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 105021);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 105022);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 120640);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 120641);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 130281);
											match s {
												[] => true,
												['о', 'д', s @ ..] => {
													super::push(out, 135929);
													match s {
														[] => true,
														['е', 'р', 'п', '-', s @ ..] => super::t50::n140674(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53951);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 53952);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82232);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 120642);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 120643);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130282);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135930);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140675(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 130283);
											match s {
												[] => true,
												['в', 'о', s @ ..] => {
													super::push(out, 135931);
													match s {
														[] => true,
														['т', s @ ..] => super::t50::n140676(s, out),
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
									super::push(out, 120644);
									s.is_empty()
								}
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
			super::push(out, 53953);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82233);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105023);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120645);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130284);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135932);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140677(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 82234);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 82235);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 105024);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 105025);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 120646);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130285);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135933);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n140678(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', '\u{1}', s @ ..] => {
									super::push(out, 120647);
									s.is_empty()
								}
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
			super::push(out, 53954);
			s.is_empty()
		}
		['у', s @ ..] => super::t50::n54502(s, out),
		['ы', s @ ..] => super::t50::n54503(s, out),
		['ю', s @ ..] => super::t50::n54504(s, out),
		['я', s @ ..] => super::t50::n54505(s, out),
		_ => false,
	}
}

/// Words ending -ление.
pub(crate) fn n26889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26705);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53958);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82241);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 82242);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 82243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 82244);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 105035);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120654);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130289);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135937);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140680(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 82245);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 105036);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120655);
									s.is_empty()
								}
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
			super::push(out, 53959);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82246);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 82247);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120656);
									match s {
										[] => true,
										['п', 'у', 'о', s @ ..] => {
											super::push(out, 130290);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135938);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 120657);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 130291);
											s.is_empty()
										}
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
					super::push(out, 82248);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 105037);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120658);
									match s {
										[] => true,
										['п', 'с', s @ ..] => {
											super::push(out, 130292);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 105038);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120659);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130293);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135939);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82249);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 82250);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105039);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120660);
									s.is_empty()
								}
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
			super::push(out, 53960);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 105040);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 105041);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 105042);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 120661);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130294);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 105043);
							match s {
								[] => true,
								['г', 'з', s @ ..] => {
									super::push(out, 120662);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130295);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135940);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140681(s, out),
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
									super::push(out, 120663);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 120664);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130296);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 130297);
											s.is_empty()
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
								['п', s @ ..] => {
									super::push(out, 120665);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 120666);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130298);
											match s {
												[] => true,
												['\u{1}', s @ ..] => super::t50::n137351(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => super::t50::n106159(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t50::n83114(s, out),
				['о', s @ ..] => super::t50::n83115(s, out),
				['т', s @ ..] => super::t50::n83116(s, out),
				['я', s @ ..] => super::t50::n83117(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t50::n54509(s, out),
		['м', s @ ..] => super::t50::n54510(s, out),
		['о', s @ ..] => super::t50::n54511(s, out),
		['п', s @ ..] => super::t50::n54512(s, out),
		['с', s @ ..] => super::t50::n54513(s, out),
		['т', s @ ..] => super::t50::n54514(s, out),
		['ш', s @ ..] => super::t50::n54515(s, out),
		['ы', s @ ..] => super::t50::n54516(s, out),
		['я', s @ ..] => super::t50::n54517(s, out),
		_ => false,
	}
}

/// Words ending -аговремение.
pub(crate) fn n137365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135955);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139255);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 141167);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нение.
pub(crate) fn n26891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26707);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53974);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105063);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 120689);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130316);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 105064);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 105065);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120690);
									s.is_empty()
								}
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
			super::push(out, 53975);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 53976);
			match s {
				[] => true,
				['л', 'ч', s @ ..] => {
					super::push(out, 82268);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 105066);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 120691);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130317);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135956);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82269);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 82270);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105067);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120692);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130318);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135957);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140696(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53977);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 53978);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105068);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 105069);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 120693);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 105070);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 120694);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 130319);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135958);
													match s {
														[] => true,
														['м', s @ ..] => super::t50::n140697(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 130320);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135959);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140698(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105071);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 105072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 53979);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82271);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 82272);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 105073);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 120695);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130321);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135960);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53980);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 53981);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82273);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 105074);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120696);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 130322);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135961);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -остроение.
pub(crate) fn n54532(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 82274);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 105075);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 120697);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 130323);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 135962);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 139259);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n142616(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82275);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105076);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'е', 'ж', 'а', 'н', 'е', 'р', 'т', '\u{1}', s @ ..] => {
			super::push(out, 82276);
			s.is_empty()
		}
		['т', 'е', 'л', 'о', s @ ..] => {
			match s {
				[] => true,
				['м', 'а', 'с', '\u{1}', s @ ..] => {
					super::push(out, 105077);
					s.is_empty()
				}
				['т', 'р', 'е', 'в', '\u{1}', s @ ..] => {
					super::push(out, 105078);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -готерпение.
pub(crate) fn n106199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105079);
	s.is_empty()
}

/// Words ending -рение.
pub(crate) fn n26894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26710);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 53985);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82278);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 82279);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 82280);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 82281);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 82282);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 105082);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120698);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130324);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135963);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53986);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82283);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 105083);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'у', s @ ..] => {
			super::push(out, 53987);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 82284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 53988);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82285);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 82286);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105085);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 120700);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 130325);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135964);
													s.is_empty()
												}
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
							super::push(out, 105086);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120701);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130326);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 135965);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140700(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53989);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82287);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 105087);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120702);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130327);
											s.is_empty()
										}
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
					super::push(out, 82288);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 53990);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 82289);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 105088);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120703);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130328);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135966);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140701(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 82290);
					s.is_empty()
				}
				['ш', 'с', 'а', 'р', 'е', s @ ..] => {
					super::push(out, 82291);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 105089);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120704);
									s.is_empty()
								}
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
			super::push(out, 53991);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82292);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120705);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 120706);
									match s {
										[] => true,
										['а', 'р', 'о', s @ ..] => {
											super::push(out, 130329);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 135967);
													match s {
														[] => true,
														['а', s @ ..] => super::t50::n140702(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 82293);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105090);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120707);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130330);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135968);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82294);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 105091);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 120708);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130331);
											s.is_empty()
										}
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
					super::push(out, 82295);
					match s {
						[] => true,
						['а', s @ ..] => super::t50::n106213(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 53992);
			s.is_empty()
		}
		['у', s @ ..] => super::t50::n54542(s, out),
		_ => false,
	}
}

/// Words ending -сение.
pub(crate) fn n26895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26711);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82298);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105095);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120710);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130333);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135970);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140704(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 82299);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тение.
pub(crate) fn n26896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26712);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 53993);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 82300);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 105096);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 105097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120711);
									s.is_empty()
								}
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
			super::push(out, 53994);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82301);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 105098);
							match s {
								[] => true,
								['е', 'р', 'с', '\u{1}', s @ ..] => {
									super::push(out, 120712);
									s.is_empty()
								}
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
			super::push(out, 53995);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					super::push(out, 82302);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чение.
pub(crate) fn n26897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26713);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 82303);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 105099);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 120713);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130334);
											s.is_empty()
										}
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
					super::push(out, 82304);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 105100);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 120714);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130335);
											s.is_empty()
										}
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
					super::push(out, 82305);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 82306);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 82307);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105102);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 120715);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130336);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135971);
													s.is_empty()
												}
												_ => false,
											}
										}
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
						['м', s @ ..] => {
							super::push(out, 105103);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 105104);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 120716);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130337);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135972);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 53996);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 53997);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82308);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105105);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 120717);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 120718);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130338);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 135973);
													s.is_empty()
												}
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
							super::push(out, 105106);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 105107);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 120719);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 130339);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 105108);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 120720);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130340);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135974);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82309);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105109);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 120721);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130341);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135975);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 130342);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 135976);
													s.is_empty()
												}
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
							super::push(out, 105110);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 105111);
							match s {
								[] => true,
								['д', 'е', 'м', '\u{1}', s @ ..] => {
									super::push(out, 120722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 82310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105112);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 105113);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120723);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130343);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135977);
													s.is_empty()
												}
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
							super::push(out, 105114);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 120724);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130344);
											s.is_empty()
										}
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
					super::push(out, 82311);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105115);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120725);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 105116);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 120726);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130345);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135978);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82312);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 82313);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 82314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t50::n54550(s, out),
		['н', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82315);
					s.is_empty()
				}
				['я', s @ ..] => super::t50::n83187(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t50::n54552(s, out),
		['р', s @ ..] => {
			super::push(out, 53999);
			s.is_empty()
		}
		['у', s @ ..] => super::t50::n54554(s, out),
		['ы', s @ ..] => super::t50::n54555(s, out),
		['ь', s @ ..] => {
			super::push(out, 54002);
			s.is_empty()
		}
		['ю', s @ ..] => super::t50::n54557(s, out),
		['я', s @ ..] => super::t50::n54558(s, out),
		_ => false,
	}
}

/// Words ending -шение.
pub(crate) fn n26898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26714);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54005);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82333);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120745);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'г', 'з', s @ ..] => {
					super::push(out, 82334);
					s.is_empty()
				}
				['р', 'к', '\u{1}', s @ ..] => {
					super::push(out, 82335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 54006);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82336);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105139);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 105140);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 120746);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 130358);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 120747);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 82337);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 82338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 54007);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 54008);
			s.is_empty()
		}
		['у', 'р', s @ ..] => {
			super::push(out, 54009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 82339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 54010);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щение.
pub(crate) fn n26899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26715);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54011);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105141);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 120748);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130359);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 105142);
							match s {
								[] => true,
								['о', '-', s @ ..] => {
									super::push(out, 120749);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130360);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135985);
													match s {
														[] => true,
														['ь', s @ ..] => super::t50::n140710(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 54012);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 54013);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82340);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105144);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120750);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 130361);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135986);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140711(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105145);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 120751);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130362);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135987);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n140712(s, out),
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
									super::push(out, 120752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 105146);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 82341);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 105147);
							match s {
								[] => true,
								['р', 'е', 'п', 'о', s @ ..] => {
									super::push(out, 120754);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 130363);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135988);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140713(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105148);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120755);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130364);
											s.is_empty()
										}
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
					super::push(out, 82342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 54014);
			s.is_empty()
		}
		['о', 'р', s @ ..] => {
			super::push(out, 54015);
			match s {
				[] => true,
				['п', 'о', s @ ..] => {
					super::push(out, 82343);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 54016);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 54017);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82344);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 105150);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 120756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерепокожное.
pub(crate) fn n137417(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136006);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 139283);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -невозможное.
pub(crate) fn n137418(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136007);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 139284);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ожертвенное.
pub(crate) fn n137419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136008);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139285);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141183);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стественное.
pub(crate) fn n137420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136009);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139286);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -несусветное.
pub(crate) fn n137421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136010);
	s.is_empty()
}

/// Words ending -абстрактное.
pub(crate) fn n137422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136011);
	s.is_empty()
}

/// Words ending -неизвестное.
pub(crate) fn n137423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136012);
	s.is_empty()
}

/// Words ending -рнокопытное.
pub(crate) fn n137424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136013);
	s.is_empty()
}

/// Words ending -озачаточное.
pub(crate) fn n137425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136014);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139287);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ноклеточное.
pub(crate) fn n137426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136015);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 139288);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141184);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ациональное.
pub(crate) fn n137427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136016);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139289);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 141185);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дсознательное.
pub(crate) fn n137428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136017);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139290);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 141186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ное-рыльное.
pub(crate) fn n137429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136018);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 139291);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141187);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рахопиздище.
pub(crate) fn n137439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136028);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 139295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юта-убежище.
pub(crate) fn n137440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136029);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139296);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141189);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 142277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонище.
pub(crate) fn n55087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54528);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 82762);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 105426);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фонище.
pub(crate) fn n55088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54529);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 82763);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 105427);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 120915);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 130446);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136030);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 139297);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнище.
pub(crate) fn n27357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27170);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54530);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 82764);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тнище.
pub(crate) fn n27358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27171);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 54531);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82765);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105429);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унище.
pub(crate) fn n27359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27172);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 54532);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 54533);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82767);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 105430);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 54534);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 82768);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янище.
pub(crate) fn n27360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27173);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 54535);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 82769);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105432);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 120917);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130447);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54536);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 82770);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 82771);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 105434);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 120918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аргументище.
pub(crate) fn n137442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136031);
	s.is_empty()
}

/// Words ending -одоворотище.
pub(crate) fn n137443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136032);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 139298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сквознячище.
pub(crate) fn n137444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136033);
	s.is_empty()
}

/// Words ending -домонадовые.
pub(crate) fn n137448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136037);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ставридовые.
pub(crate) fn n137449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136038);
	s.is_empty()
}

/// Words ending -икобразовые.
pub(crate) fn n137450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136039);
	s.is_empty()
}

/// Words ending -ковые.
pub(crate) fn n27420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27233);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 54649);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82879);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 105511);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120961);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 130471);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 136040);
													match s {
														[] => true,
														['м', s @ ..] => super::t50::n140743(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54650);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 82880);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105512);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 120962);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130472);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 136041);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54651);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 82881);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 105513);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 120963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 105514);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 120964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 105515);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 120965);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 130473);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136042);
													match s {
														[] => true,
														['л', s @ ..] => super::t50::n140744(s, out),
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
									super::push(out, 120966);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130474);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136043);
													s.is_empty()
												}
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
							super::push(out, 105516);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 105517);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 120968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 105518);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 120969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 105519);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 120970);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130475);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 136044);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82882);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105520);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 120971);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130476);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136045);
													s.is_empty()
												}
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
							super::push(out, 105521);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 120972);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 130477);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136046);
													match s {
														[] => true,
														['н', s @ ..] => super::t50::n140745(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105522);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 120973);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 82883);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105523);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 120974);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 130478);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 105524);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 120975);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 130479);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 136047);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140746(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => super::t50::n106648(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t50::n55211(s, out),
		['о', s @ ..] => {
			super::push(out, 54653);
			match s {
				[] => true,
				['с', s @ ..] => super::t50::n83763(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t50::n55213(s, out),
		['с', s @ ..] => super::t50::n55214(s, out),
		['т', s @ ..] => super::t50::n55215(s, out),
		['у', s @ ..] => super::t50::n55216(s, out),
		['ч', s @ ..] => super::t50::n55217(s, out),
		['ш', s @ ..] => super::t50::n55218(s, out),
		['я', s @ ..] => super::t50::n55219(s, out),
		_ => false,
	}
}

/// Words ending -иаловые.
pub(crate) fn n83777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82903);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 105547);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 120998);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 130497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лловые.
pub(crate) fn n55221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54662);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 82904);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105548);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 120999);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 130498);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136057);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 139309);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оловые.
pub(crate) fn n55222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54663);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 82905);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 105549);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 121000);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 130499);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 136058);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139310);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятловые.
pub(crate) fn n83780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82906);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 105550);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уловые.
pub(crate) fn n55224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54665);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 82907);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 105551);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 121001);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 130500);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136059);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139311);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 82908);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105552);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мовые.
pub(crate) fn n27422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27235);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 54666);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 82909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 54667);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82910);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105553);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новые.
pub(crate) fn n27423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27236);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54668);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 82911);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 105554);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121003);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130501);
											s.is_empty()
										}
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
					super::push(out, 82912);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 105555);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121004);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130502);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54669);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82913);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105556);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121005);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130503);
											s.is_empty()
										}
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
					super::push(out, 82914);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 105557);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 121006);
									s.is_empty()
								}
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
			super::push(out, 54670);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82915);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105558);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121007);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130504);
											s.is_empty()
										}
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
					super::push(out, 82916);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105559);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121008);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 130505);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 136060);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54671);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 82917);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105560);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121009);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130506);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136061);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140752(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54672);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 82918);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 54673);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 82919);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 105562);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повые.
pub(crate) fn n27424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27237);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 54674);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82920);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровые.
pub(crate) fn n27425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27238);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54675);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 82921);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105564);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 121011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 82922);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105565);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121012);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130507);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54676);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 82923);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105566);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121013);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 82924);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105567);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 121014);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130508);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136062);
													match s {
														[] => true,
														['ф', s @ ..] => super::t50::n140753(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54677);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 82925);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 82926);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 54678);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 82927);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 105570);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 121015);
									s.is_empty()
								}
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
			super::push(out, 54679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 82928);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совые.
pub(crate) fn n27426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27239);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 54680);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 82929);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 105571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 82930);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 105572);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121016);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130509);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136063);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82931);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105573);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 121017);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130510);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136064);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n140754(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 82932);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 105575);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121018);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130511);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136065);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 82933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 54681);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82934);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105576);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121019);
									s.is_empty()
								}
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
			super::push(out, 54682);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82935);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 105577);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 121020);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130512);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 136066);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54683);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82936);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 105578);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 121021);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130513);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товые.
pub(crate) fn n27427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27240);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 54684);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 82937);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 105579);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 121022);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130514);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136067);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140755(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54685);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 82938);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 105580);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121023);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130515);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136068);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 121024);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130516);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 136069);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54686);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 82939);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 105581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 54687);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 82940);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105582);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121025);
									s.is_empty()
								}
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
			super::push(out, 54688);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 82941);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховые.
pub(crate) fn n27428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27241);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 54689);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 82942);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 105583);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчатоклювые.
pub(crate) fn n137481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136070);
	s.is_empty()
}

/// Words ending -фалеобразные.
pub(crate) fn n137487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136076);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 139319);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141196);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -авлеобразные.
pub(crate) fn n137488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136077);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139320);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сосеобразные.
pub(crate) fn n137489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136078);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 139321);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гусеобразные.
pub(crate) fn n137490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136079);
	s.is_empty()
}

/// Words ending -акшеобразные.
pub(crate) fn n137491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136080);
	s.is_empty()
}

/// Words ending -ракообразные.
pub(crate) fn n137492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136081);
	s.is_empty()
}

/// Words ending -ескообразные.
pub(crate) fn n137493(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136082);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139322);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юскообразные.
pub(crate) fn n137494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136083);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 139323);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 141199);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аукообразные.
pub(crate) fn n137495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136084);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 139324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 141200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -медиаданные.
pub(crate) fn n137496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136085);
	s.is_empty()
}

/// Words ending -видеоданные.
pub(crate) fn n137497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136086);
	s.is_empty()
}

/// Words ending -аудиоданные.
pub(crate) fn n137498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136087);
	s.is_empty()
}

/// Words ending -омиссионные.
pub(crate) fn n137499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136088);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 139325);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -асно-черные.
pub(crate) fn n137500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136089);
	s.is_empty()
}

/// Words ending -игиецветные.
pub(crate) fn n137501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136090);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 139326);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 141201);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142280);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емнецветные.
pub(crate) fn n137502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136091);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139327);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -розоцветные.
pub(crate) fn n137503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136092);
	s.is_empty()
}

/// Words ending -откоцветные.
pub(crate) fn n137504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136093);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 139328);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 141202);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 142281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -естоцветные.
pub(crate) fn n137505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136094);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139329);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трехцветные.
pub(crate) fn n137506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136095);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 139330);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стохоботные.
pub(crate) fn n137507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136096);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139331);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -стные.
pub(crate) fn n27454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27267);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 54719);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 82974);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105619);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 121068);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 130558);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 136097);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытные.
pub(crate) fn n27455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27268);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 54720);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 82975);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105620);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121069);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 130559);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136098);
													match s {
														[] => true,
														['а', s @ ..] => super::t50::n140772(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -верхурочные.
pub(crate) fn n137510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136099);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 139333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -премиальные.
pub(crate) fn n137511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136100);
	s.is_empty()
}

/// Words ending -латеральные.
pub(crate) fn n137512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136101);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 139334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ставрополье.
pub(crate) fn n137516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136105);
	s.is_empty()
}

/// The word подкуполье.
pub(crate) fn n137517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136106);
	s.is_empty()
}

/// Words ending -ванье.
pub(crate) fn n27557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27369);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54831);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 83054);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105674);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 83055);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 105675);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 83056);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 83057);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 54832);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 83058);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 83059);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 83060);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 83061);
					s.is_empty()
				}
				['ч', 'т', s @ ..] => {
					super::push(out, 83062);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105678);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121103);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130578);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54833);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 83063);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 83064);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 83065);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 54834);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83066);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 83067);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105679);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121104);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130579);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 105680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121105);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 83068);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105681);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121106);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130580);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 105682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 83069);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 83070);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105683);
							s.is_empty()
						}
						['е', 'р', s @ ..] => {
							super::push(out, 105684);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121107);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130581);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136107);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140775(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105685);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121108);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130582);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136108);
													s.is_empty()
												}
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
							super::push(out, 105686);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 121109);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130583);
											s.is_empty()
										}
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
					super::push(out, 83071);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 83072);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 105687);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121110);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130584);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 105688);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 105689);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121111);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130585);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136109);
													s.is_empty()
												}
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
							super::push(out, 105690);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121112);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130586);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 105691);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121113);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130587);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 105692);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121114);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130588);
											s.is_empty()
										}
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
					super::push(out, 83073);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 83074);
					match s {
						[] => true,
						['ы', s @ ..] => super::t50::n106817(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t50::n83951(s, out),
				['х', s @ ..] => {
					super::push(out, 83076);
					s.is_empty()
				}
				['ч', s @ ..] => super::t50::n83953(s, out),
				['ш', s @ ..] => super::t50::n83954(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t50::n55394(s, out),
		['р', s @ ..] => super::t50::n55395(s, out),
		['у', s @ ..] => super::t50::n55396(s, out),
		['ы', s @ ..] => super::t50::n55397(s, out),
		_ => false,
	}
}

/// Words ending -канье.
pub(crate) fn n27563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27375);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83125);
					s.is_empty()
				}
				['к', 'с', s @ ..] => {
					super::push(out, 83126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', '\u{1}', s @ ..] => {
					super::push(out, 83127);
					s.is_empty()
				}
				['т', 'о', s @ ..] => {
					super::push(out, 83128);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 105759);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121172);
									s.is_empty()
								}
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
			super::push(out, 54863);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83129);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 83130);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 105760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 83131);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 83132);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105761);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121173);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130625);
											s.is_empty()
										}
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
					super::push(out, 83133);
					match s {
						[] => true,
						['а', 'н', s @ ..] => {
							super::push(out, 105762);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 105763);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121175);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130626);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54864);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83134);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 83135);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105764);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 83136);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 105765);
							s.is_empty()
						}
						['к', 'е', s @ ..] => {
							super::push(out, 105766);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121177);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130627);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136130);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 83137);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 105767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121178);
									s.is_empty()
								}
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
			super::push(out, 54865);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 54866);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83138);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 83139);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 105769);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121179);
									s.is_empty()
								}
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
			super::push(out, 54867);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 54868);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 83140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 54869);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83141);
					s.is_empty()
				}
				['о', 'м', s @ ..] => {
					super::push(out, 83142);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105770);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 83143);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 105771);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 121181);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 130628);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136131);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54870);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83144);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 105772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 105773);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 105774);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 121183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105775);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121184);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 105776);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 105777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105778);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 105779);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 83145);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 105780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105781);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121187);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 121188);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 105782);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 83146);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 105783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 54871);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 54872);
			match s {
				[] => true,
				['н', s @ ..] => super::t50::n84033(s, out),
				['т', s @ ..] => super::t50::n84034(s, out),
				['ц', s @ ..] => super::t50::n84035(s, out),
				_ => false,
			}
		}
		['ч', s @ ..] => super::t50::n55435(s, out),
		['ы', s @ ..] => super::t50::n55436(s, out),
		['ь', s @ ..] => super::t50::n55437(s, out),
		['ю', s @ ..] => super::t50::n55438(s, out),
		['я', s @ ..] => super::t50::n55439(s, out),
		_ => false,
	}
}

/// Words ending -деланье.
pub(crate) fn n84044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83157);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 105797);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 105798);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 121197);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оланье.
pub(crate) fn n55443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54881);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 83159);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 105799);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121198);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 130632);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136133);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сланье.
pub(crate) fn n55444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54882);
	s.is_empty()
}

/// Words ending -тланье.
pub(crate) fn n55445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54883);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 83160);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыланье.
pub(crate) fn n55446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54884);
	s.is_empty()
}

/// Words ending -манье.
pub(crate) fn n27565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27377);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 54885);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 83161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 54886);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 83162);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 105800);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 121199);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130633);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 136134);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54887);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 83163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нанье.
pub(crate) fn n27566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27378);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 54888);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 54889);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -панье.
pub(crate) fn n27567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27379);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 54890);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 54891);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 83164);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 105802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 83165);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 105803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 54892);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 83166);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105804);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 83167);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 105805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 83168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 54893);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 83169);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105806);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 121201);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130634);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 136135);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 83170);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 105807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 83171);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105808);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 83172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 54894);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 83173);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 105809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 54895);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 54896);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 83174);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 83175);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 105810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 83176);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 83177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 54897);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 83178);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 54898);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 83179);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 105813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 54899);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 83180);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 83181);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ранье.
pub(crate) fn n27568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27380);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54900);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 83182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 54901);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 54902);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 83183);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 105814);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 121203);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130635);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54903);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 83184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 54904);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 54905);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 54906);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 83185);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -санье.
pub(crate) fn n27569(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27381);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54907);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 54908);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 83186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -танье.
pub(crate) fn n27570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27382);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54909);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83187);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105815);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121204);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 130636);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 105816);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 83188);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 105817);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 121205);
									s.is_empty()
								}
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
			super::push(out, 54910);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 54911);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 54912);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 83189);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 105818);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 121206);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130637);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54913);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 83190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 54914);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 54915);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 105819);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 121207);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130638);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 105820);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 105821);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121208);
									s.is_empty()
								}
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
			super::push(out, 54916);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 83191);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 83192);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 54917);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83193);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 105824);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 121209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 105825);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 105826);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 121210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 83194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 54918);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 83195);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 105827);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121211);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ханье.
pub(crate) fn n27571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27383);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54919);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83196);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 83197);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105828);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 83198);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 83199);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 83200);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105829);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 105830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 54920);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 54921);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 54922);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83202);
					s.is_empty()
				}
				['б', '\u{1}', s @ ..] => {
					super::push(out, 83203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 54923);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 83204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 54924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 54925);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 83206);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 105831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 83207);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105832);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цанье.
pub(crate) fn n27572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27384);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54926);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 83208);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 83209);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 83210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 54927);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 83211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 54928);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 83212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 54929);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 83213);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105834);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 83214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чанье.
pub(crate) fn n27573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27385);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54930);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 83215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 54931);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 83216);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105835);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 105836);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121214);
									s.is_empty()
								}
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
			super::push(out, 54932);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 83217);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105837);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 83218);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 105838);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 121216);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130639);
											s.is_empty()
										}
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
					super::push(out, 83219);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 105839);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121217);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130640);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136136);
													match s {
														[] => true,
														['д', s @ ..] => super::t50::n140786(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105840);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121218);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130641);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 136137);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140787(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105841);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121219);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 130642);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136138);
													match s {
														[] => true,
														['н', s @ ..] => super::t50::n140788(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105842);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121220);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 130643);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136139);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140789(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105843);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 105844);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121221);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130644);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 136140);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140790(s, out),
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
									super::push(out, 121222);
									match s {
										[] => true,
										['з', s @ ..] => super::t50::n132002(s, out),
										_ => false,
									}
								}
								['ы', s @ ..] => super::t50::n122496(s, out),
								['я', s @ ..] => {
									super::push(out, 121224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t50::n106973(s, out),
						['т', s @ ..] => {
							super::push(out, 105846);
							s.is_empty()
						}
						['ь', s @ ..] => super::t50::n106975(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t50::n84108(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t50::n55497(s, out),
		['н', s @ ..] => super::t50::n55498(s, out),
		['о', s @ ..] => super::t50::n55499(s, out),
		['р', s @ ..] => super::t50::n55500(s, out),
		['у', s @ ..] => super::t50::n55501(s, out),
		['ы', s @ ..] => super::t50::n55502(s, out),
		['я', s @ ..] => super::t50::n55503(s, out),
		_ => false,
	}
}

/// Words ending -шанье.
pub(crate) fn n27574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27386);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83233);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105858);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 83234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 54940);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 83235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 54941);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 83236);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 54942);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 83237);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 54943);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 83238);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 83239);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121229);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 121230);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 130649);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54944);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 83240);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 105859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щанье.
pub(crate) fn n27575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27387);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 54945);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83241);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шепелявенье.
pub(crate) fn n137555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136144);
	s.is_empty()
}

/// Words ending -киноведенье.
pub(crate) fn n137556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136145);
	s.is_empty()
}

/// Words ending -атуроведенье.
pub(crate) fn n137557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136146);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 139354);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141215);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -женье.
pub(crate) fn n27578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27390);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 83251);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 105872);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121244);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 83252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'н', s @ ..] => {
			super::push(out, 54953);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'в', s @ ..] => {
			super::push(out, 54954);
			match s {
				[] => true,
				['д', 'з', s @ ..] => {
					super::push(out, 83254);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 105873);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 121245);
									s.is_empty()
								}
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
			super::push(out, 54955);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 54956);
			s.is_empty()
		}
		['у', 'р', s @ ..] => {
			super::push(out, 54957);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83255);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 105874);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 121246);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130657);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136147);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54958);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 83256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105875);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 105876);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 121247);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 130658);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 136148);
													match s {
														[] => true,
														['к', s @ ..] => super::t50::n140795(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105877);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ленье.
pub(crate) fn n27579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27391);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 54959);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83257);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 105878);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 121248);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130659);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121249);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130660);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 136149);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140796(s, out),
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
									super::push(out, 121250);
									s.is_empty()
								}
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
			super::push(out, 54960);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 54961);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83258);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 105879);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121251);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 130661);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 136150);
													match s {
														[] => true,
														['п', s @ ..] => super::t50::n140797(s, out),
														_ => false,
													}
												}
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
							super::push(out, 105880);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121252);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 130662);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 136151);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -менье.
pub(crate) fn n27580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27392);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			super::push(out, 54963);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 83259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 54964);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 83260);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 83261);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 105881);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121253);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 121254);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 130663);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54965);
			s.is_empty()
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 83262);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105882);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 121255);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 130664);
											s.is_empty()
										}
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
					super::push(out, 83263);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренье.
pub(crate) fn n27584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27396);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 54967);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83264);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 83265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 54968);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83266);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 105885);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121256);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'м', s @ ..] => {
			super::push(out, 54969);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83267);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 105886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121257);
									s.is_empty()
								}
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
			super::push(out, 54970);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 54971);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83268);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 83269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 105887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 83270);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 105888);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 121258);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 130665);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 136152);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140798(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 54972);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 83271);
					match s {
						[] => true,
						['м', 'с', 'у', s @ ..] => {
							super::push(out, 105889);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 121259);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 130666);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 136153);
													match s {
														[] => true,
														['л', s @ ..] => super::t50::n140799(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ретенье.
pub(crate) fn n55541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54973);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 83272);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 105890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

