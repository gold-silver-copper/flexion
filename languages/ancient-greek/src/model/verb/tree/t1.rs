use flexion::Path;

/// Words ending -υμι.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1133);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4167);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 4168);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5668);
											match s {
												[] => true,
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
			super::push(out, 348);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1134);
					match s {
						[] => true,
						['α', s @ ..] => super::t2::n2573(s, out),
						['η', s @ ..] => super::t2::n2574(s, out),
						['ι', s @ ..] => super::t2::n2575(s, out),
						['ρ', s @ ..] => super::t2::n2576(s, out),
						['υ', s @ ..] => {
							super::push(out, 2525);
							match s {
								[] => true,
								['ε', s @ ..] => super::t2::n4411(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => super::t2::n1140(s, out),
				['κ', s @ ..] => super::t2::n1141(s, out),
				['μ', s @ ..] => super::t2::n1142(s, out),
				['ν', s @ ..] => super::t2::n1143(s, out),
				['ρ', s @ ..] => super::t2::n1144(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωμι.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4195);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 4196);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 5691);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6849);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8801(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 5692);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6850);
													match s {
														[] => true,
														['μ', s @ ..] => super::t2::n8802(s, out),
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
									super::push(out, 4197);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5693);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6851);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8803(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 5694);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 6852);
													match s {
														[] => true,
														['α', s @ ..] => super::t2::n8804(s, out),
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
									super::push(out, 4198);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5695);
											match s {
												[] => true,
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
									super::push(out, 4199);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5696);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6853);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8805(s, out),
														['τ', s @ ..] => super::t2::n8806(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 5697);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6854);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8807(s, out),
														['α', s @ ..] => super::t2::n8808(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τερηθειην.
pub(crate) fn n7724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6865);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 7583);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκοιην.
pub(crate) fn n7725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6866);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 7584);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θνηκοιτην.
pub(crate) fn n7726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6867);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 7585);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εγονοιτην.
pub(crate) fn n7727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6868);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 7586);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκασιν.
pub(crate) fn n7728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6869);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 7587);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκωσιν.
pub(crate) fn n7729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6870);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 7588);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γεγονωσιν.
pub(crate) fn n7730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6871);
	s.is_empty()
}

/// Words ending -εγενησθον.
pub(crate) fn n7731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6872);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 7589);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word προειπον.
pub(crate) fn n7732(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word παρειπον.
pub(crate) fn n7733(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -προσειπον.
pub(crate) fn n7734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6873);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ορον.
pub(crate) fn n378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 378);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1172);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
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

/// Words ending -τον.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1173);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2569);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4233);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 5724);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6874);
													match s {
														[] => true,
														['τ', s @ ..] => super::t2::n8823(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 380);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2570);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4234);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 5725);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6875);
													match s {
														[] => true,
														['τ', s @ ..] => super::t2::n8824(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1175);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2571);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4235);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5726);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 6876);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 381);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2572);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4236);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5727);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 6877);
													match s {
														[] => true,
														['σ', s @ ..] => super::t2::n8825(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1177);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2573);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4237);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 5728);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6878);
													match s {
														[] => true,
														['τ', s @ ..] => super::t2::n8826(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1178);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2574);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4238);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5729);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 6879);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 382);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2575);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 4239);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 5730);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6880);
													match s {
														[] => true,
														['τ', s @ ..] => super::t2::n8827(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1180);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2576);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 4240);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 5731);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 6881);
													match s {
														[] => true,
														['ε', s @ ..] => super::t2::n8828(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2577);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4241);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 5732);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6882);
													match s {
														[] => true,
														['γ', s @ ..] => super::t2::n8829(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εγενησθων.
pub(crate) fn n7744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6883);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 7597);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εθνηκετων.
pub(crate) fn n7745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6884);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 7598);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γεγονετων.
pub(crate) fn n7746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6885);
	s.is_empty()
}

/// Words ending -θνηκοντων.
pub(crate) fn n7747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6886);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 7599);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 7958);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εγονοντων.
pub(crate) fn n7748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6887);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 7600);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word κολυμβαω.
pub(crate) fn n7754(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -επιμειδαω.
pub(crate) fn n7755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6893);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προσαυδαω.
pub(crate) fn n7756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6894);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word μεταυδαω.
pub(crate) fn n7757(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ιαω.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 410);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1208);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2600);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4262);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5750);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 5751);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 6895);
													match s {
														[] => true,
														['α', s @ ..] => super::t2::n8838(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 411);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2601);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4263);
									match s {
										[] => true,
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
			super::push(out, 412);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
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
			super::push(out, 413);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2602);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4264);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5752);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6896);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 414);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2603);
							match s {
								[] => true,
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
			super::push(out, 415);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2604);
							match s {
								[] => true,
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
			super::push(out, 416);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2605);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4265);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 5753);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6897);
													s.is_empty()
												}
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
							super::push(out, 2606);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4266);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5754);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6898);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8839(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1215);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2607);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2608);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4268);
									match s {
										[] => true,
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

/// Words ending -καω.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
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
			super::push(out, 418);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1218);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2609);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4269);
									match s {
										[] => true,
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
							super::push(out, 2610);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4270);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5755);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 6899);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8840(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 419);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1219);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2611);
							match s {
								[] => true,
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
			super::push(out, 420);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1220);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2612);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4271);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 5756);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6900);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8841(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 421);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1221);
					match s {
						[] => true,
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

/// Words ending -λαω.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1222);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2613);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4272);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5757);
											match s {
												[] => true,
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
					super::push(out, 1223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2614);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2615);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4273);
									match s {
										[] => true,
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
			super::push(out, 423);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2616);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4274);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5758);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 6901);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8842(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2617);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4275);
									match s {
										[] => true,
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
			super::push(out, 424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1227);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2618);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4276);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5759);
											match s {
												[] => true,
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
					super::push(out, 1228);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2619);
							match s {
								[] => true,
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
			super::push(out, 426);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1229);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2620);
							match s {
								[] => true,
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
			super::push(out, 427);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1230);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2621);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4277);
									match s {
										[] => true,
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
			super::push(out, 428);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
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

/// Words ending -μαω.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1233);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2622);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4278);
									s.is_empty()
								}
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
			super::push(out, 431);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1234);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2623);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4279);
									match s {
										[] => true,
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
			super::push(out, 432);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1235);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2624);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 4280);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 5760);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6902);
													match s {
														[] => true,
														['κ', s @ ..] => super::t2::n8843(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2625);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2626);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4281);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5761);
											match s {
												[] => true,
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
							super::push(out, 2627);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4282);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5762);
											match s {
												[] => true,
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
			super::push(out, 433);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1237);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2628);
							match s {
								[] => true,
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
			super::push(out, 434);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1238);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2629);
							match s {
								[] => true,
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
			super::push(out, 435);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1239);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2631);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2632);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4283);
									match s {
										[] => true,
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
			super::push(out, 437);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1242);
					match s {
						[] => true,
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

/// Words ending -ναω.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 439);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 440);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2634);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2635);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4285);
									match s {
										[] => true,
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
					super::push(out, 1246);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2636);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4286);
									match s {
										[] => true,
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
							super::push(out, 2637);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4287);
									s.is_empty()
								}
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
			super::push(out, 441);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1248);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2638);
							match s {
								[] => true,
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
			super::push(out, 442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1250);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2639);
							match s {
								[] => true,
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
					super::push(out, 1251);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2640);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4288);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 5763);
											match s {
												[] => true,
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
			super::push(out, 444);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2641);
							match s {
								[] => true,
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
			super::push(out, 445);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2642);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2643);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4289);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5764);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6903);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8844(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 446);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2644);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4290);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5765);
											match s {
												[] => true,
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
		['υ', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2645);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2646);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4291);
									match s {
										[] => true,
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

/// Words ending -οαω.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1257);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2647);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4292);
									match s {
										[] => true,
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
							super::push(out, 2648);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4293);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5766);
											match s {
												[] => true,
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
		['γ', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -παω.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1259);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2649);
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
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1261);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2650);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4295);
									match s {
										[] => true,
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
							super::push(out, 2651);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4296);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5767);
											match s {
												[] => true,
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
					super::push(out, 1262);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2652);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4297);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5768);
											match s {
												[] => true,
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
		['υ', s @ ..] => {
			super::push(out, 452);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2653);
							match s {
								[] => true,
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
		['ω', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2654);
							match s {
								[] => true,
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

/// Words ending -ραω.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 454);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1267);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1268);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2655);
							match s {
								[] => true,
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
					super::push(out, 1269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2656);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2657);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4298);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1270);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2658);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4299);
									match s {
										[] => true,
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
			super::push(out, 457);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1271);
					match s {
						[] => true,
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
			super::push(out, 458);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1272);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2659);
							match s {
								[] => true,
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
			super::push(out, 459);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1273);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1274);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2660);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4300);
									match s {
										[] => true,
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
					super::push(out, 1275);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2661);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4301);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5769);
											match s {
												[] => true,
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
					super::push(out, 1276);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2662);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4302);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2663);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4303);
									match s {
										[] => true,
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
					super::push(out, 1278);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2664);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4304);
									match s {
										[] => true,
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
					super::push(out, 1279);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2665);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4305);
									match s {
										[] => true,
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
				['φ', s @ ..] => {
					super::push(out, 1280);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2666);
							match s {
								[] => true,
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
			super::push(out, 460);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1281);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2667);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4306);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5770);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6904);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8845(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 461);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1282);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2668);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4307);
									match s {
										[] => true,
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
			super::push(out, 462);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1283);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1284);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2669);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4308);
									match s {
										[] => true,
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
			super::push(out, 463);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1285);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2670);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4309);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5771);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 6905);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8846(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σαω.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1287);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2671);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4310);
									match s {
										[] => true,
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
			super::push(out, 466);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2672);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4311);
									match s {
										[] => true,
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

/// Words ending -ταω.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2673);
							match s {
								[] => true,
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
		['ε', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1290);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2674);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4312);
									match s {
										[] => true,
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
			super::push(out, 469);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1291);
					match s {
						[] => true,
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
			super::push(out, 470);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1292);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2675);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4313);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5772);
											s.is_empty()
										}
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
					super::push(out, 1293);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2676);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 4314);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5773);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6906);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8847(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 471);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1294);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2677);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2678);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4315);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5774);
											match s {
												[] => true,
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
							super::push(out, 2679);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4316);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 5775);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 6907);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8848(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 472);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1295);
					match s {
						[] => true,
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
			super::push(out, 473);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1296);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2680);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 2681);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4317);
									match s {
										[] => true,
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
					super::push(out, 1297);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2682);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4318);
									match s {
										[] => true,
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
			super::push(out, 474);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1298);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2683);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4319);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5776);
											match s {
												[] => true,
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
		['υ', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1299);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2684);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4320);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 5777);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6908);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 476);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1301);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2685);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4321);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 4322);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5778);
											match s {
												[] => true,
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

/// Words ending -θυαω.
pub(crate) fn n477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['χ', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φαω.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1303);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2687);
							match s {
								[] => true,
								['ψ', s @ ..] => {
									super::push(out, 4324);
									match s {
										[] => true,
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
			super::push(out, 479);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1304);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2688);
							match s {
								[] => true,
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

/// Words ending -χαω.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1305);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2689);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4325);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 5779);
											match s {
												[] => true,
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
		['ο', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1306);
					match s {
						[] => true,
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
			super::push(out, 482);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1307);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψαω.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 483);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1308);
					match s {
						[] => true,
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

/// Words ending -ωαω.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1309);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ανταναγω.
pub(crate) fn n7792(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -λεγω.
pub(crate) fn n504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 504);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1335);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2715);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2716);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4337);
							match s {
								[] => true,
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
					super::push(out, 2717);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4338);
							match s {
								[] => true,
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
					super::push(out, 2718);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4339);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 5787);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6911);
											s.is_empty()
										}
										['γ', s @ ..] => {
											super::push(out, 6912);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7607);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n9483(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1337);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2719);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4340);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5788);
									match s {
										[] => true,
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
				['φ', s @ ..] => {
					super::push(out, 2720);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4341);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5789);
									match s {
										[] => true,
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
			super::push(out, 1338);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2721);
					match s {
						[] => true,
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
			super::push(out, 1339);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2722);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4342);
							match s {
								[] => true,
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
			super::push(out, 1340);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2723);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4343);
							match s {
								[] => true,
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
		['φ', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τεγω.
pub(crate) fn n506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 506);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 1343);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψεγω.
pub(crate) fn n507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 507);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηγω.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 508);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2724);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4344);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5790);
											match s {
												[] => true,
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
					super::push(out, 1345);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2725);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4345);
									match s {
										[] => true,
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
			super::push(out, 509);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1346);
					match s {
						[] => true,
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

/// Words ending -ιγω.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2726);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4346);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 4347);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5791);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 6913);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8851(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 511);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1348);
					match s {
						[] => true,
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
			super::push(out, 512);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1349);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1350);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2727);
							match s {
								[] => true,
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

/// Words ending -λγω.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1352);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2728);
							match s {
								[] => true,
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

/// Words ending -ργω.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1353);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1354);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2730);
							match s {
								[] => true,
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
					super::push(out, 1356);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2731);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4348);
									s.is_empty()
								}
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
			super::push(out, 515);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2732);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2733);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4349);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5792);
											match s {
												[] => true,
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

/// Words ending -σγω.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1358);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2734);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4350);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5793);
											match s {
												[] => true,
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
							super::push(out, 2735);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4351);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 5794);
											match s {
												[] => true,
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

/// Words ending -υγω.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2736);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2737);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4352);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 5795);
											match s {
												[] => true,
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
									super::push(out, 4353);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5796);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 6914);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8852(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2738);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4354);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 5797);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 6915);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8853(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2739);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4355);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5798);
											match s {
												[] => true,
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
		['ρ', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1360);
					match s {
						[] => true,
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

/// Words ending -ωγω.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word υποδειδω.
pub(crate) fn n7806(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -γεω.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2752);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4361);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5800);
											s.is_empty()
										}
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
					super::push(out, 1381);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2753);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4362);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5801);
											match s {
												[] => true,
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
							super::push(out, 2754);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4363);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 5802);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6916);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t2::n8854(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1382);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2755);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4364);
									match s {
										[] => true,
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
					super::push(out, 1383);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2756);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4365);
									match s {
										[] => true,
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
		['γ', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2757);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4366);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 5803);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6917);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8855(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 537);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1385);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2758);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2759);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4368);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 5804);
											match s {
												[] => true,
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
					super::push(out, 1387);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2760);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 4369);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 5805);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6918);
													match s {
														[] => true,
														['ε', s @ ..] => super::t2::n8856(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1388);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2761);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4370);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 5806);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 6919);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8857(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 538);
			match s {
				[] => true,
				['ῥ', s @ ..] => {
					super::push(out, 1389);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2762);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2763);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t2::n540(s, out),
		['ρ', s @ ..] => super::t2::n541(s, out),
		['υ', s @ ..] => super::t2::n542(s, out),
		['ω', s @ ..] => super::t2::n543(s, out),
		_ => false,
	}
}

/// The word αποκηδεω.
pub(crate) fn n7838(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -γωδεω.
pub(crate) fn n1429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1412);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2787);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 4398);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5834);
							match s {
								[] => true,
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

/// Words ending -ρωδεω.
pub(crate) fn n1430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1413);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 2788);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4399);
					match s {
						[] => true,
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

/// Words ending -εεω.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
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

/// Words ending -ζεω.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 553);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
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

/// Words ending -θεω.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 555);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2789);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4400);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 5835);
											match s {
												[] => true,
												['φ', s @ ..] => {
													super::push(out, 6943);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8879(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1417);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2790);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4401);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5836);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 6944);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8880(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2791);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4402);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 5837);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6945);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8881(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 557);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2793);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 4403);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 5838);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6946);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8882(s, out),
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
									super::push(out, 4404);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5839);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6947);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8883(s, out),
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
									super::push(out, 4405);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 5840);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 6948);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8884(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 558);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1421);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2794);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4406);
									match s {
										[] => true,
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
		['λ', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2795);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2796);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4407);
									match s {
										[] => true,
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
			super::push(out, 560);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2798);
							match s {
								[] => true,
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
			super::push(out, 561);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
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
			super::push(out, 562);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1426);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2799);
							match s {
								[] => true,
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
			super::push(out, 563);
			match s {
				[] => true,
				['μ', s @ ..] => super::t2::n1444(s, out),
				['ο', s @ ..] => super::t2::n1445(s, out),
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['ο', s @ ..] => super::t2::n1446(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t2::n565(s, out),
		_ => false,
	}
}

/// Words ending -ιεω.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2805);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2806);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4412);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5843);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6951);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8888(s, out),
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
									super::push(out, 4413);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 5844);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6952);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8889(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2807);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4414);
									match s {
										[] => true,
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
							super::push(out, 2808);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 4415);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5845);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 6953);
													match s {
														[] => true,
														['α', s @ ..] => super::t2::n8890(s, out),
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
									super::push(out, 4416);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 5846);
											match s {
												[] => true,
												['ψ', s @ ..] => {
													super::push(out, 6954);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8891(s, out),
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
									super::push(out, 4417);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5847);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 6955);
													match s {
														[] => true,
														['γ', s @ ..] => super::t2::n8892(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['η', s @ ..] => {
											super::push(out, 5848);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 6956);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t2::n8893(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 5849);
											match s {
												[] => true,
												['ω', s @ ..] => {
													super::push(out, 6957);
													match s {
														[] => true,
														['λ', s @ ..] => super::t2::n8894(s, out),
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
									super::push(out, 4418);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5850);
											match s {
												[] => true,
												['ο', s @ ..] => super::t2::n7855(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ω', s @ ..] => super::t2::n4702(s, out),
								_ => false,
							}
						}
						['σ', s @ ..] => super::t2::n2911(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κεω.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2810);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4421);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 5853);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 6960);
													match s {
														[] => true,
														['κ', s @ ..] => super::t2::n8897(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2811);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 4422);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5854);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 5855);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 6961);
													match s {
														[] => true,
														['λ', s @ ..] => super::t2::n8898(s, out),
														_ => false,
													}
												}
												['χ', s @ ..] => {
													super::push(out, 6962);
													match s {
														[] => true,
														['ι', s @ ..] => super::t2::n8899(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 568);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1435);
					match s {
						[] => true,
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
			super::push(out, 569);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4423);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 4424);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 5856);
											match s {
												[] => true,
												['ξ', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['σ', s @ ..] => {
													super::push(out, 6963);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8900(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2813);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4425);
									match s {
										[] => true,
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
							super::push(out, 2814);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4426);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 5857);
											match s {
												[] => true,
												['φ', s @ ..] => {
													super::push(out, 6964);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8901(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1437);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2815);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t2::n1455(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2822);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2823);
							match s {
								[] => true,
								['ε', s @ ..] => super::t2::n4717(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => super::t2::n1457(s, out),
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t2::n571(s, out),
		['σ', s @ ..] => super::t2::n572(s, out),
		_ => false,
	}
}

/// Words ending -λεω.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1444);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1445);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2828);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2829);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4437);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5864);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 6971);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 4438);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5865);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6972);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8909(s, out),
														['σ', s @ ..] => super::t2::n8910(s, out),
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
									super::push(out, 4439);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5866);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 6973);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8911(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 5867);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 6974);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8912(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2830);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4440);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 4441);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5868);
											match s {
												[] => true,
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
							super::push(out, 2831);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4442);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5869);
											match s {
												[] => true,
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
							super::push(out, 2832);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4443);
									match s {
										[] => true,
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
							super::push(out, 2833);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4444);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5870);
											match s {
												[] => true,
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
							super::push(out, 2834);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4445);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 5871);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 6975);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8913(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1446);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t2::n574(s, out),
		['η', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1451);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2844);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2845);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4454);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t2::n6445(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => super::t2::n1470(s, out),
				['π', s @ ..] => super::t2::n1471(s, out),
				['φ', s @ ..] => super::t2::n1472(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t2::n578(s, out),
		['ο', s @ ..] => super::t2::n579(s, out),
		['π', s @ ..] => super::t2::n580(s, out),
		['τ', s @ ..] => super::t2::n581(s, out),
		['υ', s @ ..] => super::t2::n582(s, out),
		['φ', s @ ..] => super::t2::n583(s, out),
		['χ', s @ ..] => super::t2::n584(s, out),
		['ω', s @ ..] => super::t2::n585(s, out),
		_ => false,
	}
}

/// Words ending -μεω.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1473);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2868);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4471);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5890);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 5891);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 6983);
													match s {
														[] => true,
														['α', s @ ..] => super::t2::n8920(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 5892);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 6984);
													match s {
														[] => true,
														['ν', s @ ..] => super::t2::n8921(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['μ', s @ ..] => {
											super::push(out, 5893);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 6985);
													match s {
														[] => true,
														['ξ', s @ ..] => super::t2::n8922(s, out),
														['σ', s @ ..] => super::t2::n8923(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1474);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2869);
							match s {
								[] => true,
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
			super::push(out, 588);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1475);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2870);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4472);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 5894);
											match s {
												[] => true,
												['β', s @ ..] => {
													super::push(out, 6986);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8924(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2871);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4473);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5895);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6987);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8925(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 589);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1476);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2872);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4474);
									match s {
										[] => true,
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
		['ο', s @ ..] => super::t2::n590(s, out),
		['ρ', s @ ..] => {
			super::push(out, 591);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1482);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2882);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 2883);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4484);
									match s {
										[] => true,
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
			super::push(out, 592);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1483);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2884);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4485);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 5905);
											match s {
												[] => true,
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
					super::push(out, 1484);
					match s {
						[] => true,
						['κ', s @ ..] => super::t2::n2994(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t2::n593(s, out),
		_ => false,
	}
}

/// Words ending -νεω.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 594);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 595);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1486);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2888);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4488);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 5909);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 6998);
													match s {
														[] => true,
														['α', s @ ..] => super::t2::n8940(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1487);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2889);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4489);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5910);
											match s {
												[] => true,
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
			super::push(out, 596);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2890);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4490);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5911);
											s.is_empty()
										}
										['ξ', s @ ..] => {
											super::push(out, 5912);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 6999);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8941(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2891);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4491);
									match s {
										[] => true,
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
					super::push(out, 1489);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2892);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4492);
									match s {
										[] => true,
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
			super::push(out, 597);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1490);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2893);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4493);
									match s {
										[] => true,
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
			super::push(out, 598);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1492);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2894);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2895);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4494);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5913);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2896);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4495);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5914);
											match s {
												[] => true,
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
					super::push(out, 1493);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1494);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2898);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4496);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5915);
											match s {
												[] => true,
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
					super::push(out, 1495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2899);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2900);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4497);
									match s {
										[] => true,
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
			super::push(out, 599);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1496);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2901);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 600);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 601);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1498);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2903);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4498);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 5916);
											match s {
												[] => true,
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
		['ο', s @ ..] => super::t2::n602(s, out),
		['π', s @ ..] => super::t2::n603(s, out),
		['υ', s @ ..] => super::t2::n604(s, out),
		['χ', s @ ..] => {
			super::push(out, 605);
			match s {
				[] => true,
				['ε', s @ ..] => super::t2::n1535(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t2::n606(s, out),
		_ => false,
	}
}

/// Words ending -ξεω.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 607);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οεω.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 608);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1522);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2930);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4522);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 5936);
											match s {
												[] => true,
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
									super::push(out, 4523);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 5937);
											match s {
												[] => true,
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
				['γ', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2931);
							match s {
								[] => true,
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
					super::push(out, 1524);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2932);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4524);
									match s {
										[] => true,
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
					super::push(out, 1525);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2933);
							match s {
								[] => true,
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
					super::push(out, 1526);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2934);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4525);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5938);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2935);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4526);
									match s {
										[] => true,
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
		['π', s @ ..] => {
			super::push(out, 610);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 612);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1528);
					match s {
						[] => true,
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

/// Words ending -πεω.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2936);
							match s {
								[] => true,
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
			super::push(out, 614);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1530);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4527);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 4528);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 5939);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 7015);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8955(s, out),
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
									super::push(out, 4529);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5940);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7016);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8956(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 5941);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7017);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8957(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1531);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2938);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4530);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5942);
											match s {
												[] => true,
												['θ', s @ ..] => {
													super::push(out, 7018);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8958(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2939);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4531);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 5943);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7019);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8959(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1532);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2940);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4532);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 5944);
											match s {
												[] => true,
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
		['υ', s @ ..] => {
			super::push(out, 615);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1535);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2943);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4534);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 4535);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 5945);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7020);
													match s {
														[] => true,
														['π', s @ ..] => super::t2::n8960(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 616);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1536);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2944);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4536);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5946);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7021);
													match s {
														[] => true,
														['ε', s @ ..] => super::t2::n8961(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρεω.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2945);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4537);
									match s {
										[] => true,
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
					super::push(out, 1538);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2946);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 4538);
									match s {
										[] => true,
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
		['γ', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2947);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 2948);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4539);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5947);
											s.is_empty()
										}
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
					super::push(out, 1540);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 2949);
							match s {
								[] => true,
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
		['ε', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1541);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2950);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4540);
									match s {
										[] => true,
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
					super::push(out, 1542);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2951);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4541);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5948);
											match s {
												[] => true,
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
							super::push(out, 2952);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4542);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 4543);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5949);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7022);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8962(s, out),
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
									super::push(out, 4544);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5950);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 620);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2953);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4545);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5951);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7023);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8963(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1544);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2954);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4546);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 5952);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7024);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n8964(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2955);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4547);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5953);
											match s {
												[] => true,
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
			super::push(out, 621);
			match s {
				[] => true,
				['α', s @ ..] => super::t2::n1566(s, out),
				['ε', s @ ..] => {
					super::push(out, 1547);
					match s {
						[] => true,
						['σ', s @ ..] => super::t2::n3075(s, out),
						['χ', s @ ..] => super::t2::n3076(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => super::t2::n622(s, out),
		['ο', s @ ..] => super::t3::n623(s, out),
		['ρ', s @ ..] => super::t3::n624(s, out),
		['τ', s @ ..] => super::t3::n625(s, out),
		['υ', s @ ..] => super::t3::n626(s, out),
		['ω', s @ ..] => super::t3::n627(s, out),
		_ => false,
	}
}

/// Words ending -σεω.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 628);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1568);
					match s {
						[] => true,
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
			super::push(out, 629);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1569);
					match s {
						[] => true,
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
			super::push(out, 630);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1570);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2994);
							match s {
								[] => true,
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
			super::push(out, 631);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1571);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2995);
							match s {
								[] => true,
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

/// Words ending -τεω.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 632);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1572);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2996);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4585);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 5988);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7047);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n8986(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1573);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2997);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 4586);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 5989);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7048);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n8987(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1574);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2998);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4587);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 5990);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7049);
													match s {
														[] => true,
														['λ', s @ ..] => super::t3::n8988(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 5991);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7050);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n8989(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 633);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1575);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4588);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3000);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4589);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 5992);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7051);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n8990(s, out),
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
									super::push(out, 4590);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 5993);
											match s {
												[] => true,
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
									super::push(out, 4591);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 5994);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7052);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n8991(s, out),
														_ => false,
													}
												}
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
							super::push(out, 3001);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4592);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 5995);
											match s {
												[] => true,
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
					super::push(out, 1576);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3002);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 4593);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 5996);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 4594);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 5997);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7053);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1577);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 3003);
							match s {
								[] => true,
								['π', s @ ..] => super::t3::n4899(s, out),
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 3004);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4596);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t3::n634(s, out),
		['ι', s @ ..] => super::t3::n635(s, out),
		['κ', s @ ..] => super::t3::n636(s, out),
		['ν', s @ ..] => super::t3::n637(s, out),
		['ο', s @ ..] => super::t3::n638(s, out),
		['ρ', s @ ..] => super::t3::n639(s, out),
		['σ', s @ ..] => super::t3::n640(s, out),
		['υ', s @ ..] => super::t3::n641(s, out),
		_ => false,
	}
}

/// Words ending -υεω.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1595);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3029);
							match s {
								[] => true,
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

/// Words ending -φεω.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1596);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3030);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4618);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6015);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7065);
													match s {
														[] => true,
														['α', s @ ..] => super::t3::n9002(s, out),
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
									super::push(out, 4619);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6016);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7066);
													match s {
														[] => true,
														['ε', s @ ..] => super::t3::n9003(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['λ', s @ ..] => {
											super::push(out, 6017);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7067);
													match s {
														[] => true,
														['γ', s @ ..] => super::t3::n9004(s, out),
														_ => false,
													}
												}
												['ι', 'ψ', '\u{1}', s @ ..] => {
													super::push(out, 7068);
													s.is_empty()
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 6018);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7069);
													match s {
														[] => true,
														['ο', s @ ..] => super::t3::n9005(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 6019);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7070);
													match s {
														[] => true,
														['ι', s @ ..] => super::t3::n9006(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 6020);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7071);
													match s {
														[] => true,
														['μ', s @ ..] => super::t3::n9007(s, out),
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
									super::push(out, 4620);
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 6021);
											match s {
												[] => true,
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
							super::push(out, 3031);
							match s {
								[] => true,
								['ο', s @ ..] => super::t3::n4929(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t3::n646(s, out),
		_ => false,
	}
}

/// Words ending -χεω.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 647);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 648);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3035);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1601);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3036);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4625);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 6026);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7076);
													s.is_empty()
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 6027);
											match s {
												[] => true,
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
							super::push(out, 3037);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4626);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6028);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7077);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9012(s, out),
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
									super::push(out, 4627);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 6029);
											match s {
												[] => true,
												['φ', s @ ..] => {
													super::push(out, 7078);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9013(s, out),
														_ => false,
													}
												}
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
							super::push(out, 3038);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4628);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6030);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7079);
													s.is_empty()
												}
												['ν', s @ ..] => {
													super::push(out, 7080);
													match s {
														[] => true,
														['υ', s @ ..] => super::t3::n9014(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1602);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3039);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4629);
									match s {
										[] => true,
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
					super::push(out, 1603);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3040);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4630);
									match s {
										[] => true,
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
		['γ', s @ ..] => {
			super::push(out, 649);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 650);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1605);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1606);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3042);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4631);
									match s {
										[] => true,
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
			super::push(out, 651);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1607);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3043);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4632);
									s.is_empty()
								}
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
			super::push(out, 652);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1608);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['α', s @ ..] => super::t3::n1632(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t3::n655(s, out),
		_ => false,
	}
}

/// Words ending -ψεω.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 656);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1614);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3054);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4642);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 6038);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7086);
													match s {
														[] => true,
														['σ', s @ ..] => super::t3::n9020(s, out),
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
									super::push(out, 4643);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 6039);
											match s {
												[] => true,
												['ω', s @ ..] => {
													super::push(out, 7087);
													match s {
														[] => true,
														['λ', s @ ..] => super::t3::n9021(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωεω.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥεω.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 658);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αζω.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 190);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 659);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 660);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1616);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['σ', s @ ..] => {
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
				['ι', s @ ..] => {
					super::push(out, 1618);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 3056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 4644);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6040);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 7088);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9022(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 6041);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7089);
													match s {
														[] => true,
														['κ', s @ ..] => super::t3::n9023(s, out),
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
									super::push(out, 4645);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6042);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7090);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9024(s, out),
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
									super::push(out, 4646);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6043);
											match s {
												[] => true,
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
									super::push(out, 4647);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6044);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7091);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 6045);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7092);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9025(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 661);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1619);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3057);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 4648);
									match s {
										[] => true,
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
					super::push(out, 1620);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3058);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 4649);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6046);
											match s {
												[] => true,
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
		['δ', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1621);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 3059);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4650);
									match s {
										[] => true,
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
					super::push(out, 1622);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3060);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4651);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6047);
											match s {
												[] => true,
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
			super::push(out, 663);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 664);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1624);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3062);
							match s {
								[] => true,
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
		['ι', s @ ..] => super::t3::n665(s, out),
		['κ', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['γ', s @ ..] => super::t3::n1659(s, out),
				['ι', s @ ..] => super::t3::n1660(s, out),
				['ρ', s @ ..] => super::t3::n1661(s, out),
				['σ', s @ ..] => {
					super::push(out, 1640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t3::n667(s, out),
		['μ', s @ ..] => super::t3::n668(s, out),
		['ν', s @ ..] => super::t3::n669(s, out),
		['ξ', s @ ..] => super::t3::n670(s, out),
		['ο', s @ ..] => super::t3::n671(s, out),
		['π', s @ ..] => super::t3::n672(s, out),
		['ρ', s @ ..] => super::t3::n673(s, out),
		['τ', s @ ..] => super::t3::n674(s, out),
		['υ', s @ ..] => super::t3::n675(s, out),
		['φ', s @ ..] => super::t3::n676(s, out),
		['χ', s @ ..] => super::t3::n677(s, out),
		['ω', s @ ..] => super::t3::n678(s, out),
		_ => false,
	}
}

/// Words ending -καταρρεζω.
pub(crate) fn n8086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7116);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιζω.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 684);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3142);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 3143);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 4717);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6085);
											match s {
												[] => true,
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
					super::push(out, 1689);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 3144);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 4718);
									match s {
										[] => true,
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
					super::push(out, 1690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3145);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 3146);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4719);
									match s {
										[] => true,
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
					super::push(out, 1691);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3147);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 4720);
									match s {
										[] => true,
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
					super::push(out, 1692);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3148);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4721);
									match s {
										[] => true,
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
			super::push(out, 686);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3149);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4722);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 6086);
											match s {
												[] => true,
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
					super::push(out, 1694);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3150);
							match s {
								[] => true,
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
					super::push(out, 1695);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3151);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4723);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6087);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7117);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9046(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 687);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1697);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3153);
							match s {
								[] => true,
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
					super::push(out, 1698);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3154);
							match s {
								[] => true,
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
			super::push(out, 688);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1699);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 3155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 3156);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 4725);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6088);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 7118);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9047(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1700);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3157);
							match s {
								[] => true,
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
					super::push(out, 1701);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3158);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4726);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6089);
											match s {
												[] => true,
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
				['ο', s @ ..] => super::t3::n1725(s, out),
				_ => false,
			}
		}
		['θ', s @ ..] => super::t3::n689(s, out),
		['κ', s @ ..] => super::t3::n690(s, out),
		['λ', s @ ..] => super::t3::n691(s, out),
		['μ', s @ ..] => super::t3::n692(s, out),
		['ν', s @ ..] => super::t3::n693(s, out),
		['ο', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1733);
					match s {
						[] => true,
						['θ', s @ ..] => super::t3::n3336(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t3::n695(s, out),
		['ρ', s @ ..] => super::t3::n696(s, out),
		['σ', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['\u{1}', s @ ..] => super::t3::n1773(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => super::t3::n698(s, out),
		['φ', s @ ..] => super::t3::n699(s, out),
		['χ', s @ ..] => super::t3::n700(s, out),
		['ω', s @ ..] => super::t3::n701(s, out),
		_ => false,
	}
}

/// Words ending -κατακλυζω.
pub(crate) fn n8179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7175);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word κελαρυζω.
pub(crate) fn n8180(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -κρωζω.
pub(crate) fn n1803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1780);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σωζω.
pub(crate) fn n712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 712);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1781);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3274);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4834);
							match s {
								[] => true,
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
			super::push(out, 1782);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3275);
					match s {
						[] => true,
						['ξ', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4835);
							match s {
								[] => true,
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

/// Words ending -ποστερηθω.
pub(crate) fn n8181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7176);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 7757);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αναπειθω.
pub(crate) fn n8182(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word διαπερθω.
pub(crate) fn n8183(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -γεγενησθω.
pub(crate) fn n8184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7177);
	s.is_empty()
}

/// Words ending -ευθω.
pub(crate) fn n724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 724);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3284);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3285);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4843);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6187);
									match s {
										[] => true,
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
		['λ', s @ ..] => {
			super::push(out, 1798);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3286);
					match s {
						[] => true,
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
			super::push(out, 1799);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3287);
					match s {
						[] => true,
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

/// The word μινυθω.
pub(crate) fn n5193(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -πυθω.
pub(crate) fn n726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 726);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χθω.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 727);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1802);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3289);
							match s {
								[] => true,
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

/// Words ending -ωθω.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 728);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3290);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4844);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6188);
											match s {
												[] => true,
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

/// The word παραπαιω.
pub(crate) fn n8187(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word διαρραιω.
pub(crate) fn n8188(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ρικλειω.
pub(crate) fn n5202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4850);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 6193);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 7178);
					match s {
						[] => true,
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

/// Words ending -οκλειω.
pub(crate) fn n3437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3302);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4851);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6194);
					match s {
						[] => true,
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

/// Words ending -νειω.
pub(crate) fn n743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 743);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1819);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξειω.
pub(crate) fn n744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 744);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1820);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3303);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4852);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6195);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7179);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7758);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρειω.
pub(crate) fn n745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 745);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 1821);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3304);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6196);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σειω.
pub(crate) fn n746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 746);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1822);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1823);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 3305);
					match s {
						[] => true,
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
			super::push(out, 1824);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3306);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4854);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6197);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7180);
											match s {
												[] => true,
												['π', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θιω.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1825);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 748);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κιω.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 749);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λιω.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 750);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1827);
					match s {
						[] => true,
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

/// Words ending -νιω.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1828);
					match s {
						[] => true,
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
			super::push(out, 752);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3307);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ριω.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 215);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1830);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3308);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4855);
									match s {
										[] => true,
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
			super::push(out, 754);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1831);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1832);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3309);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4856);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 6198);
											match s {
												[] => true,
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
				['γ', s @ ..] => {
					super::push(out, 1833);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3310);
							match s {
								[] => true,
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
					super::push(out, 1834);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3311);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4857);
									match s {
										[] => true,
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
							super::push(out, 3312);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4858);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6199);
											match s {
												[] => true,
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
					super::push(out, 1835);
					match s {
						[] => true,
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

/// Words ending -τιω.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 755);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -περιπλεκω.
pub(crate) fn n8195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7181);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σκω.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1853);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1854);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3325);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4866);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 6201);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7182);
													match s {
														[] => true,
														['α', s @ ..] => super::t3::n9106(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1855);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3326);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 4867);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6202);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 7183);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9107(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1857);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3327);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4868);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6203);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['ο', s @ ..] => {
													super::push(out, 7184);
													match s {
														[] => true,
														['π', s @ ..] => super::t3::n9108(s, out),
														_ => false,
													}
												}
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
							super::push(out, 3328);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 4869);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 6204);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7185);
													match s {
														[] => true,
														['α', s @ ..] => super::t3::n9109(s, out),
														_ => false,
													}
												}
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
							super::push(out, 3329);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4870);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6205);
											match s {
												[] => true,
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
					super::push(out, 1858);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 771);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1860);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3331);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 4871);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6206);
											match s {
												[] => true,
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
		['η', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1861);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3332);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4872);
									match s {
										[] => true,
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
				['ν', s @ ..] => super::t3::n1891(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t3::n774(s, out),
		['ο', s @ ..] => {
			super::push(out, 774);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1868);
					match s {
						[] => true,
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
			super::push(out, 775);
			match s {
				[] => true,
				['θ', s @ ..] => super::t3::n1898(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => super::t3::n777(s, out),
		_ => false,
	}
}

/// Words ending -ρυκω.
pub(crate) fn n778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 777);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1873);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1874);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωκω.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3350);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 4888);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6222);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7195);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9121(s, out),
														_ => false,
													}
												}
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
							super::push(out, 3351);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4889);
									match s {
										[] => true,
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

/// Words ending -προοφειλω.
pub(crate) fn n8220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7196);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προυφειλω.
pub(crate) fn n8221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7197);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λλω.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1879);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3354);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 3355);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4893);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6225);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7198);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 4894);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6226);
											match s {
												[] => true,
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
									super::push(out, 4895);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6227);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7199);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9124(s, out),
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
									super::push(out, 4896);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6228);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7200);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9125(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 6229);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7201);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9126(s, out),
														_ => false,
													}
												}
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
							super::push(out, 3356);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 4897);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6230);
											match s {
												[] => true,
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
									super::push(out, 4898);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6231);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7202);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9127(s, out),
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
									super::push(out, 4899);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 6232);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7203);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9128(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => super::t3::n3508(s, out),
						['μ', s @ ..] => super::t3::n3509(s, out),
						['ο', s @ ..] => super::t3::n3510(s, out),
						['ρ', s @ ..] => super::t3::n3511(s, out),
						['σ', s @ ..] => super::t3::n3512(s, out),
						_ => false,
					}
				}
				['δ', s @ ..] => super::t3::n1912(s, out),
				['θ', s @ ..] => super::t3::n1913(s, out),
				['ι', s @ ..] => {
					super::push(out, 1882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t3::n3517(s, out),
						_ => false,
					}
				}
				['κ', s @ ..] => super::t3::n1915(s, out),
				['π', s @ ..] => super::t3::n1916(s, out),
				['τ', s @ ..] => super::t3::n1917(s, out),
				['φ', s @ ..] => super::t3::n1918(s, out),
				['ψ', s @ ..] => super::t3::n1919(s, out),
				_ => false,
			}
		}
		['ε', s @ ..] => super::t3::n785(s, out),
		['ι', s @ ..] => super::t3::n786(s, out),
		['υ', s @ ..] => super::t3::n787(s, out),
		_ => false,
	}
}

/// Words ending -μβανω.
pub(crate) fn n1934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1901);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3386);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4922);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6255);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 6256);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 7210);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7770);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 8047);
													s.is_empty()
												}
												['π', s @ ..] => {
													super::push(out, 8048);
													match s {
														[] => true,
														['ε', s @ ..] => super::t3::n9901(s, out),
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
									super::push(out, 7211);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7771);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 8049);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9902(s, out),
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
									super::push(out, 7212);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7772);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 8050);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9903(s, out),
														['ο', s @ ..] => super::t3::n9904(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 7773);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 8051);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9905(s, out),
														_ => false,
													}
												}
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
							super::push(out, 6257);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7213);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7774);
											match s {
												[] => true,
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
							super::push(out, 6258);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 7214);
									match s {
										[] => true,
										['ξ', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 7775);
											match s {
												[] => true,
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
							super::push(out, 6259);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 7215);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7776);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 7777);
											match s {
												[] => true,
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
									super::push(out, 7216);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7778);
											match s {
												[] => true,
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
							super::push(out, 6260);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 7217);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7779);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 8052);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9906(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νδανω.
pub(crate) fn n1937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1904);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3389);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4924);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4925);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6261);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 4926);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζανω.
pub(crate) fn n795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 794);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1905);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3390);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4927);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6263);
									match s {
										[] => true,
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

/// Words ending -θανω.
pub(crate) fn n796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 795);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3391);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4928);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 4929);
							match s {
								[] => true,
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
			super::push(out, 1907);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3392);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4930);
							match s {
								[] => true,
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
			super::push(out, 1908);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3393);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4931);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6264);
									match s {
										[] => true,
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
			super::push(out, 1909);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κανω.
pub(crate) fn n797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 796);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1910);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξανω.
pub(crate) fn n798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 797);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3394);
					match s {
						[] => true,
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

/// Words ending -πανω.
pub(crate) fn n799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 798);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1912);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3395);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4932);
							match s {
								[] => true,
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

/// Words ending -τανω.
pub(crate) fn n800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 799);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1913);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3396);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4933);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6265);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7218);
											s.is_empty()
										}
										['φ', s @ ..] => {
											super::push(out, 7219);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7780);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9653(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1914);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3397);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4934);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6266);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 7220);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7781);
													match s {
														[] => true,
														['α', s @ ..] => super::t3::n9654(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3398);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4935);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6267);
									match s {
										[] => true,
										['π', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φανω.
pub(crate) fn n801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 800);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1915);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3399);
					match s {
						[] => true,
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

/// Words ending -χανω.
pub(crate) fn n802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 801);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3400);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 6268);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7221);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7782);
													match s {
														[] => true,
														['μ', s @ ..] => super::t3::n9655(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3401);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6269);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 6270);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7222);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7783);
													match s {
														[] => true,
														['π', s @ ..] => super::t3::n9656(s, out),
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
									super::push(out, 6271);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7223);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7784);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9657(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 7224);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7785);
													match s {
														[] => true,
														['π', s @ ..] => super::t3::n9658(s, out),
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
									super::push(out, 6272);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7225);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 7226);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7786);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9659(s, out),
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
									super::push(out, 6273);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7227);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7787);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t3::n9660(s, out),
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
									super::push(out, 6274);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7228);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7788);
													match s {
														[] => true,
														['π', s @ ..] => super::t3::n9661(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ενω.
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1917);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1918);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3402);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 4938);
									match s {
										[] => true,
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
							super::push(out, 3403);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4939);
									match s {
										[] => true,
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
							super::push(out, 3404);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 4940);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6275);
											match s {
												[] => true,
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
					super::push(out, 1919);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3405);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4941);
									match s {
										[] => true,
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
							super::push(out, 3406);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 4942);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6276);
											match s {
												[] => true,
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
					super::push(out, 1920);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3407);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3408);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 4943);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6277);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 4944);
									match s {
										[] => true,
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
					super::push(out, 1921);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3409);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 4945);
									match s {
										[] => true,
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
			super::push(out, 803);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1922);
					match s {
						[] => true,
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

